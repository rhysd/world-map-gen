#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

pub mod board;
pub mod draw;
pub mod error;
pub mod gen;
pub mod land;

mod color;
mod large_gen;
mod middle_gen;
mod slope;
