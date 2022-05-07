use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type Cache<T> = Arc<Mutex<HashMap<String, T>>>;

pub trait HaveCache<T> {
    fn cache(&self) -> &Cache<T>;
}
