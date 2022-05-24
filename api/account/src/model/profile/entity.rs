use super::avatar::{Avatar, AvatarInvalidity};
use super::display_name::DisplayName;
use super::user_name::{UserName, UserNameInvalidity};
use derive_more::Constructor;
use semval::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Constructor, PartialEq, Eq)]
pub struct Profile {
    pub name: UserName,
    pub display_name: DisplayName,
    pub avatar: Avatar,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ProfileInvalidity {
    UserName(UserNameInvalidity),
    Avatar(AvatarInvalidity),
}

impl Validate for Profile {
    type Invalidity = ProfileInvalidity;

    fn validate(&self) -> ValidationResult<Self::Invalidity> {
        ValidationContext::new()
            .validate_with(&self.name, Self::Invalidity::UserName)
            .validate_with(&self.avatar, Self::Invalidity::Avatar)
            .into()
    }
}
