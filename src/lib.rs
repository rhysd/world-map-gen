#[macro_use]
extern crate lazy_static;

pub mod board;
pub mod land;
pub mod draw;
pub mod error;
pub mod gen;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
