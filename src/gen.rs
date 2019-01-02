extern crate rand;
extern crate term_size;

use self::rand::seq::SliceRandom;
use self::rand::{rngs, Rng};
use crate::board::{Board, Point};
use crate::error::{Error, Result};
use crate::land;
use std::collections::HashSet;

fn board_size(width: Option<usize>, height: Option<usize>) -> Result<(usize, usize)> {
    if let (Some(w), Some(h)) = (width, height) {
        return Ok((w, h));
    }
    let (w, h) = term_size::dimensions().ok_or(Error::CannotDetermineTermsize)?;
    // Divide by 2 since assuming that a terminal utilizes monospace font.
    Ok((
        width.unwrap_or(w / 2),
        height.unwrap_or(std::cmp::min(w / 2, h)),
    ))
}

pub enum Scale {
    Small,
    Middle,
    Large,
}

pub struct MiddleBoardGen<'a, R: Rng + 'a> {
    rng: &'a mut R,
    altitudes: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    num_towns: usize,
    min_distance: usize,
    down_rate: u8,
    num_tops: usize,
}

impl<'a, R: Rng> MiddleBoardGen<'a, R> {
    fn new<'b: 'a>(rng: &'b mut R, width: usize, height: usize) -> Self {
        let mut altitudes = Vec::with_capacity(height);
        for _ in 0..width {
            altitudes.push(Vec::with_capacity(width));
        }

        let average_town_area_len = rng.gen_range(15, 18);
        let num_towns = width * height / (average_town_area_len * average_town_area_len);
        let min_distance = (width + height) / num_towns;
        // Note: Standard value is 20 at 48x36 board
        let down_rate = 12 + (48 * 36 * 8 / (width * height)) as u8;
        let num_tops = 3 + (width + height) * rng.gen_range(3, 7) / (48 + 36);

        MiddleBoardGen {
            rng,
            altitudes,
            width,
            height,
            num_towns,
            min_distance,
            down_rate,
            num_tops,
        }
    }

    // Down a slope
    fn down(&mut self, altitude: u8, x: usize, y: usize) {
        let delta = self.rng.gen_range(0, self.down_rate);
        if altitude < delta {
            // Skip when altitude is min since default value of altitude is 0
            return;
        }
        let altitude = altitude - delta;
        if self.altitudes[y][x] >= altitude {
            // Skip when the altitude is already calculated as other mountain's slope
            return;
        }
        self.slope(altitude, x, y);
    }

    // Create a slope of mountain
    fn slope(&mut self, altitude: u8, x: usize, y: usize) {
        self.altitudes[y][x] = altitude;
        if x > 0 {
            self.down(altitude, x - 1, y);
        }
        if self.width - 1 > x {
            self.down(altitude, x + 1, y);
        }
        if y > 0 {
            self.down(altitude, x, y - 1);
        }
        if self.height - 1 > y {
            self.down(altitude, x, y + 1);
        }
    }

    fn gen(&mut self) -> Result<Board<'static>> {
        let mut tops = HashSet::with_capacity(self.num_tops);

        // Generate tops of mountains. Every point must be unique so using HashSet
        while tops.len() < self.num_tops {
            let x = self.rng.gen_range(0, self.width);
            let y = self.rng.gen_range(0, self.height);
            tops.insert(Point { x, y });
        }
        let tops = tops;

        // Initialize altitudes
        for line in self.altitudes.iter_mut() {
            for _ in 0..self.width {
                line.push(0);
            }
        }

        for Point { x, y } in tops.iter() {
            // Altitude is 0~99. Top is always at 99
            self.slope(99, *x, *y);
        }

        let mut grounds = Vec::new();
        for (h, line) in self.altitudes.iter().enumerate() {
            for (w, alt) in line.iter().enumerate() {
                if 10 < *alt && *alt <= 40 {
                    grounds.push(Point { x: w, y: h });
                }
            }
        }
        grounds.as_mut_slice().shuffle(&mut self.rng);
        let grounds = grounds;

        let mut towns = HashSet::with_capacity(self.num_towns);

        for g in grounds.iter() {
            if towns.len() == self.num_towns {
                break;
            }
            if towns
                .iter()
                .all(|p: &Point| p.move_cost(g) > self.min_distance)
            {
                towns.insert(*g);
            }
        }
        let towns = towns;

        Ok(Board::build(self.width, self.height, |w, h| {
            let alt = self.altitudes[h][w];
            let p = Point { x: w, y: h };
            let mut chosen = if tops.contains(&p) {
                land::TOP.clone()
            } else if towns.contains(&p) {
                land::TOWN.clone()
            } else {
                match alt {
                    0...10 => land::AQUA.clone(),
                    11...40 => land::GROUND.clone(),
                    41...70 => land::FOREST.clone(),
                    71...99 => land::MOUNTAIN.clone(),
                    _ => unreachable!(),
                }
            };
            chosen.altitude = alt;
            chosen
        }))
    }
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
        scale: Option<Scale>,
        width: Option<usize>,
        height: Option<usize>,
    ) -> Result<Board<'static>> {
        let (width, height) = board_size(width, height)?;
        match scale {
            Some(Scale::Small) => self.gen_small(width, height),
            Some(Scale::Middle) => self.gen_middle(width, height),
            Some(Scale::Large) => unreachable!(),
            None if width < 15 && height < 15 => self.gen_small(width, height),
            None => self.gen_middle(width, height),
        }
    }

    fn gen_small(&mut self, width: usize, height: usize) -> Result<Board<'static>> {
        Ok(Board::build(width, height, |_, _| {
            let alt = self.rng.gen_range(0, 100);
            let mut chosen = match alt {
                0...15 => land::AQUA.clone(),
                16...55 => land::GROUND.clone(),
                56...85 => land::FOREST.clone(),
                86...99 => land::MOUNTAIN.clone(),
                _ => unreachable!(),
            };
            chosen.altitude = alt;
            chosen
        }))
    }

    fn gen_middle(&mut self, width: usize, height: usize) -> Result<Board<'static>> {
        if width * height < 10 {
            return Err(Error::TooSmallBoard(width, height));
        }

        MiddleBoardGen::new(&mut self.rng, width, height).gen()
    }
}
