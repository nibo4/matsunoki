use crate::model::meta::{AggregateRoot, Entity, Identifier};
use derive_more::{Constructor, Deref};

#[derive(Debug, Clone, PartialEq, Eq, Deref, Constructor, Default)]
pub struct UserId(pub String);

impl Identifier for UserId {}

#[derive(Debug, Clone, PartialEq, Eq, Constructor, Default)]
pub struct User {
    pub user_id: UserId,
}

impl Entity<UserId> for User {
    fn id(&self) -> &UserId {
        &self.user_id
    }
}

impl AggregateRoot<UserId> for User {}
