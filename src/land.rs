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
    Alpine,
    DeepSea,
    Path,
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
            LandKind::Alpine => "alpine",
            LandKind::DeepSea => "deep sea",
            LandKind::Path => "path",
        }
    }

    pub fn constant(&self) -> Land<'static> {
        match self {
            LandKind::Aqua => AQUA.clone(),
            LandKind::Mountain => MOUNTAIN.clone(),
            LandKind::Forest => FOREST.clone(),
            LandKind::Ground => GROUND.clone(),
            LandKind::Town => TOWN.clone(),
            LandKind::Top => TOP.clone(),
            LandKind::Alpine => ALPINE.clone(),
            LandKind::DeepSea => DEEPSEA.clone(),
            LandKind::Path => PATH.clone(),
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
            c.set_fg(Some(Color::Ansi256(102)));
            c
        },
        altitude: 0,
    };
    pub static ref ALPINE: Land<'static> = Land {
        kind: LandKind::Top,
        char: "██",
        color: {
            let mut c = ColorSpec::new();
            c.set_fg(Some(Color::Ansi256(58)));
            c
        },
        altitude: 0,
    };
    pub static ref DEEPSEA: Land<'static> = Land {
        kind: LandKind::DeepSea,
        char: "██",
        color: {
            let mut c = ColorSpec::new();
            c.set_fg(Some(Color::Ansi256(63)));
            c
        },
        altitude: 0,
    };
    pub static ref PATH: Land<'static> = Land {
        kind: LandKind::Path,
        char: "██",
        color: {
            let mut c = ColorSpec::new();
            c.set_fg(Some(Color::Ansi256(15)));
            c
        },
        altitude: 0,
    };
}
