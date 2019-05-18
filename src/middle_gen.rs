use rand;

use self::rand::seq::SliceRandom;
use self::rand::Rng;
use crate::board::{Board, Pos};
use crate::land::LandKind;
use crate::slope::SlopeGen;
use std::collections::HashSet;

pub struct MiddleBoardGen<'a, R: Rng> {
    rng: &'a mut R,
    width: usize,
    height: usize,
    num_towns: usize,
    min_distance: usize,
    down_rate: u8,
    num_tops: usize,
}

impl<'a, R: Rng> MiddleBoardGen<'a, R> {
    pub fn new<'b: 'a>(rng: &'b mut R, width: usize, height: usize) -> Self {
        let num_towns = width * height / 2048 + rng.gen_range(1, 4);
        let both = width + height;
        let min_distance = both.checked_div(num_towns).unwrap_or(both);
        // Note: Standard value is 20 at 48x36 board
        let down_rate = 12 + (48 * 36 * 8 / (width * height)) as u8;
        let num_tops = 3 + both * rng.gen_range(3, 7) / (48 + 36);

        MiddleBoardGen {
            rng,
            width,
            height,
            num_towns,
            min_distance,
            down_rate,
            num_tops,
        }
    }

    #[inline]
    fn land_kind(altitude: u8) -> LandKind {
        match altitude {
            0..=10 => LandKind::Sea,
            11..=40 => LandKind::Plain,
            41..=70 => LandKind::Forest,
            71..=99 => LandKind::Mountain,
            _ => unreachable!(),
        }
    }

    pub fn gen(&mut self) -> Board<'static> {
        let mut slope = SlopeGen::new(
            self.rng,
            self.width,
            self.height,
            self.down_rate,
            self.num_tops,
        );
        slope.gen();
        let altitudes = slope.altitudes;
        let tops = slope.tops;

        let mut plains = Vec::new();
        for (h, line) in altitudes.iter().enumerate() {
            for (w, alt) in line.iter().enumerate() {
                if Self::land_kind(*alt) == LandKind::Plain {
                    plains.push(Pos { x: w, y: h });
                }
            }
        }
        plains.as_mut_slice().shuffle(&mut self.rng);
        let plains = plains;

        let mut towns = HashSet::with_capacity(self.num_towns);

        for g in plains.iter() {
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
            let alt = altitudes[h][w];
            let p = Pos { x: w, y: h };
            if tops.contains(&p) {
                LandKind::Top.preset(alt)
            } else if towns.contains(&p) {
                LandKind::Town.preset(alt)
            } else {
                Self::land_kind(alt).preset(alt)
            }
        })
    }
}
