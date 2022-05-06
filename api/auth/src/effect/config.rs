#[cfg_attr(test, mockall::automock)]
pub trait Config {
    fn provider_id(&self) -> String;
    fn token(&self) -> String;
    fn redirect_url(&self) -> String;
}

#[cfg_attr(test, mockall::automock(type Config = MockConfig;))]
pub trait HaveConfig {
    type Config: Config;
    fn config(&self) -> Self::Config;
}
