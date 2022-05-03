pub mod ability;
pub mod effect;
pub mod model;
pub mod repository;
pub mod usecase;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
