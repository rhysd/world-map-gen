//! This module provides representation of cells in a board. And also provides constants for default cells.
//!
//! ```rust
//! extern crate termcolor;
//!
//! use world_map_gen::land::{Land, LandKind};
//!
//! // You can get default cell
//! let cell = Land::default();
//!
//!// Also your original cell can be defined
//! let cell = Land {
//!     kind: LandKind::Town,
//!     char: "[]",
//!     color: {
//!         let mut c = termcolor::ColorSpec::new();
//!         c.set_fg(Some(termcolor::Color::Black));
//!         c
//!     },
//!     altitude: 0,
//! };
//!
//! // Or you can use preset cells with specifying their altitudes
//! let cell = LandKind::Sea.preset(3);
//! let cell = LandKind::Forest.preset(50);
//! let cell = LandKind::Mountain.preset(80);
//! ```

extern crate serde;
extern crate serde_json;
extern crate termcolor;
#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;

use crate::color;
use termcolor::{Color, ColorSpec};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Represents one cell in a board
#[derive(Debug, Clone, PartialEq)]
pub struct Land<'a> {
    /// Kind of the cell like sea, plain, mountain, forest, ...
    pub kind: LandKind,
    /// Character to represent the cell. It is mainly used for rendering the board to terminal
    pub char: &'a str,
    /// Color of the cell
    pub color: ColorSpec,
    /// Altitude of the cell. It is 0 by default, and will be set on generating a map.
    pub altitude: u8,
}

impl<'a> serde::Serialize for Land<'a> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("kind", &self.kind)?;
        map.serialize_entry("char", self.char)?;
        map.serialize_entry("color", &color::SerializableColorSpec(&self.color))?;
        map.serialize_entry("altitude", &self.altitude)?;
        map.end()
    }
}

impl Default for Land<'static> {
    fn default() -> Land<'static> {
        Land {
            kind: LandKind::Plain,
            char: "██",
            color: ColorSpec::default(),
            altitude: 0,
        }
    }
}

macro_rules! define_lands {
    ($($name:ident = ($kind:ident, $color:expr, $legend:expr);)+) => {
        /// Represents the kind of cell. `preset()` method returns a preset cell constants for
        /// the kind.
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
        #[allow(missing_docs)]
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize)]
        pub enum LandKind {
            $($kind,)+
        }

        impl LandKind {
            /// Creates a preset constant with given altitude.
            #[inline]
            pub fn preset(self, altitude: u8) -> Land<'static> {
                match self {
                    $(
                        LandKind::$kind => { let mut l = $name.clone(); l.altitude = altitude; l },
                    )+
                }
            }

            /// Creates a reference to preset constant
            #[inline]
            pub fn preset_ref(self) -> &'static Land<'static> {
                match self {
                    $(
                        LandKind::$kind => &$name,
                    )+
                }
            }

            /// Returns a legend string for the land kind.
            #[inline]
            pub fn legend(self) -> &'static str {
                match self {
                    $(
                        LandKind::$kind => $legend,
                    )+
                }
            }
        }

        lazy_static! {
            $(
                static ref $name: Land<'static> = Land {
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
    SEA      = (Sea,      81, "Sea");
    MOUNTAIN = (Mountain, 94, "Mountain");
    FOREST   = (Forest,   22, "Forest");
    PLAIN    = (Plain,   118, "Plain");
    TOWN     = (Town,    226, "Town");
    TOP      = (Top,     102, "Top of Mountain");
    ALPINE   = (Highland, 58, "Highland");
    DEEPSEA  = (DeepSea,  63, "Deep Sea");
    PATH     = (Path,    193, "Path");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn constants() {
        let mut saw = HashSet::new();
        for kind in &[
            LandKind::Sea,
            LandKind::Mountain,
            LandKind::Forest,
            LandKind::Plain,
            LandKind::Town,
            LandKind::Top,
            LandKind::Highland,
            LandKind::DeepSea,
            LandKind::Path,
        ] {
            let land = kind.preset(0);
            assert_eq!(&land.kind, kind);
            match land.color.fg() {
                Some(Color::Ansi256(c)) => assert!(saw.insert(*c), "{}", *c),
                c => assert!(false, "{:?}", c),
            }
        }
    }
}
