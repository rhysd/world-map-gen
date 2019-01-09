use crate::land::Land;
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

#[derive(Debug)]
pub struct Row<'a> {
    cols: Vec<Land<'a>>,
}

impl<'a> Row<'a> {
    #[inline]
    pub fn width(&self) -> usize {
        self.cols.len()
    }

    #[inline]
    pub fn cols(&self) -> std::slice::Iter<Land<'a>> {
        self.cols.iter()
    }
}

impl<'a> Index<usize> for Row<'a> {
    type Output = Land<'a>;

    #[inline]
    fn index(&self, idx: usize) -> &Land<'a> {
        &self.cols[idx]
    }
}

impl<'a> IndexMut<usize> for Row<'a> {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut Land<'a> {
        &mut self.cols[idx]
    }
}

#[derive(Debug)]
pub struct Board<'a> {
    rows: Vec<Row<'a>>,
}

impl<'a> Board<'a> {
    pub fn build<F>(width: usize, height: usize, mut builder: F) -> Board<'a>
    where
        F: FnMut(usize, usize) -> Land<'a>,
    {
        let mut rows = Vec::with_capacity(height);
        for y in 0..height {
            let mut cols = Vec::with_capacity(width);
            for x in 0..width {
                cols.push(builder(x, y));
            }
            rows.push(Row { cols });
        }
        Board { rows }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.rows[0].width()
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.rows.len()
    }

    #[inline]
    pub fn at(&self, p: &Pos) -> &Land {
        &self.rows[p.y][p.x]
    }

    #[inline]
    pub fn rows(&self) -> std::slice::Iter<Row<'a>> {
        self.rows.iter()
    }

    #[inline]
    pub fn at_mut<'b>(&mut self, p: &'b Pos) -> &'a mut Land {
        &mut self.rows[p.y][p.x]
    }
}

impl<'a> Index<usize> for Board<'a> {
    type Output = Row<'a>;

    #[inline]
    fn index(&self, idx: usize) -> &Row<'a> {
        &self.rows[idx]
    }
}

impl<'a> IndexMut<usize> for Board<'a> {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut Row<'a> {
        &mut self.rows[idx]
    }
}
