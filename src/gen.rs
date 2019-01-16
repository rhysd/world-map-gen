//! This module provides random world map generator
//!
//! `RandomBoardGen` is a key struct to generate a world map. It contains random number generator
//! as its internal state and generates a random world map of several resolutions.
//! There are 3 resolutions for generating a world map.
//!
//! - Low
//! - Middle
//! - High
//!
//! ```rust
//! use world_map_gen::gen::RandomBoardGen;
//!
//! // Default generator initialized with hardware random number.
//! let mut gen = RandomBoardGen::default();
//!
//! // A random map generator initialized with specific seed.
//! let mut gen = RandomBoardGen::from_seed(42);
//!
//! // Generate 3x4 board. Resolution is automatically determined from the size.
//! let board = gen.gen_auto(3, 4);
//! assert_eq!(board.width(), 3);
//! assert_eq!(board.height(), 4);
//!
//! // Generate 3x4 low resolution board.
//! let board = gen.gen_small(3, 4);
//! assert_eq!(board.width(), 3);
//! assert_eq!(board.height(), 4);
//!
//! // Generate 3x4 middle resolution board.
//! let board = gen.gen_middle(3, 4);
//! assert_eq!(board.width(), 3);
//! assert_eq!(board.height(), 4);
//!
//! // Generate 3x4 large resolution board.
//! let board = gen.gen_large(3, 4);
//! assert_eq!(board.width(), 3);
//! assert_eq!(board.height(), 4);
//! ```

extern crate rand;
extern crate term_size;

use self::rand::{rngs, Rng};
use crate::board::Board;
use crate::error::{Error, Result};
use crate::land::LandKind;
use crate::large_gen::LargeBoardGen;
use crate::middle_gen::MiddleBoardGen;

#[inline]
#[allow(clippy::or_fun_call)]
fn board_size(width: Option<usize>, height: Option<usize>) -> Result<(usize, usize)> {
    if let (Some(w), Some(h)) = (width, height) {
        return Ok((w, h));
    }
    let (w, h) = term_size::dimensions().ok_or(Error::CannotDetermineTermsize)?;
    // Divide by 2 since assuming that a terminal utilizes monospace font.
    let w = w / 2;
    Ok((width.unwrap_or(w), height.unwrap_or(std::cmp::min(w, h))))
}

/// Resolution of the board.
pub enum Resolution {
    /// Low resolution. There are limited kinds of cells in generated board.
    Low,
    /// Middle resolution. Mountains are generated with slopes and towns are put.
    Middle,
    /// High resolution. More kinds such as DeapSea and Highland are used. And paths between towns
    /// are drawn.
    High,
}

/// Random world map board generator
pub struct RandomBoardGen<R: Rng> {
    rng: R,
}

impl RandomBoardGen<rngs::StdRng> {
    /// Create a new random map generator. It's internal random number generator is initialized by
    /// the given seed. Standard random number generator is used.
    pub fn from_seed(seed: u64) -> Self {
        RandomBoardGen {
            rng: rand::SeedableRng::seed_from_u64(seed),
        }
    }
}

impl Default for RandomBoardGen<rngs::ThreadRng> {
    /// Create a new random map generator. It's internal random number generator is initialized by
    /// random number generated by hardware. Default thread-local random number generator is used.
    fn default() -> Self {
        RandomBoardGen {
            rng: rand::thread_rng(),
        }
    }
}

