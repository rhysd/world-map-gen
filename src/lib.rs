//! Several kinds of random World maps generator for games
//!
//! This library provides a world maps generator with following interfaces:
//!
//! 1. Rust library as an API
//! 2. CLI tool to generate maps in terminal as visual output or as JSON output
//! 3. WebAssembly npm package for Web
//!
//! Please read README.md of repository hosted on GitHub.
//!
//! https://github.com/rhysd/world-map-gen
//!
//! This document explains 1., as an API library for Rust.
//!
//! This library provides some modules to handle a world map as one board filled up with cells.
//!
//! - `land`: `land::Land` struct represents each cell in a board
//! - `board`: `board::Board` struct represents one world map. The struct is JSON serializable with `serde_json`
//! - `draw`: Helper to draw a board to terminal or as JSON
//! - `gen`: A random world map generator to build `board::Board` struct. It provides algorithms for 3 kinds of resolutions
//! - `error`: Error type which may be returned from a map generator
//!
//! ```rust
//! use world_map_gen::RandomBoardGen;
//!
//! // Create generator instance with default random number generator
//! let mut generator = RandomBoardGen::default();
//!
//! // Generate 40x40 random world map. Map resolution (low, middle, high) is automatically
//! // determined by its width and height here.
//! //   - Low: width and height are less than 15
//! //   - Middle: width and height are less than 120
//! //   - High: Otherwise
//! let board = generator.gen_auto(40, 40);
//!
//! // Iterate each cells per row
//! for (i, row) in board.rows().enumerate() {
//!     println!("Row: {}", i);
//!     for cell in row {
//!         // cell is a world_map_gen::land::Land instance
//!
//!         // Lands are categorized with kind (e.g. Sea, Plain, Forest, Mountain, ...)
//!         println!("Kind: {:?}", cell.kind);
//!
//!         // Each cell as its altitude. For example, sea's altitude is lower and mountain's is
//!         // higher
//!         println!("Altitude: {}", cell.altitude);
//!     }
//! }
//! ```

#![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

pub mod board;
pub mod draw;
pub mod error;
pub mod gen;
pub mod land;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

mod color;
mod large_gen;
mod middle_gen;
mod slope;

pub use crate::board::Board;
pub use crate::error::Result;
pub use crate::gen::RandomBoardGen;
pub use crate::land::LandKind;

use cfg_if::cfg_if;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}
