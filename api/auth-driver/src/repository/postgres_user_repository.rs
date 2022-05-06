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
struct PostgresUserRepository {
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
impl Repository<UserId, User> for PostgresUserRepository {
    async fn resolve(&self, id: &UserId) -> Result<Option<User>, ResolveError> {
        let user_row = match query_as::<_, UserRow>("SELECT * FROM users where id=?;")
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
            query_as::<_, LoginProviderRow>("SELECT * FROM login_provider where user_id=?;")
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
mod tests {}
