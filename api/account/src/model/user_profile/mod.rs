use crate::model::meta::{AggregateRoot, Entity, Identifier};
use derive_more::{Constructor, Deref};
use serde::{Deserialize, Serialize};

use crate::model::profile::entity::Profile;

#[derive(Debug, Clone, PartialEq, Eq, Deref, Constructor, Default, Serialize, Deserialize)]
pub struct UserId(pub String);

impl Identifier for UserId {}

#[derive(Debug, Clone, Serialize, Deserialize, Constructor)]
pub struct UserProfile {
    pub id: UserId,
    pub prifile: Profile,
}

impl Entity<UserId> for UserProfile {
    fn id(&self) -> &UserId {
        &self.id
    }
}

impl AggregateRoot<UserId> for UserProfile {}
