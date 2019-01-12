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
                16...55 => land::PLAIN.clone(),
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
        assert_eq!(b.width, w);
        assert_eq!(b.height, h);
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
        assert_eq!(b.width, w);
        assert_eq!(b.height, h);

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
        assert_eq!(b.width, w);
        assert_eq!(b.height, h);

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
