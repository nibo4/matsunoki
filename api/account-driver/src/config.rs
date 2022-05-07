use account::effect::config::Config;
use derive_more::Constructor;

#[derive(Debug, Constructor)]
pub struct DefaultConfig {
    firebase_project_id: String,
}

impl Config for DefaultConfig {
    fn firebase_project_id(&self) -> &str {
        &self.firebase_project_id
    }
}
