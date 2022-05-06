use anyhow::Context;
use async_trait::async_trait;
use auth::model::login_provider::{IdInProvider, LoginProvider, ProviderKind};
use auth::model::user::{User, UserId};
use auth::repository::meta::{Repository, ResolveError};
use auth::repository::user_repository::UserRepository;
use derive_more::Constructor;
use sqlx::{query_as, PgPool};

use crate::db_conn::HaveDBConnection;

#[derive(Constructor, Debug)]
struct PostgresUserRepository<'a> {
    conn: &'a PgPool,
}

impl<'a> HaveDBConnection for PostgresUserRepository<'a> {
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
#[async_trait]
impl<'a> Repository<UserId, User> for PostgresUserRepository<'a> {
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

#[cfg(test)]
mod tests {
    use super::PostgresUserRepository;
    use crate::db_conn::{TestDBConnection, TestDBInterface};
    use auth::model::user::{User, UserId};
    use auth::repository::meta::Repository;
    #[tokio::test]
    #[ignore]
    async fn postgres_user_repository_resolve_return_to_user() {
        let db_conn = TestDBConnection::default().await;
        let repo = PostgresUserRepository::new(&db_conn.conn);
        sqlx::query("INSERT INTO users (id) VALUES ($1);")
            .bind("foo")
            .execute(&db_conn.conn)
            .await
            .unwrap();
        let expected_user = User::new(UserId("foo".to_string()), None);
        let user = repo
            .resolve(&UserId::new("foo".to_string()))
            .await
            .unwrap()
            .unwrap();

        db_conn.clean().await;

        assert_eq!(user, expected_user);
    }

    #[tokio::test]
    #[ignore]
    async fn postgres_user_repository_resolve_return_to_resolve_err() {
        let db_conn = TestDBConnection::default().await;
        let repo = PostgresUserRepository::new(&db_conn.conn);
        let result = repo.resolve(&UserId::new("foo".to_string())).await.unwrap();

        assert!(result.is_none());
    }
}
