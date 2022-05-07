use account::adapter::firebase_auth::HaveFirebaseAuthDriver;
use account::effect::config::Config;
use account::effect::id_generator::HaveIdGenerator;
use account::repository::user_repository::HaveUserRepository;
use account_driver::adapter::firebase_auth_adapter::DefaultFirebaseAuthAdapter;
use account_driver::id_generator::UUIDGenerator;
use account_driver::repository::postgres_user_repository::PostgresUserRepository;

use std::env::var;

pub struct DefaultConfig {
    firebase_project_id: String,
}

impl Config for DefaultConfig {
    fn firebase_project_id(&self) -> &str {
        &self.firebase_project_id
    }
}

impl Default for DefaultConfig {
    fn default() -> Self {
        Self {
            firebase_project_id: var("ACCOUNT_FIREBASE_PROJECT_ID")
                .expect("env ACCOUNT_FIREBASE_PROJECT_ID is not defined"),
        }
    }
}

pub struct Kernel {
    user_repo: PostgresUserRepository,
    firebase_auth_adapter: DefaultFirebaseAuthAdapter,
    id_generator: UUIDGenerator,
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
