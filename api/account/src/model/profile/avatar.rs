use derive_more::{Constructor, Deref};
use serde::Serialize;

#[derive(Constructor, Deref, Debug, Clone, Serialize)]
pub struct Avatar {
    url: String,
}
