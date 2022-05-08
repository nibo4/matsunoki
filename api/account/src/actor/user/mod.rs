use derive_more::{Constructor, Deref};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deref, Constructor, Default, Serialize, Deserialize)]
pub struct UserId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Constructor, Default)]
pub struct User(pub UserId);

pub mod profile_creator_ability;
