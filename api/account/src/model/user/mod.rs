use crate::model::meta::{AggregateRoot, Entity, Identifier};
use derive_more::{Constructor, Deref};
use serde::Serialize;

use super::login_provider::LoginProvider;

#[derive(Debug, Clone, PartialEq, Eq, Deref, Constructor, Default, Serialize)]
pub struct UserId(pub String);

impl Identifier for UserId {}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct User {
    pub id: UserId,
    pub providers: Vec<LoginProvider>,
}

impl Into<crate::actor::user::User> for User {
    fn into(self) -> crate::actor::user::User {
        crate::actor::user::User::new(crate::actor::user::UserId::new(self.id.0))
    }
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
