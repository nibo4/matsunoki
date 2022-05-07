pub mod adapter;
pub mod cache;
pub mod config;
pub mod db_conn;
pub mod repository;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
