use derive_more::Constructor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    Google,
}

impl Default for ProviderKind {
    fn default() -> Self {
        Self::Google
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Constructor, Default)]
pub struct IdInProvider(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Constructor, Default)]
pub struct LoginProvider {
    pub kind: ProviderKind,
    pub id_in_provider: IdInProvider,
}
