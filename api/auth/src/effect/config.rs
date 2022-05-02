pub trait Config {
    fn provider_id(&self) -> String;
    fn token(&self) -> String;
    fn redirect_url(&self) -> String;
}

trait HaveConfig {
    type Config: Config;
    fn config(&self) -> Self::Config;
}
