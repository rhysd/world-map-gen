extern crate termcolor;

use termcolor::{Color, ColorSpec};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CellKind {
    Aqua,
    Mountain,
    Forest,
    Ground,
}

impl CellKind {
    pub fn name(&self) -> &str {
        match self {
            CellKind::Aqua => "aqua",
            CellKind::Mountain => "mountain",
            CellKind::Forest => "forest",
            CellKind::Ground => "ground",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell<'a> {
    pub kind: CellKind,
    pub char: &'a str,
    pub color: ColorSpec,
}

lazy_static! {
    pub static ref AQUA: Cell<'static> = Cell {
        kind: CellKind::Aqua,
        char: "██",
        color: {
            let mut c = ColorSpec::new();
            c.set_fg(Some(Color::Ansi256(81)));
            c
        },
    };
    pub static ref MOUNTAIN: Cell<'static> = Cell {
        kind: CellKind::Mountain,
        char: "██",
        color: {
            let mut c = ColorSpec::new();
            c.set_fg(Some(Color::Ansi256(94)));
            c
        },
    };
    pub static ref FOREST: Cell<'static> = Cell {
        kind: CellKind::Forest,
        char: "██",
        color: {
            let mut c = ColorSpec::new();
            c.set_fg(Some(Color::Ansi256(22)));
            c
        },
    };
    pub static ref GROUND: Cell<'static> = Cell {
        kind: CellKind::Ground,
        char: "██",
        color: {
            let mut c = ColorSpec::new();
            c.set_fg(Some(Color::Ansi256(118)));
            c
        },
    };
}
