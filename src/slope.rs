extern crate rand;

use self::rand::Rng;
use crate::board::Pos;
use std::cmp;
use std::collections::HashSet;

pub struct SlopeGen<'a, R: Rng + 'a> {
    pub altitudes: Vec<Vec<u8>>,
    pub tops: HashSet<Pos>,
    rng: &'a mut R,
    width: usize,
    height: usize,
    down_rate: u8,
    num_tops: usize,
}

impl<'a, R: Rng> SlopeGen<'a, R> {
    pub fn new<'b: 'a>(
        rng: &'b mut R,
        width: usize,
        height: usize,
        down_rate: u8,
        num_tops: usize,
    ) -> Self {
        let mut altitudes = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(0);
            }
            altitudes.push(row);
        }

        // Too small num_tops causes infinite loop at .gen()
        let num_tops = cmp::min(num_tops, width * height);

        SlopeGen {
            rng,
            altitudes,
            width,
            height,
            down_rate,
            num_tops,
            tops: HashSet::with_capacity(num_tops),
        }
    }

    // Down a slope
    fn down(&mut self, altitude: u8, x: usize, y: usize) {
        let delta = self.rng.gen_range(0, self.down_rate);
        let altitude = altitude.saturating_sub(delta);
        if self.altitudes[y][x] >= altitude {
            // Skip when the altitude is already calculated as other mountain's slope
            return;
        }
        self.slope(altitude, x, y);
    }

    // Create a slope of mountain
    fn slope(&mut self, altitude: u8, x: usize, y: usize) {
        self.altitudes[y][x] = altitude;
        if altitude == 0 {
            return;
        }
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

    pub fn gen(&mut self) {
        println!("num tops: {}", self.num_tops);
        while self.tops.len() < self.num_tops {
            let x = self.rng.gen_range(0, self.width);
            let y = self.rng.gen_range(0, self.height);
            self.slope(99, x, y);
            self.tops.insert(Pos { x, y });
        }
    }
}
