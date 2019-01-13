extern crate serde;
extern crate serde_json;

use crate::land::Land;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

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
    cells: Vec<Vec<Land<'a>>>,
}

pub struct Iter<'a> {
    cells: &'a [Vec<Land<'a>>],
    x: usize,
    y: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Land<'a>;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.cells.len(), None)
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.cells.len() {
            return None;
        }
        let row = &self.cells[self.y];
        if self.x < row.len() {
            self.x += 1;
            Some(&row[self.x - 1])
        } else {
            self.x = 0;
            self.y += 1;
            self.next()
        }
    }
}

impl<'a> Board<'a> {
    pub fn build<F>(width: usize, height: usize, mut builder: F) -> Board<'a>
    where
        F: FnMut(usize, usize) -> Land<'a>,
    {
        let mut cells = Vec::with_capacity(height);
        for y in 0..height {
            let mut cols = Vec::with_capacity(width);
            for x in 0..width {
                cols.push(builder(x, y));
            }
            cells.push(cols);
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
    pub fn at(&self, x: usize, y: usize) -> &Land {
        &self.cells[y][x]
    }

    #[inline]
    pub fn rows(&self) -> std::slice::Iter<Vec<Land<'a>>> {
        self.cells.iter()
    }

    #[inline]
    pub fn at_mut<'b>(&mut self, p: &'b Pos) -> &'a mut Land {
        &mut self.cells[p.y][p.x]
    }

    #[inline]
    pub fn iter(&self) -> Iter {
        Iter {
            cells: &self.cells,
            x: 0,
            y: 0,
        }
    }
}

impl<'a> Index<Pos> for Board<'a> {
    type Output = Land<'a>;

    #[inline]
    fn index(&self, p: Pos) -> &Land<'a> {
        &self.cells[p.y][p.x]
    }
}

impl<'a> IndexMut<Pos> for Board<'a> {
    #[inline]
    fn index_mut(&mut self, p: Pos) -> &mut Land<'a> {
        &mut self.cells[p.y][p.x]
    }
}

impl<'a> serde::Serialize for Board<'a> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("width", &self.width)?;
        map.serialize_entry("height", &self.height)?;
        map.serialize_entry("cells", &self.cells)?;

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
}
