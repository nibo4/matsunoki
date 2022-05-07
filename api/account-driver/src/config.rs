use account::effect::config::Config;
use derive_more::Constructor;

#[derive(Debug, Clone, Constructor)]
pub struct DefaultConfig {
    pub firebase_project_id: String,
    pub max_connections: u32,
}

impl Config for DefaultConfig {
    fn firebase_project_id(&self) -> &str {
        &self.firebase_project_id
    }
    fn max_connections(&self) -> &u32 {
        &self.max_connections
    }
}
