use account::adapter::firebase_auth::HaveFirebaseAuthDriver;
use account::effect::id_generator::HaveIdGenerator;
use account::repository::user_profile_repository::HaveUserProfileRepository;
use account::repository::user_repository::HaveUserRepository;
use account_driver::adapter::firebase_auth_adapter::DefaultFirebaseAuthAdapter;
use account_driver::config::DefaultConfig;
use account_driver::db_conn::build_conn;
use account_driver::id_generator::UUIDGenerator;
use account_driver::repository::postgres_user_profile_repository::PostgresUserProfileRepository;
use account_driver::repository::postgres_user_repository::PostgresUserRepository;

use derive_more::Deref;
use std::collections::HashMap;
use std::env::var;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deref)]
pub struct HttpControllerConfig(DefaultConfig);

impl Default for HttpControllerConfig {
    fn default() -> Self {
        Self(DefaultConfig {
            firebase_project_id: var("ACCOUNT_FIREBASE_PROJECT_ID")
                .expect("env ACCOUNT_FIREBASE_PROJECT_ID is not defined"),
            max_connections: var("ACCOUNT_DB_MAX_CONNECTIONS")
                .expect("env ACCOUNT_DB_MAX_CONNECTIONS is not defined")
                .parse::<u32>()
                .expect("env ACCOUNT_DB_MAX_CONNECTIONS is not numeric"),
        })
    }
}

#[derive(Clone)]
pub struct Kernel {
    user_repo: PostgresUserRepository,
    user_profile_repo: PostgresUserProfileRepository,
    firebase_auth_adapter: DefaultFirebaseAuthAdapter,
    id_generator: UUIDGenerator,
}

impl HaveUserProfileRepository for Kernel {
    type UserProfileRepository = PostgresUserProfileRepository;
    fn user_profile_repository(&self) -> &Self::UserProfileRepository {
        &self.user_profile_repo
    }
}

impl HaveUserRepository for Kernel {
    type UserRepository = PostgresUserRepository;
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repo
    }
}

impl HaveFirebaseAuthDriver for Kernel {
    type FirebaseAuthDriver = DefaultFirebaseAuthAdapter;
    fn firebase_auth(&self) -> &Self::FirebaseAuthDriver {
        &self.firebase_auth_adapter
    }
}

impl HaveIdGenerator for Kernel {
    type IdGenerator = UUIDGenerator;
    fn id_generator(&self) -> &Self::IdGenerator {
        &self.id_generator
    }
}

pub async fn init() -> Kernel {
    let config = HttpControllerConfig::default();
    let pool = build_conn(&config.0).await;
    let jwks_cache = Arc::new(Mutex::new(HashMap::new()));

    Kernel {
        user_repo: PostgresUserRepository::new(pool.clone()),
        user_profile_repo: PostgresUserProfileRepository::new(pool.clone()),
        firebase_auth_adapter: DefaultFirebaseAuthAdapter::new(config.0.clone(), jwks_cache),
        id_generator: UUIDGenerator::new(),
    }
}
