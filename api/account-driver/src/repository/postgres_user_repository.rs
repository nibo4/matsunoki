use account::model::login_provider::{IdInProvider, LoginProvider, ProviderKind};
use account::model::user::{User, UserId};
use account::repository::meta::{Repository, ResolveError};
use account::repository::user_repository::{FilterByIdInProviderError, StoreError, UserRepository};
use anyhow::Context;
use async_trait::async_trait;
use derive_more::Constructor;
use indoc::indoc;
use sqlx::{query, query_as, PgPool};
use tracing::info;

use crate::db_conn::HaveDBConnection;

#[derive(Constructor, Debug, Clone)]
pub struct PostgresUserRepository {
    conn: PgPool,
}

impl HaveDBConnection for PostgresUserRepository {
    fn db_connection(&self) -> &PgPool {
        &self.conn
    }
}

#[derive(sqlx::FromRow)]
struct UserRow {
    pub id: String,
}

#[derive(sqlx::FromRow, Clone)]
struct LoginProviderRow {
    pub kind: String,
    pub id_in_provider: String,
    pub user_id: String,
}

impl TryFrom<LoginProviderRow> for LoginProvider {
    type Error = anyhow::Error;
    fn try_from(x: LoginProviderRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            ProviderKind::try_from(x.kind).context("ProviderKind")?,
            IdInProvider::new(x.id_in_provider),
        ))
    }
}

impl TryFrom<&LoginProviderRow> for LoginProvider {
    type Error = anyhow::Error;
    fn try_from(x: &LoginProviderRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            ProviderKind::try_from(x.kind.clone()).context("ProviderKind")?,
            IdInProvider::new(x.id_in_provider.clone()),
        ))
    }
}

