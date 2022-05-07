use account::effect::id_generator::IdGenerator;
use derive_more::Constructor;
use uuid::Uuid;

#[derive(Debug, Constructor, Clone)]
pub struct UUIDGenerator;

impl IdGenerator for UUIDGenerator {
    fn generate(&self) -> String {
        Uuid::new_v4().to_string()
    }
}
