use crate::repository::internal::Repository;
use crate::model::user::{UserId, User};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("id: {id} entity is not exist")]
    AlreadyExist {
        id: String
    },
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error)
}

pub trait UserRepository: Repository<UserId, User> {
    fn store(u: User) -> Result<(), StoreError>;
}
