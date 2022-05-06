use derive_more::Constructor;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    Google,
}

#[derive(Error, Debug, Constructor)]
#[error("Failed provider kind convert. source: {id}")]
pub struct ProviderKindConvertError {
    id: String,
}

impl TryFrom<String> for ProviderKind {
    type Error = ProviderKindConvertError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Google" => Ok(ProviderKind::Google),
            _ => Err(ProviderKindConvertError::new(value)),
        }
    }
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
