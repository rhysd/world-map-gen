extern crate termcolor;

use termcolor::{Color, ColorSpec};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LandKind {
    Aqua,
    Mountain,
    Forest,
    Ground,
    Town,
    Top,
}

impl LandKind {
    pub fn name(&self) -> &str {
        match self {
            LandKind::Aqua => "aqua",
            LandKind::Mountain => "mountain",
            LandKind::Forest => "forest",
            LandKind::Ground => "ground",
            LandKind::Town => "town",
            LandKind::Top => "top",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Land<'a> {
    pub kind: LandKind,
    pub char: &'a str,
    pub color: ColorSpec,
    pub altitude: u8,
}

lazy_static! {
    pub static ref AQUA: Land<'static> = Land {
        kind: LandKind::Aqua,
        char: "██",
        color: {
            let mut c = ColorSpec::new();
            c.set_fg(Some(Color::Ansi256(81)));
            c
        },
        altitude: 0,
    };
    pub static ref MOUNTAIN: Land<'static> = Land {
        kind: LandKind::Mountain,
        char: "██",
        color: {
            let mut c = ColorSpec::new();
            c.set_fg(Some(Color::Ansi256(94)));
            c
        },
        altitude: 0,
    };
    pub static ref FOREST: Land<'static> = Land {
        kind: LandKind::Forest,
        char: "██",
        color: {
            let mut c = ColorSpec::new();
            c.set_fg(Some(Color::Ansi256(22)));
            c
        },
        altitude: 0,
    };
    pub static ref GROUND: Land<'static> = Land {
        kind: LandKind::Ground,
        char: "██",
        color: {
            let mut c = ColorSpec::new();
            c.set_fg(Some(Color::Ansi256(118)));
            c
        },
        altitude: 0,
    };
    pub static ref TOWN: Land<'static> = Land {
        kind: LandKind::Town,
        char: "██",
        color: {
            let mut c = ColorSpec::new();
            c.set_fg(Some(Color::Ansi256(226)));
            c
        },
        altitude: 0,
    };
    pub static ref TOP: Land<'static> = Land {
        kind: LandKind::Top,
        char: "██",
        color: {
            let mut c = ColorSpec::new();
            c.set_fg(Some(Color::Ansi256(101)));
            c
        },
        altitude: 0,
    };
}
