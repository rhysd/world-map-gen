extern crate serde;
extern crate serde_json;

use crate::land::Land;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::slice;

//       width
//  O--------------> x
// h|
// e|
// i|
// g|
// h|
// t|
//  V
//  y

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    #[inline]
    pub fn move_cost(&self, other: &Pos) -> usize {
        use std::cmp::{max, min};
        let dx = max(self.x, other.x) - min(self.x, other.x);
        let dy = max(self.y, other.y) - min(self.y, other.y);
        dx + dy
    }
}

#[derive(Debug, PartialEq)]
pub struct Board<'a> {
    pub width: usize,
    pub height: usize,
    cells: Vec<Land<'a>>,
}

impl<'a> Board<'a> {
    pub fn build<F>(width: usize, height: usize, mut builder: F) -> Board<'a>
    where
        F: FnMut(usize, usize) -> Land<'a>,
    {
        let mut cells = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                cells.push(builder(x, y));
            }
        }
        Board {
            cells,
            width,
            height,
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    fn index_at(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline]
    pub fn at(&self, x: usize, y: usize) -> &Land {
        &self.cells[self.index_at(x, y)]
    }

    #[inline]
    pub fn at_mut(&mut self, x: usize, y: usize) -> &'a mut Land {
        let idx = self.index_at(x, y);
        &mut self.cells[idx]
    }

    #[inline]
    pub fn iter<'b>(&'b self) -> slice::Iter<'b, Land<'a>> {
        self.cells.iter()
    }

    #[inline]
    pub fn iter_mut<'b>(&'b mut self) -> slice::IterMut<'b, Land<'a>> {
        self.cells.iter_mut()
    }

    #[inline]
    pub fn rows<'b>(&'b self) -> slice::Chunks<'b, Land<'a>> {
        self.cells.chunks(self.width)
    }

    #[inline]
    pub fn rows_mut<'b>(&'b mut self) -> slice::ChunksMut<'b, Land<'a>> {
        self.cells.chunks_mut(self.width)
    }
}

impl<'a> Index<Pos> for Board<'a> {
    type Output = Land<'a>;

    #[inline]
    fn index(&self, p: Pos) -> &Land<'a> {
        &self.cells[self.index_at(p.x, p.y)]
    }
}

impl<'a> IndexMut<Pos> for Board<'a> {
    #[inline]
    fn index_mut(&mut self, p: Pos) -> &mut Land<'a> {
        let idx = self.index_at(p.x, p.y);
        &mut self.cells[idx]
    }
}

impl<'a> serde::Serialize for Board<'a> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;

        struct Cells<'a, 'b: 'a> {
            w: usize,
            h: usize,
            vec: &'a Vec<Land<'b>>,
        }
        impl<'a, 'b> serde::Serialize for Cells<'a, 'b> {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use serde::ser::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some(self.h))?;
                for row in self.vec.chunks(self.w) {
                    seq.serialize_element(row)?;
                }
                seq.end()
            }
        }

        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("width", &self.width)?;
        map.serialize_entry("height", &self.height)?;
        map.serialize_entry(
            "cells",
            &Cells {
                w: self.width,
                h: self.height,
                vec: &self.cells,
            },
        )?;

        // Do not derive Serialize trait since legends should be contained in serialized JSON output
        let legends = self
            .iter()
            .map(|cell| (cell.kind, cell.kind.legend()))
            .collect::<HashMap<_, _>>();
        map.serialize_entry("legends", &legends)?;

        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::land::LandKind;
    use termcolor::ColorSpec;

    #[test]
    fn build_board() {
        let board = Board::build(2, 3, |x, y| Land {
            kind: LandKind::Town,
            char: "hi",
            color: ColorSpec::default(),
            altitude: (x + 2 * y) as u8,
        });
        assert_eq!(board.width, 2);
        assert_eq!(board.height, 3);
        for y in 0..board.height {
            for x in 0..board.width {
                let land = &board[Pos { x, y }];
                assert_eq!(land.kind, LandKind::Town);
                assert_eq!(land.altitude, (x + 2 * y) as u8);
                let land2 = board.at(x, y);
                assert_eq!(land, land2);
            }
        }
    }

    #[test]
    fn iter_board() {
        let mut idx = 0;
        let board = Board::build(2, 3, |_, _| {
            idx += 1;
            Land {
                kind: LandKind::Town,
                char: "hi",
                color: ColorSpec::default(),
                altitude: idx,
            }
        });
        for (idx, land) in board.iter().enumerate() {
            assert_eq!(land.kind, LandKind::Town);
            assert_eq!(land.altitude, (idx as u8) + 1);
        }
    }

    #[test]
    fn iter_empty_board() {
        let board = Board::build(0, 0, |_, _| Land::default());
        for _ in board.iter() {
            assert!(false);
        }
    }

    #[test]
    fn iter_rows() {
        let board = Board::build(2, 3, |x, y| Land {
            kind: LandKind::Town,
            char: "hi",
            color: ColorSpec::default(),
            altitude: (x + 2 * y) as u8,
        });
        for row in board.rows() {
            assert_eq!(row.len(), 2);
        }
        assert_eq!(board.rows().count(), 3);

        let mut board = Board::build(2, 3, |x, y| Land {
            kind: LandKind::Town,
            char: "hi",
            color: ColorSpec::default(),
            altitude: (x + 2 * y) as u8,
        });
        for row in board.rows_mut() {
            assert_eq!(row.len(), 2);
        }
        assert_eq!(board.rows_mut().count(), 3);
    }
}