#[async_trait]
impl Repository<UserId, User> for PostgresUserRepository {
    async fn resolve(&self, id: &UserId) -> Result<Option<User>, ResolveError> {
        let user_row = match query_as::<_, UserRow>("SELECT * FROM users where id=$1;")
            .bind(&id.0)
            .fetch_optional(self.db_connection())
            .await
            .context("Failed execute query")?
        {
            Some(row) => row,
            None => {
                return Ok(None);
            }
        };
        let provider_rows =
            query_as::<_, LoginProviderRow>("SELECT * FROM login_providers where user_id=$1;")
                .bind(&id.0)
                .fetch_all(self.db_connection())
                .await
                .context("Failed execute query")?;
        let providers = provider_rows
            .iter()
            .map(|row| LoginProvider::try_from(row.clone()))
            .collect::<Result<Vec<LoginProvider>, anyhow::Error>>()?;

        Ok(Some(User::new(UserId::new(user_row.id), Some(providers))))
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id_in_provider(
        &self,
        id_in_provider: &IdInProvider,
    ) -> Result<Option<User>, FilterByIdInProviderError> {
        let provider_rows = query_as::<_, LoginProviderRow>(
            "SELECT * FROM login_providers WHERE id_in_provider=$1",
        )
        .bind(&id_in_provider.0)
        .fetch_all(self.db_connection())
        .await
        .context("Failed execute query")?;
        let first_provider_row = match provider_rows.get(0) {
            Some(r) => r,
            None => return Ok(None),
        };
        let user = match query_as::<_, UserRow>("SELECT * FROM users WHERE id=$1")
            .bind(&first_provider_row.user_id)
            .fetch_optional(self.db_connection())
            .await
            .context("Failed find user query")?
        {
            Some(u) => u,
            None => return Ok(None),
        };
        let providers = provider_rows
            .iter()
            .map(|row| LoginProvider::try_from(row))
            .collect::<Result<Vec<LoginProvider>, anyhow::Error>>()?;
        Ok(Some(User::new(UserId::new(user.id), Some(providers))))
    }

    #[tracing::instrument(skip(self))]
    async fn store(&self, u: &User) -> Result<(), StoreError> {
        info!("Start transaction");
        let mut transaction = self
            .db_connection()
            .begin()
            .await
            .context("failed get context")?;
        for login_provider in u.providers.iter() {
            query(indoc! {"
                INSERT INTO login_providers (user_id, kind, id_in_provider, updated_at) VALUES ($1, $2, $3, NOW())
                ON CONFLICT ON CONSTRAINT login_providers_pkey
                DO UPDATE SET user_id=$1, kind=$2, id_in_provider=$3, updated_at=NOW();
            "})
                .bind(&u.id.0)
                .bind(String::from(&login_provider.kind))
                .bind(&login_provider.id_in_provider.0)
                .execute(&mut transaction).await.context("failed login_provider store")?;
        }
        query(indoc! {"
            INSERT INTO users (id, updated_at) VALUES ($1, NOW())
            ON CONFLICT ON CONSTRAINT users_pkey
            DO UPDATE SET id=$1, updated_at=NOW();
        "})
        .bind(u.id.0.clone())
        .execute(&mut transaction)
        .await
        .context("failed user store")?;
        transaction
            .commit()
            .await
            .context("failed commit postgres_user_repository store")?;
        info!("Commit transaction");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::PostgresUserRepository;
    use crate::db_conn::{TestDBConnection, TestDBInterface};
    use account::model::login_provider::{IdInProvider, LoginProvider, ProviderKind};
    use account::model::user::{User, UserId};

    use account::repository::meta::Repository;
    use account::repository::user_repository::UserRepository;
    #[tokio::test]
    #[ignore]
    async fn postgres_user_repository_resolve_return_to_user() {
        let db_conn = TestDBConnection::default().await;
        let repo = PostgresUserRepository::new(db_conn.conn.clone());
        sqlx::query("INSERT INTO users (id, updated_at) VALUES ($1, NOW());")
            .bind("foo")
            .execute(&repo.conn)
            .await
            .unwrap();
        let expected_user = User::new(UserId("foo".to_string()), None);
        let user = repo
            .resolve(&UserId::new("foo".to_string()))
            .await
            .unwrap()
            .unwrap();

        db_conn.flush().await;

        assert_eq!(user, expected_user);
    }

    #[tokio::test]
    #[ignore]
    async fn postgres_user_repository_resolve_return_to_resolve_err() {
        let db_conn = TestDBConnection::default().await;
        let repo = PostgresUserRepository::new(db_conn.conn);
        let result = repo.resolve(&UserId::new("foo".to_string())).await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn postgres_user_repository_find_by_id_in_filter_return_to_user() {
        let db_conn = TestDBConnection::default().await;
        let repo = PostgresUserRepository::new(db_conn.conn.clone());
        let login_providers = vec![LoginProvider::new(
            ProviderKind::Google,
            IdInProvider::new("test1".to_string()),
        )];
        let user = User::new(UserId::new("dummy1".to_string()), Some(login_providers));
        repo.store(&user).await.unwrap();
        let find_result = repo
            .find_by_id_in_provider(&IdInProvider::new("test1".to_string()))
            .await;
        db_conn.flush().await;

        assert_eq!(find_result.unwrap(), Some(user));
    }

    #[tokio::test]
    #[ignore]
    async fn postgres_user_repository_find_by_id_in_filter_return_to_none() {
        let db_conn = TestDBConnection::default().await;
        let repo = PostgresUserRepository::new(db_conn.conn.clone());
        let login_providers = vec![LoginProvider::new(
            ProviderKind::Google,
            IdInProvider::new("test1".to_string()),
        )];
        let user = User::new(UserId::new("dummy1".to_string()), Some(login_providers));
        repo.store(&user).await.unwrap();
        let find_result = repo
            .find_by_id_in_provider(&IdInProvider::new("test2".to_string()))
            .await;
        db_conn.flush().await;

        assert_eq!(find_result.unwrap(), None);
    }
}
