#[cfg_attr(test, mockall::automock)]
pub trait Config {
    fn firebase_project_id(&self) -> &str;
    fn max_connections(&self) -> &u32;
}

#[cfg_attr(test, mockall::automock(type Config = MockConfig;))]
pub trait HaveConfig {
    type Config: Config;
    fn config(&self) -> &Self::Config;
}
