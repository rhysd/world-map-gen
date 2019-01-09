extern crate serde;
extern crate serde_json;
extern crate termcolor;

use crate::color;
use termcolor::{Color, ColorSpec};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
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
    pub fn constant(self) -> Land<'static> {
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

impl<'a> serde::Serialize for Land<'a> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("kind", &self.kind)?;
        map.serialize_entry("char", self.char)?;
        map.serialize_entry("color", &color::serializable_spec(&self.color))?;
        map.serialize_entry("altitude", &self.altitude)?;
        map.end()
    }
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
    PATH     = (Path, 193);
}
