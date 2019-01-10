extern crate rand;
extern crate term_size;

use self::rand::{rngs, Rng};
use crate::board::Board;
use crate::error::{Error, Result};
use crate::land;
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

pub enum Resolution {
    Low,
    Middle,
    High,
}

pub struct RandomBoardGen<R: Rng> {
    rng: R,
}

impl RandomBoardGen<rngs::StdRng> {
    pub fn from_seed(seed: u64) -> Self {
        RandomBoardGen {
            rng: rand::SeedableRng::seed_from_u64(seed),
        }
    }
}

impl Default for RandomBoardGen<rngs::ThreadRng> {
    fn default() -> Self {
        RandomBoardGen {
            rng: rand::thread_rng(),
        }
    }
}

impl<R: Rng> RandomBoardGen<R> {
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

    pub fn gen_auto(&mut self, width: usize, height: usize) -> Board<'static> {
        if width < 15 && height < 15 {
            self.gen_small(width, height)
        } else if width < 120 && height < 120 {
            self.gen_middle(width, height)
        } else {
            self.gen_large(width, height)
        }
    }

    pub fn gen_small(&mut self, width: usize, height: usize) -> Board<'static> {
        Board::build(width, height, |_, _| {
            let alt = self.rng.gen_range(0, 100);
            let mut chosen = match alt {
                0...15 => land::SEA.clone(),
                16...55 => land::GROUND.clone(),
                56...85 => land::FOREST.clone(),
                86...99 => land::MOUNTAIN.clone(),
                _ => unreachable!(),
            };
            chosen.altitude = alt;
            chosen
        })
    }

    pub fn gen_middle(&mut self, width: usize, height: usize) -> Board<'static> {
        MiddleBoardGen::new(&mut self.rng, width, height).gen()
    }

    pub fn gen_large(&mut self, width: usize, height: usize) -> Board<'static> {
        LargeBoardGen::new(&mut self.rng, width, height).gen()
    }
}
