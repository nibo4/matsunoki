use auth::adapter::firebase_auth::{AccessToken, FirebaseAuthDriver};
use auth_driver::adapter::firebase_auth_adapter::*;
use auth_driver::config::DefaultConfig;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let token = match args.get(1) {
        Some(t) => t,
        None => {
            unreachable!("token is not found")
        }
    };
    let project_id = match args.get(2) {
        Some(t) => t,
        None => {
            unreachable!("token is not found")
        }
    };
    let config = DefaultConfig::new(project_id.clone());
    let adapter = DefaultFirebaseAuthAdapter::new(config);
    let verify_result = adapter.verify(AccessToken::new(token.clone())).await;
    println!("{:?}", verify_result);
}
