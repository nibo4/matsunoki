use super::avatar::Avatar;
use super::display_name::DisplayName;
use super::user_name::UserName;
use derive_more::{Constructor, Deref};
use serde::Serialize;

#[derive(Constructor, Debug, Clone, Serialize)]
pub struct Profile {
    pub name: UserName,
    pub display_name: DisplayName,
    pub avatar: Avatar,
}