impl<R: Rng> RandomBoardGen<R> {
    /// Most generic method to generate a random map. When resolution is `None`, the resolution is
    /// determined from its size. When width and/or height are `None` they will be determined from
    /// the terminal size. In the case, when terminal size cannot be obtained, this method returns
    /// an error.
    pub fn gen(
        &mut self,
        resolution: &Option<Resolution>,
        width: Option<usize>,
        height: Option<usize>,
    ) -> Result<Board<'static>> {
        let (width, height) = board_size(width, height)?;
        Ok(match resolution {
            Some(Resolution::Low) => self.gen_small(width, height),
            Some(Resolution::Middle) => self.gen_middle(width, height),
            Some(Resolution::High) => self.gen_large(width, height),
            None => self.gen_auto(width, height),
        })
    }

    /// Generate a random map of given width and height. Resolution is automatically determined from
    /// its size.
    /// - When width and height are lower than 15: low resolution
    /// - When width and height are lower than 120: middle resolution
    /// - Otherwise: high resolution
    pub fn gen_auto(&mut self, width: usize, height: usize) -> Board<'static> {
        if width < 15 && height < 15 {
            self.gen_small(width, height)
        } else if width < 120 && height < 120 {
            self.gen_middle(width, height)
        } else {
            self.gen_large(width, height)
        }
    }

    /// Generate a low resolution random map of given width and height.
    pub fn gen_small(&mut self, width: usize, height: usize) -> Board<'static> {
        Board::build(width, height, |_, _| {
            let alt = self.rng.gen_range(0, 100);
            let mut chosen = match alt {
                0...15 => LandKind::Sea.constant(),
                16...55 => LandKind::Plain.constant(),
                56...85 => LandKind::Forest.constant(),
                86...99 => LandKind::Mountain.constant(),
                _ => unreachable!(),
            };
            chosen.altitude = alt;
            chosen
        })
    }

    /// Generate a middle resolution random map of given width and height.
    pub fn gen_middle(&mut self, width: usize, height: usize) -> Board<'static> {
        MiddleBoardGen::new(&mut self.rng, width, height).gen()
    }

    /// Generate a large resolution random map of given width and height.
    pub fn gen_large(&mut self, width: usize, height: usize) -> Board<'static> {
        LargeBoardGen::new(&mut self.rng, width, height).gen()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Pos;
    use crate::land::LandKind::*;

    #[test]
    fn same_seed() {
        let mut g = RandomBoardGen::from_seed(1);
        let b1 = g.gen_small(4, 4);
        let mut g = RandomBoardGen::from_seed(1);
        let b2 = g.gen_small(4, 4);
        assert_eq!(b1, b2);
    }

    #[test]
    fn default_gen() {
        let mut g = RandomBoardGen::default();
        let b1 = g.gen_small(4, 4);
        let mut g = RandomBoardGen::default();
        let b2 = g.gen_small(4, 4);
        assert_ne!(b1, b2);
    }

    #[test]
    fn gen_small() {
        let (w, h) = (3, 4);
        let b = RandomBoardGen::default().gen_auto(w, h);
        assert_eq!(b.width(), w);
        assert_eq!(b.height(), h);
        for y in 0..h {
            for x in 0..w {
                let p = Pos { x, y };
                let l = &b[p];
                assert!(
                    [Sea, Plain, Forest, Mountain].contains(&l.kind),
                    "{:?} at {:?}",
                    l,
                    p
                );
                assert!(l.altitude < 100);
            }
        }
    }

    #[test]
    fn gen_middle() {
        let (w, h) = (20, 20);
        let b = RandomBoardGen::default().gen_auto(w, h);
        assert_eq!(b.width(), w);
        assert_eq!(b.height(), h);

        let mut found_top = false;
        for y in 0..h {
            for x in 0..w {
                let p = Pos { x, y };
                let l = &b[p];
                assert!(
                    [Sea, Plain, Forest, Mountain, Top, Town].contains(&l.kind),
                    "{:?} at {:?}",
                    l,
                    p
                );
                if l.kind == Top {
                    found_top = true;
                }
                if l.kind == Town {
                    // The same altitude as plains
                    assert!(11 <= l.altitude && l.altitude <= 40);
                }
                assert!(l.altitude < 100);
            }
        }

        assert!(found_top);
    }

    #[test]
    fn gen_large() {
        let (w, h) = (150, 200);
        let b = RandomBoardGen::default().gen_auto(w, h);
        assert_eq!(b.width(), w);
        assert_eq!(b.height(), h);

        let mut found_top = false;
        let mut found_town = false;
        let mut found_path = false;

        for y in 0..h {
            for x in 0..w {
                let p = Pos { x, y };
                let l = &b[p];
                assert!(
                    [DeepSea, Sea, Plain, Forest, Mountain, Highland, Top, Town, Path]
                        .contains(&l.kind),
                    "{:?} at {:?}",
                    l,
                    p
                );
                if l.kind == Top {
                    found_top = true;
                }
                if l.kind == Town {
                    found_town = true;
                    // TODO Check path connectivity
                }
                if l.kind == Path {
                    found_path = true;
                }
                assert!(l.altitude < 100);
            }
        }

        assert!(found_top);
        assert!(found_town);
        assert!(found_path);
    }
}
