extern crate rand;

use self::rand::seq::SliceRandom;
use self::rand::Rng;
use crate::board::{Board, Pos};
use crate::land;
use std::collections::HashSet;

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
    pub fn new<'b: 'a>(rng: &'b mut R, width: usize, height: usize) -> Self {
        let mut altitudes = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(0);
            }
            altitudes.push(row);
        }

        let num_towns = width * height / 2048 + rng.gen_range(0, 4);
        let min_distance = if num_towns != 0 {
            (width + height) / num_towns
        } else {
            width + height
        };
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

    #[inline]
    fn land_kind(altitude: u8) -> land::LandKind {
        match altitude {
            0...10 => land::LandKind::Sea,
            11...40 => land::LandKind::Ground,
            41...70 => land::LandKind::Forest,
            71...99 => land::LandKind::Mountain,
            _ => unreachable!(),
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

    pub fn gen(&mut self) -> Board<'static> {
        let mut tops = HashSet::with_capacity(self.num_tops);

        // Generate tops of mountains. Every point must be unique so using HashSet
        while tops.len() < self.num_tops {
            let x = self.rng.gen_range(0, self.width);
            let y = self.rng.gen_range(0, self.height);
            tops.insert(Pos { x, y });
        }
        let tops = tops;

        for Pos { x, y } in tops.iter() {
            // Altitude is 0~99. Top is always at 99
            self.slope(99, *x, *y);
        }

        let mut grounds = Vec::new();
        for (h, line) in self.altitudes.iter().enumerate() {
            for (w, alt) in line.iter().enumerate() {
                if Self::land_kind(*alt) == land::LandKind::Ground {
                    grounds.push(Pos { x: w, y: h });
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
                .all(|p: &Pos| p.move_cost(g) > self.min_distance)
            {
                towns.insert(*g);
            }
        }
        let towns = towns;

        Board::build(self.width, self.height, |w, h| {
            let alt = self.altitudes[h][w];
            let p = Pos { x: w, y: h };
            let mut chosen = if tops.contains(&p) {
                land::TOP.clone()
            } else if towns.contains(&p) {
                land::TOWN.clone()
            } else {
                Self::land_kind(alt).constant()
            };
            chosen.altitude = alt;
            chosen
        })
    }
}
