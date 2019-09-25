#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod db;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
