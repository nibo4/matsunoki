use derive_more::Constructor;

#[derive(Debug, Clone, PartialEq, Eq, Constructor, Default)]
pub struct User {
    uid: String
}
