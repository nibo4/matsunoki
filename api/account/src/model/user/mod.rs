use crate::model::meta::{AggregateRoot, Entity, Identifier};
use derive_more::{Constructor, Deref};

use super::login_provider::LoginProvider;

#[derive(Debug, Clone, PartialEq, Eq, Deref, Constructor, Default)]
pub struct UserId(pub String);

impl Identifier for UserId {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct User {
    pub id: UserId,
    pub providers: Vec<LoginProvider>,
}

impl User {
    pub fn new(id: UserId, providers: Option<Vec<LoginProvider>>) -> Self {
        User {
            id,
            providers: providers.unwrap_or_else(|| vec![] as Vec<LoginProvider>),
        }
    }
}

impl Entity<UserId> for User {
    fn id(&self) -> &UserId {
        &self.id
    }
}

impl AggregateRoot<UserId> for User {}
