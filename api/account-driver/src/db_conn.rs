use crate::config::DefaultConfig;
use account::effect::config::Config;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub trait HaveDBConnection {
    fn db_connection(&self) -> &PgPool;
}

pub async fn build_conn(config: &DefaultConfig) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(*config.max_connections())
        .connect("postgres://development:development@localhost:5433/matsunoki-account")
        .await
        .unwrap();
    pool
}

#[cfg(test)]
pub struct TestDBConnection {
    pub conn: PgPool,
}

#[cfg(test)]
impl HaveDBConnection for TestDBConnection {
    fn db_connection(&self) -> &PgPool {
        &self.conn
    }
}

#[cfg(test)]
#[derive(sqlx::FromRow, Debug)]
pub struct Table(pub String);

#[cfg(test)]
#[async_trait::async_trait]
pub trait TestDBInterface {
    async fn flush(&self);
    async fn default() -> Self;
}

#[cfg(test)]
#[async_trait::async_trait]
impl TestDBInterface for TestDBConnection {
    async fn flush(&self) {
        let tables = sqlx::query_as::<_, Table>("SELECT table_name FROM information_schema.tables WHERE table_type = 'BASE TABLE' AND table_schema NOT IN ('pg_catalog', 'information_schema');").fetch_all(self.db_connection()).await.unwrap();
        for table in tables {
            sqlx::query(format!("DELETE FROM {};", &table.0).as_str())
                .execute(self.db_connection())
                .await
                .unwrap();
        }
    }
    async fn default() -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://development:development@localhost:5433/matsunoki-account")
            .await
            .unwrap();
        let conn = TestDBConnection { conn: pool };
        conn.flush().await;
        conn
    }
}
