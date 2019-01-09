extern crate termcolor;

use termcolor::{Color, ColorSpec};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LandKind {
    Sea,
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
            LandKind::Sea => "sea",
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
            LandKind::Sea => SEA.clone(),
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

macro_rules! define_lands {
    ($($name:ident = ($kind:ident, $color:expr);)+) => {
        lazy_static! {
            $(
                pub static ref $name: Land<'static> = Land {
                    kind: LandKind::$kind,
                    char: "██",
                    color: {
                        let mut c = ColorSpec::new();
                        c.set_fg(Some(Color::Ansi256($color)));
                        c
                    },
                    altitude: 0,
                };
            )+
        }
    }
}

define_lands! {
    SEA      = (Sea, 81);
    MOUNTAIN = (Mountain, 94);
    FOREST   = (Forest, 22);
    GROUND   = (Ground, 118);
    TOWN     = (Town, 226);
    TOP      = (Top, 102);
    ALPINE   = (Alpine, 58);
    DEEPSEA  = (DeepSea, 63);
    PATH     = (Path, 15);
}
