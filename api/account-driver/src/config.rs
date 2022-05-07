use account::effect::config::Config;
use derive_more::Constructor;

#[derive(Debug, Clone, Constructor)]
pub struct DefaultConfig {
    pub firebase_project_id: String,
    pub max_connections: usize,
}

impl Config for DefaultConfig {
    fn firebase_project_id(&self) -> &str {
        &self.firebase_project_id
    }
    fn max_connections(&self) -> &usize {
        &self.max_connections
    }
}
