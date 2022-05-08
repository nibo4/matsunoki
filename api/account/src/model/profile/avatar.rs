use semval::prelude::*;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Deserialize)]
pub struct Avatar {
    pub url: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AvatarInvalidity {
    NotUrl,
}

impl Validate for Avatar {
    type Invalidity = AvatarInvalidity;

    fn validate(&self) -> ValidationResult<Self::Invalidity> {
        ValidationContext::new()
            .invalidate_if(Url::parse(&self.url).is_err(), Self::Invalidity::NotUrl)
            .into()
    }
}

impl TryFrom<String> for Avatar {
    type Error = (Avatar, ValidationContext<AvatarInvalidity>);
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = Avatar { url: value };
        Ok(value.into_validated()?.into())
    }
}

#[cfg(test)]
mod tests {
    use super::Avatar;

    #[test]
    fn try_from_string_to_avatar_is_ok() {
        assert!(Avatar::try_from("https://example.com".to_string()).is_ok());
        assert_eq!(
            Avatar::try_from("https://example.com".to_string()).unwrap(),
            Avatar {
                url: "https://example.com".to_string()
            }
        )
    }

    #[test]
    fn try_from_string_to_avatar_is_err_when_invalid_url() {
        assert!(Avatar::try_from("invalid_url!".to_string()).is_err());
    }
}
