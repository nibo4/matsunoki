use crate::actor::user::UserId;
use crate::model::meta::{AggregateRoot, Entity, Identifier};
use crate::model::profile::entity::Profile;
use derive_more::{Constructor, Deref, From};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, Deref, Constructor, Default, Serialize, Deserialize, From,
)]
pub struct UserProfileId(pub String);

impl From<UserId> for UserProfileId {
    fn from(u: UserId) -> Self {
        UserProfileId::new(u.0)
    }
}

impl Identifier for UserProfileId {}

#[derive(Debug, Clone, Serialize, Deserialize, Constructor, PartialEq, Eq)]
pub struct UserProfile {
    pub id: UserProfileId,
    pub profile: Profile,
}

impl Entity<UserProfileId> for UserProfile {
    fn id(&self) -> &UserProfileId {
        &self.id
    }
}

impl AggregateRoot<UserProfileId> for UserProfile {}
