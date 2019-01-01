extern crate rand;

use self::rand::Rng;
use crate::board;
use crate::cell;

pub fn gen_small(width: usize, height: usize) -> board::Board<'static> {
    let mut rng = rand::thread_rng();
    board::Board::build(width, height, |_, _| match rng.gen_range(0, 100) {
        0...15 => cell::AQUA.clone(),
        16...55 => cell::GROUND.clone(),
        56...85 => cell::FOREST.clone(),
        86...99 => cell::MOUNTAIN.clone(),
        _ => unreachable!(),
    })
}
