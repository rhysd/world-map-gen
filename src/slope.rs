use rand;

use self::rand::Rng;
use crate::board::Pos;
use std::cmp;
use std::collections::HashSet;

pub struct SlopeGen<'a, R: Rng> {
    pub altitudes: Vec<Vec<u8>>,
    pub tops: HashSet<Pos>,
    rng: &'a mut R,
    width: usize,
    height: usize,
    down_rate: u8,
    num_tops: usize,
}

#[repr(u8)]
enum Dir {
    Above,
    Below,
    Left,
    Right,
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
    fn down(&mut self, altitude: u8, x: usize, y: usize, down_rates: [u8; 4], dir: Dir) {
        let delta = self.rng.gen_range(0, down_rates[dir as usize]);
        let altitude = altitude.saturating_sub(delta);
        if self.altitudes[y][x] >= altitude {
            // Skip when the altitude is already calculated as other mountain's slope
            return;
        }
        self.slope(altitude, x, y, down_rates);
    }

    // Create a slope of mountain
    fn slope(&mut self, altitude: u8, x: usize, y: usize, down_rates: [u8; 4]) {
        self.altitudes[y][x] = altitude;
        if altitude == 0 {
            return;
        }
        if x > 0 {
            self.down(altitude, x - 1, y, down_rates, Dir::Left);
        }
        if self.width - 1 > x {
            self.down(altitude, x + 1, y, down_rates, Dir::Right);
        }
        if y > 0 {
            self.down(altitude, x, y - 1, down_rates, Dir::Above);
        }
        if self.height - 1 > y {
            self.down(altitude, x, y + 1, down_rates, Dir::Below);
        }
    }

    // Generate down rates with 30% noise per direction
    fn random_down_rates(&mut self) -> [u8; 4] {
        let mut rates = [0, 0, 0, 0];
        let min = i32::from(self.down_rate) * 7 / 10;
        let deg = i32::from(self.down_rate) - min;
        if deg > 0 {
            let mut budget = deg * 4;
            for rate in rates.iter_mut().take(3) {
                let deg_max = cmp::min(budget, deg * 2 + 1);
                if deg_max > 0 {
                    let noise = self.rng.gen_range(0, deg_max);
                    *rate = (min as u8) + (noise as u8);
                    budget -= noise;
                }
            }
            rates[3] = (min as u8) + cmp::max(budget, 0) as u8;
        }
        rates
    }

    pub fn gen(&mut self) {
        while self.tops.len() < self.num_tops {
            let x = self.rng.gen_range(0, self.width);
            let y = self.rng.gen_range(0, self.height);
            let down_rates = self.random_down_rates();
            self.slope(99, x, y, down_rates);
            self.tops.insert(Pos { x, y });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_slope_invariant() {
        let mut rng = rand::thread_rng();
        let mut slope = SlopeGen::new(&mut rng, 3, 4, 5, 2);
        slope.gen();

        let alt = slope.altitudes;
        let top = slope.tops;

        assert_eq!(alt.len(), 4); // height
        assert_eq!(alt[0].len(), 3); // width
        assert_eq!(top.len(), 2);

        for y in 0..4 {
            for x in 0..3 {
                if top.contains(&Pos { x, y }) {
                    // Skip tops
                    continue;
                }
                let a = alt[y][x];
                let ix = x as isize;
                let iy = y as isize;
                let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                let b = dirs.iter().any(|(dx, dy)| {
                    let x = ix + dx;
                    let y = iy + dy;
                    if x < 0 || 3 <= x || y < 0 || 4 <= y {
                        false
                    } else {
                        alt[y as usize][x as usize] >= a
                    }
                });
                assert!(
                    b,
                    "All cells altitudes around the cell are smaller at ({}, {})",
                    x, y
                );
            }
        }
    }
}
