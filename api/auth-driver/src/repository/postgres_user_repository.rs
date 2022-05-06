use anyhow::Context;
use async_trait::async_trait;
use auth::model::login_provider::{IdInProvider, LoginProvider, ProviderKind};
use auth::model::user::{User, UserId};
use auth::repository::meta::{Repository, ResolveError};
use auth::repository::user_repository::{FilterByIdInProviderError, StoreError, UserRepository};
use derive_more::Constructor;
use indoc::indoc;
use sqlx::{query, query_as, PgPool, Transaction};

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

#[async_trait]
impl<'a> UserRepository for PostgresUserRepository<'a> {
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

    async fn store(&self, u: &User) -> Result<(), StoreError> {
        let mut transaction = self
            .db_connection()
            .begin()
            .await
            .context("failed get context")?;
        for login_provider in u.providers.iter() {
            query(indoc! {"
                INSERT INTO login_providers (user_id, kind, id_in_provider, updated_at) VALUES ($1, $2, $3, NOW())
                ON CONFRICT ON CONSTRAINT login_providers_pkey
                DO UPDATE SET user_id=$1, kind=$2, id_in_provider=$3, updated_at=NOW();
            "})
                .bind(&u.id.0)
                .bind(String::from(&login_provider.kind))
                .bind(&login_provider.id_in_provider.0)
                .execute(&mut transaction).await.context("failed login_provider store")?;
        }
        query(indoc! {"
            INSERT INTO users (user_id, updated_at) VALUES ($1, NOW())
            ON CONFRICT ON CONSTRAINT login_providers_pkey
            DO UPDATE SET user_id=$1, updated_at=NOW();
        "})
        .bind(u.id.0.clone())
        .execute(&mut transaction)
        .await
        .context("failed user store")?;
        transaction
            .commit()
            .await
            .context("failed commit postgres_user_repository store")?;
        Ok(())
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
        sqlx::query("INSERT INTO users (id, updated_at) VALUES ($1, NOW());")
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

        db_conn.flush().await;

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