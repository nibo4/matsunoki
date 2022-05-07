use account::adapter::firebase_auth::{AccessToken, FirebaseAuthDriver};
use account_driver::adapter::firebase_auth_adapter::*;
use account_driver::config::DefaultConfig;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
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
    let config = DefaultConfig::new(project_id.clone(), 5);
    let cache = Arc::new(Mutex::new(HashMap::new()));
    let adapter = DefaultFirebaseAuthAdapter::new(config, cache);
    let verify_result = adapter.verify(AccessToken::new(token.clone())).await;
    let verify_result = adapter.verify(AccessToken::new(token.clone())).await;
    println!("{:?}", verify_result);
}
