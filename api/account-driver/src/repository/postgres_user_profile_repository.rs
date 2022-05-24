use crate::db_conn::HaveDBConnection;
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use derive_more::Constructor;
use indoc::indoc;
use semval::prelude::*;
use sqlx::{query, query_as, PgPool};

use account::model::profile::avatar::Avatar;
use account::model::profile::display_name::DisplayName;
use account::model::profile::entity::{Profile, ProfileInvalidity};
use account::model::profile::user_name::UserName;
use account::model::user_profile::{UserProfile, UserProfileId};
use account::repository::meta::{Repository, ResolveError};
use account::repository::user_profile_repository::{StoreError, UserProfileRepository};

#[derive(Constructor, Debug, Clone)]
pub struct PostgresUserProfileRepository {
    conn: PgPool,
}

impl HaveDBConnection for PostgresUserProfileRepository {
    fn db_connection(&self) -> &PgPool {
        &self.conn
    }
}

#[derive(sqlx::FromRow)]
struct UserRow {
    pub id: String,
}

#[derive(sqlx::FromRow)]
struct ProfileRow {
    name: String,
    display_name: String,
    avatar_url: String,
}

impl TryFrom<ProfileRow> for Profile {
    type Error = ValidationContext<ProfileInvalidity>;
    fn try_from(value: ProfileRow) -> Result<Self, Self::Error> {
        let profile = Profile {
            name: UserName(value.name),
            display_name: DisplayName(value.display_name),
            avatar: Avatar {
                url: value.avatar_url,
            },
        };
        profile.validate()?;
        Ok(profile)
    }
}

#[async_trait]
impl Repository<UserProfileId, UserProfile> for PostgresUserProfileRepository {
    async fn resolve(&self, id: &UserProfileId) -> Result<Option<UserProfile>, ResolveError> {
        let user_id = match query_as::<_, UserRow>("SELECT * FROM users where id=$1;")
            .bind(&id.0)
            .fetch_optional(self.db_connection())
            .await
            .context("Failed execute query")?
        {
            Some(row) => row.id,
            None => {
                return Ok(None);
            }
        };
        let profile = query_as::<_, ProfileRow>("SELECT * FROM profiles where user_id=$1;")
            .bind(&id.0)
            .fetch_one(self.db_connection())
            .await
            .context("Failed execute query")?;

        let profile = Profile::try_from(profile)
            .map_err(|_| anyhow!("Cannot convert profile_row to model. id: {}", user_id))?;
        Ok(Some(UserProfile::new(UserProfileId::new(user_id), profile)))
    }
}

#[async_trait]
impl UserProfileRepository for PostgresUserProfileRepository {
    async fn store(&self, up: &UserProfile) -> Result<(), StoreError> {
        query(indoc! {"
            INSERT INTO profiles (user_id, name, display_name, avatar_url, updated_at) VALUES ($1, $2, $3, $4, NOW())
            ON CONFLICT ON CONSTRAINT profiles_pkey
            DO UPDATE SET user_id=$1, name=$2, display_name=$3, avatar_url=$4, updated_at=NOW();
        "})
            .bind(&up.id.0)
            .bind(&up.profile.name.0)
            .bind(&up.profile.display_name.0)
            .bind(&up.profile.avatar.url)
            .execute(self.db_connection())
            .await
            .context("Failed insert profile")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::PostgresUserProfileRepository;
    use crate::db_conn::{TestDBConnection, TestDBInterface};
    use account::model::profile::avatar::Avatar;
    use account::model::profile::display_name::DisplayName;
    use account::model::profile::entity::Profile;
    use account::model::profile::user_name::UserName;
    use account::model::user_profile::UserProfile;
    use account::model::user_profile::UserProfileId;
    use account::repository::meta::Repository;
    use account::repository::user_profile_repository::UserProfileRepository;

    #[tokio::test]
    #[ignore]
    async fn test_postgres_user_profile_repository_resolve_is_resolved_user_profile() {
        let db_conn = TestDBConnection::default().await;
        let repo = PostgresUserProfileRepository::new(db_conn.conn.clone());
        sqlx::query("INSERT INTO users (id, updated_at) VALUES ($1, NOW());")
            .bind("foo")
            .execute(&repo.conn)
            .await
            .unwrap();
        sqlx::query("INSERT INTO profiles (user_id, name, display_name, avatar_url, updated_at) VALUES ($1, $2, $3, $4, NOW());")
            .bind("foo")
            .bind("foo")
            .bind("fooo")
            .bind("https://example.com")
            .execute(&repo.conn)
            .await
            .unwrap();
        let expected_user_profile = UserProfile::new(
            UserProfileId::new("foo".to_string()),
            Profile::new(
                UserName("foo".to_string()),
                DisplayName::new("fooo".to_string()),
                Avatar::new("https://example.com".to_string()),
            ),
        );
        let user_profile = repo
            .resolve(&UserProfileId::new("foo".to_string()))
            .await
            .unwrap()
            .unwrap();

        db_conn.flush().await;

        assert_eq!(user_profile, expected_user_profile);
    }

    #[tokio::test]
    #[ignore]
    async fn test_postgres_user_profile_repository_resolve_is_not_resolved_user_profile_when_not_exist(
    ) {
        let db_conn = TestDBConnection::default().await;
        let repo = PostgresUserProfileRepository::new(db_conn.conn.clone());
        let user_profile = repo
            .resolve(&UserProfileId::new("foo".to_string()))
            .await
            .unwrap();

        db_conn.flush().await;

        assert!(user_profile.is_none());
    }

    #[tokio::test]
    #[ignore]
    #[cfg(test)]
    async fn test_postgres_user_profile_repository_store_is_stored_user_profile() {
        let db_conn = TestDBConnection::default().await;
        let repo = PostgresUserProfileRepository::new(db_conn.conn.clone());
        let expected_user_profile = UserProfile::new(
            UserProfileId::new("fooo".to_string()),
            Profile::new(
                UserName("fooo".to_string()),
                DisplayName("fooo".to_string()),
                Avatar::new("https://example.com".to_string()),
            ),
        );
        sqlx::query("INSERT INTO users (id, updated_at) VALUES ($1, NOW());")
            .bind(&expected_user_profile.id.to_string())
            .execute(&repo.conn)
            .await
            .unwrap();
        let result = repo.store(&expected_user_profile).await;

        let user = repo
            .resolve(&expected_user_profile.id)
            .await
            .unwrap()
            .unwrap();
        assert!(result.is_ok());
        assert_eq!(user, expected_user_profile);
        db_conn.flush().await;
    }
}
