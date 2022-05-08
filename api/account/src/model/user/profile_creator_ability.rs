use super::User;
use crate::ability::profile_creator::ProfileCreator;
use crate::model::profile::avatar::Avatar;
use crate::model::profile::display_name::DisplayName;
use crate::model::profile::entity::{Profile, ProfileInvalidity};
use crate::model::profile::user_name::UserName;
use semval::prelude::*;

impl ProfileCreator for User {
    fn create_profile(
        &self,
        name: String,
        display_name: String,
        avatar: String,
    ) -> Result<Profile, ValidationContext<ProfileInvalidity>> {
        let profile = Profile {
            name: UserName(name),
            display_name: DisplayName(display_name),
            avatar: Avatar { url: avatar },
        };

        profile.validate()?;
        Ok(profile)
    }
}

#[cfg(test)]
mod tests {
    use super::User;
    use crate::{ability::profile_creator::ProfileCreator, model::user::UserId};

    #[test]
    fn create_profile_is_ok() {
        let user = User::new(UserId::new("dummy".to_string()), None);
        let profile_result = user.create_profile(
            "xxx".to_string(),
            "yyyy".to_string(),
            "https://lh3.googleusercontent.com/a-/AOh14GiibtsjDIo7GRGVpJQxg3pD0azpzNvYaeI7v9Lldg=s288-p-no".to_string()
        );
        assert!(profile_result.is_ok());
    }

    #[test]
    fn create_profile_is_err() {
        let user = User::new(UserId::new("dummy".to_string()), None);
        let profile_result = user.create_profile(
            "xxx".to_string(),
            "yyyy".to_string(),
            "adfasdfasdasd".to_string(),
        );
        assert!(profile_result.is_err());
    }
}
