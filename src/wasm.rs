extern crate termcolor2rgb;
extern crate wasm_bindgen;

use crate::board;
use crate::gen::RandomBoardGen;
use crate::land::LandKind;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Cell {
    pub kind: LandKind,
    pub altitude: u8,
}

#[wasm_bindgen]
pub struct Board {
    pub width: usize,
    pub height: usize,
    cells: Vec<Cell>,
    colors: HashMap<LandKind, (u8, u8, u8)>,
}

#[wasm_bindgen]
impl Board {
    pub fn at(&self, x: usize, y: usize) -> Cell {
        let idx = y * self.width + x;
        self.cells[idx]
    }

    pub fn land_color(&self, kind: LandKind) -> String {
        match self.colors.get(&kind) {
            Some((r, g, b)) => format!("#{:02x}{:02x}{:02x}", r, g, b),
            None => "".to_string(),
        }
    }

    pub fn land_legend(&self, kind: LandKind) -> String {
        kind.legend().to_string()
    }
}

impl Board {
    fn from_board(b: board::Board) -> Board {
        use termcolor2rgb::ColorExt;

        let mut cells = Vec::with_capacity(b.width() * b.height());
        let mut colors = HashMap::new();
        for cell in b.iter() {
            cells.push(Cell {
                kind: cell.kind,
                altitude: cell.altitude,
            });
            colors
                .entry(cell.kind)
                .or_insert_with(|| cell.color.fg().map(|c| c.to_rgb()).unwrap_or((0, 0, 0)));
        }

        Board {
            width: b.width(),
            height: b.height(),
            cells,
            colors,
        }
    }
}

#[wasm_bindgen]
pub fn gen_board(width: usize, height: usize) -> Board {
    let mut gen = RandomBoardGen::default();
    let board = gen.gen_auto(width, height);
    Board::from_board(board)
}
