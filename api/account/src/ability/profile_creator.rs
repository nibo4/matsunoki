use crate::model::profile::entity::{Profile, ProfileInvalidity};
use semval::prelude::ValidationContext;

pub trait ProfileCreator {
    fn create_profile(
        &self,
        name: String,
        display_name: String,
        avatar: String,
    ) -> Result<Profile, ValidationContext<ProfileInvalidity>>;
}
