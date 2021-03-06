use derive_more::{Constructor, Deref};
use serde::{Deserialize, Serialize};

#[derive(Constructor, Deref, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DisplayName(pub String);
