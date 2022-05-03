use derive_more::{Constructor, Deref};
use crate::model::meta::{AggregateRoot,Identifier, Entity};

#[derive(Debug, Clone, PartialEq, Eq, Deref, Constructor, Default)]
pub struct UserId(String);

impl Identifier for UserId {}

#[derive(Debug, Clone, PartialEq, Eq, Constructor, Default)]
pub struct User {
    id: UserId
}

impl Entity<UserId> for User {
    fn id(&self) -> &UserId {
        &self.id
    }
}

impl AggregateRoot<UserId> for User {}
