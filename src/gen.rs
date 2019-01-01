extern crate rand;
extern crate term_size;

use self::rand::{rngs, Rng};
use crate::board;
use crate::error::{Error, Result};
use crate::land;

fn board_size(width: Option<usize>, height: Option<usize>) -> Result<(usize, usize)> {
    if let (Some(w), Some(h)) = (width, height) {
        return Ok((w, h));
    }
    let (w, h) = term_size::dimensions().ok_or(Error::CannotDetermineTermsize)?;
    // Divide by 2 since assuming that a terminal utilizes monospace font.
    Ok((width.unwrap_or(w / 2), height.unwrap_or(h)))
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

impl RandomBoardGen<rngs::ThreadRng> {
    pub fn new() -> Self {
        RandomBoardGen {
            rng: rand::thread_rng(),
        }
    }
}

impl<R: Rng> RandomBoardGen<R> {
    pub fn gen(
        &mut self,
        width: Option<usize>,
        height: Option<usize>,
    ) -> Result<board::Board<'static>> {
        let (width, height) = board_size(width, height)?;
        if width < 15 && height < 15 {
            Ok(self.gen_small(width, height))
        } else {
            Ok(self.gen_middle(width, height))
        }
    }

    pub fn gen_small(&mut self, width: usize, height: usize) -> board::Board<'static> {
        board::Board::build(width, height, |_, _| match self.rng.gen_range(0, 100) {
            0...15 => land::AQUA.clone(),
            16...55 => land::GROUND.clone(),
            56...85 => land::FOREST.clone(),
            86...99 => land::MOUNTAIN.clone(),
            _ => unreachable!(),
        })
    }

    pub fn gen_middle(&mut self, width: usize, height: usize) -> board::Board<'static> {
        unimplemented!()
    }
}
