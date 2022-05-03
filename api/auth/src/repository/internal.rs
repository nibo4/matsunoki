use crate::model::meta::{AggregateRoot, Identifier};
use async_trait::async_trait;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResolveError {
    #[error("id: {id} entity is not exist")]
    NotExist { id: String },
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[async_trait]
pub trait Repository<I: Identifier, T: AggregateRoot<I>> {
    async fn resolve(&self, id: I) -> Result<T, ResolveError>;
}
