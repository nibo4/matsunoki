pub mod ability;
pub mod model;
pub mod usecase;
pub mod effect;
pub mod repository;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
