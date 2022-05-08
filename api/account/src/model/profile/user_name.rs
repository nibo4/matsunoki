use derive_more::{Constructor, Deref};
use semval::prelude::*;
use serde::Serialize;

#[derive(Constructor, Deref, Debug, Clone, Serialize, PartialEq, Eq)]
pub struct UserName(pub String);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UserNameInvalidity {
    MinLength,
    MaxLength,
    Format,
}

impl Validate for UserName {
    type Invalidity = UserNameInvalidity;
    fn validate(&self) -> ValidationResult<Self::Invalidity> {
        ValidationContext::new()
            .invalidate_if(self.len() < 2, Self::Invalidity::MinLength)
            .invalidate_if(20 < self.len(), Self::Invalidity::MaxLength)
            .invalidate_if(
                self.chars().filter(|c| c.is_ascii_uppercase()).count() != 0,
                Self::Invalidity::Format,
            )
            .invalidate_if(
                self.chars()
                    .filter(|c| !c.is_ascii_alphanumeric() && *c != '_')
                    .count()
                    != 0,
                Self::Invalidity::Format,
            )
            .into()
    }
}

impl TryFrom<String> for UserName {
    type Error = (UserName, ValidationContext<UserNameInvalidity>);
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = UserName::new(value);
        Ok(value.into_validated()?.into())
    }
}

#[cfg(test)]
mod tests {
    use super::{UserName, UserNameInvalidity};

    #[test]
    fn test_try_from_string_user_name_is_ok() {
        let name_result = UserName::try_from("valid".to_string());
        assert!(name_result.is_ok());
        assert_eq!(name_result.unwrap(), UserName("valid".to_string()))
    }

    #[test]
    fn test_try_from_string_user_name_is_ok_when_smallest() {
        let name_result = UserName::try_from("va".to_string());
        assert!(name_result.is_ok());
        assert_eq!(name_result.unwrap(), UserName("va".to_string()))
    }

    #[test]
    fn test_try_from_string_user_name_is_ok_when_largest() {
        let name_result = UserName::try_from("vavavavavavavavavava".to_string());
        assert!(name_result.is_ok());
        assert_eq!(
            name_result.unwrap(),
            UserName("vavavavavavavavavava".to_string())
        )
    }

    #[test]
    fn test_try_from_string_user_name_is_ok_where_use_underscore() {
        let name_result = UserName::try_from("va_lid".to_string());
        assert!(name_result.is_ok());
        assert_eq!(name_result.unwrap(), UserName("va_lid".to_string()))
    }

    #[test]
    fn test_try_from_string_user_name_is_ok_where_use_numeric() {
        let name_result = UserName::try_from("val1d".to_string());
        assert!(name_result.is_ok());
        assert_eq!(name_result.unwrap(), UserName("val1d".to_string()))
    }

    #[test]
    fn test_try_from_string_user_name_is_err_where_use_hyphen() {
        let name_result = UserName::try_from("va-lid".to_string());
        assert!(name_result.is_err());
        assert_eq!(
            name_result.unwrap_err().1.into_iter().next(),
            Some(UserNameInvalidity::Format)
        );
    }

    #[test]
    fn test_try_from_string_user_name_is_err_where_use_percent() {
        let name_result = UserName::try_from("va%lid".to_string());
        assert!(name_result.is_err());
        assert_eq!(
            name_result.unwrap_err().1.into_iter().next(),
            Some(UserNameInvalidity::Format)
        );
    }

    #[test]
    fn test_try_from_string_user_name_is_err_where_use_big_arphabet() {
        let name_result = UserName::try_from("Valid".to_string());
        assert!(name_result.is_err());
        assert_eq!(
            name_result.unwrap_err().1.into_iter().next(),
            Some(UserNameInvalidity::Format)
        );
    }

    #[test]
    fn test_try_from_string_user_name_is_err_where_use_whitespace() {
        let name_result = UserName::try_from("V lid".to_string());
        assert!(name_result.is_err());
        assert_eq!(
            name_result.unwrap_err().1.into_iter().next(),
            Some(UserNameInvalidity::Format)
        );
    }

    #[test]
    fn test_try_from_string_user_name_is_err_where_use_whitespace_full_width() {
        let name_result = UserName::try_from("V　lid".to_string());
        assert!(name_result.is_err());
        assert_eq!(
            name_result.unwrap_err().1.into_iter().next(),
            Some(UserNameInvalidity::Format)
        );
    }

    #[test]
    fn test_try_from_string_user_name_is_err_when_1() {
        let name_result = UserName::try_from("v".to_string());
        assert!(name_result.is_err());
        assert_eq!(
            name_result.unwrap_err().1.into_iter().next(),
            Some(UserNameInvalidity::MinLength)
        );
    }

    #[test]
    fn test_try_from_string_user_name_is_err_when_21() {
        let name_result = UserName::try_from("vavavavavavavavavavaa".to_string());
        assert!(name_result.is_err());
        assert_eq!(
            name_result.unwrap_err().1.into_iter().next(),
            Some(UserNameInvalidity::MaxLength)
        );
    }
}
