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
        while self.tops.len() < self.num_tops {
            let x = self.rng.gen_range(0, self.width);
            let y = self.rng.gen_range(0, self.height);
            self.slope(99, x, y);
            self.tops.insert(Pos { x, y });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_slope() {
        let mut rng = rand::thread_rng();
        let mut slope = SlopeGen::new(&mut rng, 3, 4, 1, 2);
        slope.gen();

        let alt = slope.altitudes;
        let top = slope.tops;

        assert_eq!(alt.len(), 4); // height
        assert_eq!(alt[0].len(), 3); // width
        assert_eq!(top.len(), 2);

        for y in 0..4 {
            for x in 0..3 {
                let a = alt[y][x];
                if top.contains(&Pos { x, y }) {
                    assert_eq!(a, 99);
                } else {
                    assert!(90 < a && a <= 99);
                }
            }
        }

        for Pos { x, y } in top.into_iter() {
            for (x, y) in &[
                (x, y.wrapping_sub(1)),
                (x, y + 1),
                (x.wrapping_sub(1), y),
                (x + 1, y),
            ] {
                if *x < 3 && *y < 4 {
                    let a = alt[*y][*x];
                    assert!(a == 98 || a == 99);
                }
            }
        }
    }
}
