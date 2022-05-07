#[cfg_attr(test, mockall::automock)]
pub trait IdGenerator {
    fn generate(&self) -> String;
}

#[cfg_attr(test, mockall::automock(type IdGenerator = MockIdGenerator;))]
pub trait HaveIdGenerator {
    type IdGenerator: IdGenerator;
    fn id_generator(&self) -> &Self::IdGenerator;
}
