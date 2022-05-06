use sqlx::PgPool;

pub trait HaveDBConnection {
    fn db_connection(&self) -> &PgPool;
}
