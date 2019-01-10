extern crate serde;
extern crate serde_json;
extern crate termcolor;
use termcolor::{Color, ColorSpec};

fn color2rgb(color: &Color) -> (u8, u8, u8) {
    match color {
        Color::Rgb(r, g, b) => (*r, *g, *b),
        Color::Black => (0x00, 0x00, 0x00),
        Color::Red => (0x80, 0x00, 0x00),
        Color::Green => (0x00, 0x80, 0x00),
        Color::Yellow => (0x80, 0x80, 0x00),
        Color::Blue => (0x00, 0x00, 0x80),
        Color::Magenta => (0x80, 0x00, 0x80),
        Color::Cyan => (0x00, 0x80, 0x80),
        Color::White => (0xc0, 0xc0, 0xc0),
        // Primary 3-bit (8 colors). Unique representation!
        Color::Ansi256(00) => (0x00, 0x00, 0x00),
        Color::Ansi256(01) => (0x80, 0x00, 0x00),
        Color::Ansi256(02) => (0x00, 0x80, 0x00),
        Color::Ansi256(03) => (0x80, 0x80, 0x00),
        Color::Ansi256(04) => (0x00, 0x00, 0x80),
        Color::Ansi256(05) => (0x80, 0x00, 0x80),
        Color::Ansi256(06) => (0x00, 0x80, 0x80),
        Color::Ansi256(07) => (0xc0, 0xc0, 0xc0),
        // Equivalent "bright" versions of original 8 colors.
        Color::Ansi256(08) => (0x80, 0x80, 0x80),
        Color::Ansi256(09) => (0xff, 0x00, 0x00),
        Color::Ansi256(10) => (0x00, 0xff, 0x00),
        Color::Ansi256(11) => (0xff, 0xff, 0x00),
        Color::Ansi256(12) => (0x00, 0x00, 0xff),
        Color::Ansi256(13) => (0xff, 0x00, 0xff),
        Color::Ansi256(14) => (0x00, 0xff, 0xff),
        Color::Ansi256(15) => (0xff, 0xff, 0xff),
        // Strictly ascending.
        Color::Ansi256(16) => (0x00, 0x00, 0x00),
        Color::Ansi256(17) => (0x00, 0x00, 0x5f),
        Color::Ansi256(18) => (0x00, 0x00, 0x87),
        Color::Ansi256(19) => (0x00, 0x00, 0xaf),
        Color::Ansi256(20) => (0x00, 0x00, 0xd7),
        Color::Ansi256(21) => (0x00, 0x00, 0xff),
        Color::Ansi256(22) => (0x00, 0x5f, 0x00),
        Color::Ansi256(23) => (0x00, 0x5f, 0x5f),
        Color::Ansi256(24) => (0x00, 0x5f, 0x87),
        Color::Ansi256(25) => (0x00, 0x5f, 0xaf),
        Color::Ansi256(26) => (0x00, 0x5f, 0xd7),
        Color::Ansi256(27) => (0x00, 0x5f, 0xff),
        Color::Ansi256(28) => (0x00, 0x87, 0x00),
        Color::Ansi256(29) => (0x00, 0x87, 0x5f),
        Color::Ansi256(30) => (0x00, 0x87, 0x87),
        Color::Ansi256(31) => (0x00, 0x87, 0xaf),
        Color::Ansi256(32) => (0x00, 0x87, 0xd7),
        Color::Ansi256(33) => (0x00, 0x87, 0xff),
        Color::Ansi256(34) => (0x00, 0xaf, 0x00),
        Color::Ansi256(35) => (0x00, 0xaf, 0x5f),
        Color::Ansi256(36) => (0x00, 0xaf, 0x87),
        Color::Ansi256(37) => (0x00, 0xaf, 0xaf),
        Color::Ansi256(38) => (0x00, 0xaf, 0xd7),
        Color::Ansi256(39) => (0x00, 0xaf, 0xff),
        Color::Ansi256(40) => (0x00, 0xd7, 0x00),
        Color::Ansi256(41) => (0x00, 0xd7, 0x5f),
        Color::Ansi256(42) => (0x00, 0xd7, 0x87),
        Color::Ansi256(43) => (0x00, 0xd7, 0xaf),
        Color::Ansi256(44) => (0x00, 0xd7, 0xd7),
        Color::Ansi256(45) => (0x00, 0xd7, 0xff),
        Color::Ansi256(46) => (0x00, 0xff, 0x00),
        Color::Ansi256(47) => (0x00, 0xff, 0x5f),
        Color::Ansi256(48) => (0x00, 0xff, 0x87),
        Color::Ansi256(49) => (0x00, 0xff, 0xaf),
        Color::Ansi256(50) => (0x00, 0xff, 0xd7),
        Color::Ansi256(51) => (0x00, 0xff, 0xff),
        Color::Ansi256(52) => (0x5f, 0x00, 0x00),
        Color::Ansi256(53) => (0x5f, 0x00, 0x5f),
        Color::Ansi256(54) => (0x5f, 0x00, 0x87),
        Color::Ansi256(55) => (0x5f, 0x00, 0xaf),
        Color::Ansi256(56) => (0x5f, 0x00, 0xd7),
        Color::Ansi256(57) => (0x5f, 0x00, 0xff),
        Color::Ansi256(58) => (0x5f, 0x5f, 0x00),
        Color::Ansi256(59) => (0x5f, 0x5f, 0x5f),
        Color::Ansi256(60) => (0x5f, 0x5f, 0x87),
        Color::Ansi256(61) => (0x5f, 0x5f, 0xaf),
        Color::Ansi256(62) => (0x5f, 0x5f, 0xd7),
        Color::Ansi256(63) => (0x5f, 0x5f, 0xff),
        Color::Ansi256(64) => (0x5f, 0x87, 0x00),
        Color::Ansi256(65) => (0x5f, 0x87, 0x5f),
        Color::Ansi256(66) => (0x5f, 0x87, 0x87),
        Color::Ansi256(67) => (0x5f, 0x87, 0xaf),
        Color::Ansi256(68) => (0x5f, 0x87, 0xd7),
        Color::Ansi256(69) => (0x5f, 0x87, 0xff),
        Color::Ansi256(70) => (0x5f, 0xaf, 0x00),
        Color::Ansi256(71) => (0x5f, 0xaf, 0x5f),
        Color::Ansi256(72) => (0x5f, 0xaf, 0x87),
        Color::Ansi256(73) => (0x5f, 0xaf, 0xaf),
        Color::Ansi256(74) => (0x5f, 0xaf, 0xd7),
        Color::Ansi256(75) => (0x5f, 0xaf, 0xff),
        Color::Ansi256(76) => (0x5f, 0xd7, 0x00),
        Color::Ansi256(77) => (0x5f, 0xd7, 0x5f),
        Color::Ansi256(78) => (0x5f, 0xd7, 0x87),
        Color::Ansi256(79) => (0x5f, 0xd7, 0xaf),
        Color::Ansi256(80) => (0x5f, 0xd7, 0xd7),
        Color::Ansi256(81) => (0x5f, 0xd7, 0xff),
        Color::Ansi256(82) => (0x5f, 0xff, 0x00),
        Color::Ansi256(83) => (0x5f, 0xff, 0x5f),
        Color::Ansi256(84) => (0x5f, 0xff, 0x87),
        Color::Ansi256(85) => (0x5f, 0xff, 0xaf),
        Color::Ansi256(86) => (0x5f, 0xff, 0xd7),
        Color::Ansi256(87) => (0x5f, 0xff, 0xff),
        Color::Ansi256(88) => (0x87, 0x00, 0x00),
        Color::Ansi256(89) => (0x87, 0x00, 0x5f),
        Color::Ansi256(90) => (0x87, 0x00, 0x87),
        Color::Ansi256(91) => (0x87, 0x00, 0xaf),
        Color::Ansi256(92) => (0x87, 0x00, 0xd7),
        Color::Ansi256(93) => (0x87, 0x00, 0xff),
        Color::Ansi256(94) => (0x87, 0x5f, 0x00),
        Color::Ansi256(95) => (0x87, 0x5f, 0x5f),
        Color::Ansi256(96) => (0x87, 0x5f, 0x87),
        Color::Ansi256(97) => (0x87, 0x5f, 0xaf),
        Color::Ansi256(98) => (0x87, 0x5f, 0xd7),
        Color::Ansi256(99) => (0x87, 0x5f, 0xff),
        Color::Ansi256(100) => (0x87, 0x87, 0x00),
        Color::Ansi256(101) => (0x87, 0x87, 0x5f),
        Color::Ansi256(102) => (0x87, 0x87, 0x87),
        Color::Ansi256(103) => (0x87, 0x87, 0xaf),
        Color::Ansi256(104) => (0x87, 0x87, 0xd7),
        Color::Ansi256(105) => (0x87, 0x87, 0xff),
        Color::Ansi256(106) => (0x87, 0xaf, 0x00),
        Color::Ansi256(107) => (0x87, 0xaf, 0x5f),
        Color::Ansi256(108) => (0x87, 0xaf, 0x87),
        Color::Ansi256(109) => (0x87, 0xaf, 0xaf),
        Color::Ansi256(110) => (0x87, 0xaf, 0xd7),
        Color::Ansi256(111) => (0x87, 0xaf, 0xff),
        Color::Ansi256(112) => (0x87, 0xd7, 0x00),
        Color::Ansi256(113) => (0x87, 0xd7, 0x5f),
        Color::Ansi256(114) => (0x87, 0xd7, 0x87),
        Color::Ansi256(115) => (0x87, 0xd7, 0xaf),
        Color::Ansi256(116) => (0x87, 0xd7, 0xd7),
        Color::Ansi256(117) => (0x87, 0xd7, 0xff),
        Color::Ansi256(118) => (0x87, 0xff, 0x00),
        Color::Ansi256(119) => (0x87, 0xff, 0x5f),
        Color::Ansi256(120) => (0x87, 0xff, 0x87),
        Color::Ansi256(121) => (0x87, 0xff, 0xaf),
        Color::Ansi256(122) => (0x87, 0xff, 0xd7),
        Color::Ansi256(123) => (0x87, 0xff, 0xff),
        Color::Ansi256(124) => (0xaf, 0x00, 0x00),
        Color::Ansi256(125) => (0xaf, 0x00, 0x5f),
        Color::Ansi256(126) => (0xaf, 0x00, 0x87),
        Color::Ansi256(127) => (0xaf, 0x00, 0xaf),
        Color::Ansi256(128) => (0xaf, 0x00, 0xd7),
        Color::Ansi256(129) => (0xaf, 0x00, 0xff),
        Color::Ansi256(130) => (0xaf, 0x5f, 0x00),
        Color::Ansi256(131) => (0xaf, 0x5f, 0x5f),
        Color::Ansi256(132) => (0xaf, 0x5f, 0x87),
        Color::Ansi256(133) => (0xaf, 0x5f, 0xaf),
        Color::Ansi256(134) => (0xaf, 0x5f, 0xd7),
        Color::Ansi256(135) => (0xaf, 0x5f, 0xff),
        Color::Ansi256(136) => (0xaf, 0x87, 0x00),
        Color::Ansi256(137) => (0xaf, 0x87, 0x5f),
        Color::Ansi256(138) => (0xaf, 0x87, 0x87),
        Color::Ansi256(139) => (0xaf, 0x87, 0xaf),
        Color::Ansi256(140) => (0xaf, 0x87, 0xd7),
        Color::Ansi256(141) => (0xaf, 0x87, 0xff),
        Color::Ansi256(142) => (0xaf, 0xaf, 0x00),
        Color::Ansi256(143) => (0xaf, 0xaf, 0x5f),
        Color::Ansi256(144) => (0xaf, 0xaf, 0x87),
        Color::Ansi256(145) => (0xaf, 0xaf, 0xaf),
        Color::Ansi256(146) => (0xaf, 0xaf, 0xd7),
        Color::Ansi256(147) => (0xaf, 0xaf, 0xff),
        Color::Ansi256(148) => (0xaf, 0xd7, 0x00),
        Color::Ansi256(149) => (0xaf, 0xd7, 0x5f),
        Color::Ansi256(150) => (0xaf, 0xd7, 0x87),
        Color::Ansi256(151) => (0xaf, 0xd7, 0xaf),
        Color::Ansi256(152) => (0xaf, 0xd7, 0xd7),
        Color::Ansi256(153) => (0xaf, 0xd7, 0xff),
        Color::Ansi256(154) => (0xaf, 0xff, 0x00),
        Color::Ansi256(155) => (0xaf, 0xff, 0x5f),
        Color::Ansi256(156) => (0xaf, 0xff, 0x87),
        Color::Ansi256(157) => (0xaf, 0xff, 0xaf),
        Color::Ansi256(158) => (0xaf, 0xff, 0xd7),
        Color::Ansi256(159) => (0xaf, 0xff, 0xff),
        Color::Ansi256(160) => (0xd7, 0x00, 0x00),
        Color::Ansi256(161) => (0xd7, 0x00, 0x5f),
        Color::Ansi256(162) => (0xd7, 0x00, 0x87),
        Color::Ansi256(163) => (0xd7, 0x00, 0xaf),
        Color::Ansi256(164) => (0xd7, 0x00, 0xd7),
        Color::Ansi256(165) => (0xd7, 0x00, 0xff),
        Color::Ansi256(166) => (0xd7, 0x5f, 0x00),
        Color::Ansi256(167) => (0xd7, 0x5f, 0x5f),
        Color::Ansi256(168) => (0xd7, 0x5f, 0x87),
        Color::Ansi256(169) => (0xd7, 0x5f, 0xaf),
        Color::Ansi256(170) => (0xd7, 0x5f, 0xd7),
        Color::Ansi256(171) => (0xd7, 0x5f, 0xff),
        Color::Ansi256(172) => (0xd7, 0x87, 0x00),
        Color::Ansi256(173) => (0xd7, 0x87, 0x5f),
        Color::Ansi256(174) => (0xd7, 0x87, 0x87),
        Color::Ansi256(175) => (0xd7, 0x87, 0xaf),
        Color::Ansi256(176) => (0xd7, 0x87, 0xd7),
        Color::Ansi256(177) => (0xd7, 0x87, 0xff),
        Color::Ansi256(178) => (0xd7, 0xaf, 0x00),
        Color::Ansi256(179) => (0xd7, 0xaf, 0x5f),
        Color::Ansi256(180) => (0xd7, 0xaf, 0x87),
        Color::Ansi256(181) => (0xd7, 0xaf, 0xaf),
        Color::Ansi256(182) => (0xd7, 0xaf, 0xd7),
        Color::Ansi256(183) => (0xd7, 0xaf, 0xff),
        Color::Ansi256(184) => (0xd7, 0xd7, 0x00),
        Color::Ansi256(185) => (0xd7, 0xd7, 0x5f),
        Color::Ansi256(186) => (0xd7, 0xd7, 0x87),
        Color::Ansi256(187) => (0xd7, 0xd7, 0xaf),
        Color::Ansi256(188) => (0xd7, 0xd7, 0xd7),
        Color::Ansi256(189) => (0xd7, 0xd7, 0xff),
        Color::Ansi256(190) => (0xd7, 0xff, 0x00),
        Color::Ansi256(191) => (0xd7, 0xff, 0x5f),
        Color::Ansi256(192) => (0xd7, 0xff, 0x87),
        Color::Ansi256(193) => (0xd7, 0xff, 0xaf),
        Color::Ansi256(194) => (0xd7, 0xff, 0xd7),
        Color::Ansi256(195) => (0xd7, 0xff, 0xff),
        Color::Ansi256(196) => (0xff, 0x00, 0x00),
        Color::Ansi256(197) => (0xff, 0x00, 0x5f),
        Color::Ansi256(198) => (0xff, 0x00, 0x87),
        Color::Ansi256(199) => (0xff, 0x00, 0xaf),
        Color::Ansi256(200) => (0xff, 0x00, 0xd7),
        Color::Ansi256(201) => (0xff, 0x00, 0xff),
        Color::Ansi256(202) => (0xff, 0x5f, 0x00),
        Color::Ansi256(203) => (0xff, 0x5f, 0x5f),
        Color::Ansi256(204) => (0xff, 0x5f, 0x87),
        Color::Ansi256(205) => (0xff, 0x5f, 0xaf),
        Color::Ansi256(206) => (0xff, 0x5f, 0xd7),
        Color::Ansi256(207) => (0xff, 0x5f, 0xff),
        Color::Ansi256(208) => (0xff, 0x87, 0x00),
        Color::Ansi256(209) => (0xff, 0x87, 0x5f),
        Color::Ansi256(210) => (0xff, 0x87, 0x87),
        Color::Ansi256(211) => (0xff, 0x87, 0xaf),
        Color::Ansi256(212) => (0xff, 0x87, 0xd7),
        Color::Ansi256(213) => (0xff, 0x87, 0xff),
        Color::Ansi256(214) => (0xff, 0xaf, 0x00),
        Color::Ansi256(215) => (0xff, 0xaf, 0x5f),
        Color::Ansi256(216) => (0xff, 0xaf, 0x87),
        Color::Ansi256(217) => (0xff, 0xaf, 0xaf),
        Color::Ansi256(218) => (0xff, 0xaf, 0xd7),
        Color::Ansi256(219) => (0xff, 0xaf, 0xff),
        Color::Ansi256(220) => (0xff, 0xd7, 0x00),
        Color::Ansi256(221) => (0xff, 0xd7, 0x5f),
        Color::Ansi256(222) => (0xff, 0xd7, 0x87),
        Color::Ansi256(223) => (0xff, 0xd7, 0xaf),
        Color::Ansi256(224) => (0xff, 0xd7, 0xd7),
        Color::Ansi256(225) => (0xff, 0xd7, 0xff),
        Color::Ansi256(226) => (0xff, 0xff, 0x00),
        Color::Ansi256(227) => (0xff, 0xff, 0x5f),
        Color::Ansi256(228) => (0xff, 0xff, 0x87),
        Color::Ansi256(229) => (0xff, 0xff, 0xaf),
        Color::Ansi256(230) => (0xff, 0xff, 0xd7),
        Color::Ansi256(231) => (0xff, 0xff, 0xff),
        // Gray-scale range.
        Color::Ansi256(232) => (0x08, 0x08, 0x08),
        Color::Ansi256(233) => (0x12, 0x12, 0x12),
        Color::Ansi256(234) => (0x1c, 0x1c, 0x1c),
        Color::Ansi256(235) => (0x26, 0x26, 0x26),
        Color::Ansi256(236) => (0x30, 0x30, 0x30),
        Color::Ansi256(237) => (0x3a, 0x3a, 0x3a),
        Color::Ansi256(238) => (0x44, 0x44, 0x44),
        Color::Ansi256(239) => (0x4e, 0x4e, 0x4e),
        Color::Ansi256(240) => (0x58, 0x58, 0x58),
        Color::Ansi256(241) => (0x62, 0x62, 0x62),
        Color::Ansi256(242) => (0x6c, 0x6c, 0x6c),
        Color::Ansi256(243) => (0x76, 0x76, 0x76),
        Color::Ansi256(244) => (0x80, 0x80, 0x80),
        Color::Ansi256(245) => (0x8a, 0x8a, 0x8a),
        Color::Ansi256(246) => (0x94, 0x94, 0x94),
        Color::Ansi256(247) => (0x9e, 0x9e, 0x9e),
        Color::Ansi256(248) => (0xa8, 0xa8, 0xa8),
        Color::Ansi256(249) => (0xb2, 0xb2, 0xb2),
        Color::Ansi256(250) => (0xbc, 0xbc, 0xbc),
        Color::Ansi256(251) => (0xc6, 0xc6, 0xc6),
        Color::Ansi256(252) => (0xd0, 0xd0, 0xd0),
        Color::Ansi256(253) => (0xda, 0xda, 0xda),
        Color::Ansi256(254) => (0xe4, 0xe4, 0xe4),
        Color::Ansi256(255) => (0xee, 0xee, 0xee),
        _ => unimplemented!(),
    }
}

pub struct SerializableColorSpec<'a> {
    fg: Option<&'a Color>,
    bg: Option<&'a Color>,
    bold: bool,
    underline: bool,
    intense: bool,
}

impl<'a> serde::Serialize for SerializableColorSpec<'a> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(4))?;
        for (key, value) in &[("fg", self.fg), ("bg", self.bg)] {
            match value {
                None => map.serialize_entry(key, &serde_json::Value::Null)?,
                Some(c) => {
                    let (r, g, b) = color2rgb(c);
                    map.serialize_entry(key, &format_args!("#{:02x}{:02x}{:02x}", r, g, b))?;
                }
            }
        }
        map.serialize_entry("bold", &self.bold)?;
        map.serialize_entry("underline", &self.underline)?;
        map.serialize_entry("intense", &self.intense)?;
        map.end()
    }
}

pub fn serializable_spec<'a>(spec: &'a ColorSpec) -> SerializableColorSpec<'a> {
    SerializableColorSpec {
        fg: spec.fg(),
        bg: spec.bg(),
        bold: spec.bold(),
        underline: spec.underline(),
        intense: spec.intense(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_spec_8bit_and_256bit() {
        let mut c = ColorSpec::new();
        c.set_fg(Some(Color::Ansi256(156)));
        c.set_bg(Some(Color::Magenta));
        c.set_bold(true);
        c.set_intense(true);
        let actual = serde_json::to_value(&serializable_spec(&c)).unwrap();
        let expect: serde_json::Value = serde_json::from_str(
            r##"{
                "fg": "#afff87",
                "bg": "#800080",
                "bold": true,
                "underline": false,
                "intense": true
            }"##,
        )
        .unwrap();
        assert_eq!(actual, expect);
    }

    #[test]
    fn serialize_spec_dark_256bit_and_rgb() {
        let mut c = ColorSpec::new();
        c.set_fg(Some(Color::Ansi256(232)));
        c.set_bg(Some(Color::Rgb(0x12, 0x34, 0x56)));
        c.set_underline(true);
        let actual = serde_json::to_value(&serializable_spec(&c)).unwrap();
        let expect: serde_json::Value = serde_json::from_str(
            r##"{
                "fg": "#080808",
                "bg": "#123456",
                "bold": false,
                "underline": true,
                "intense": false
            }"##,
        )
        .unwrap();
        assert_eq!(actual, expect);
    }

    #[test]
    fn serialize_spec_no_color() {
        let c = ColorSpec::new();
        let actual = serde_json::to_value(&serializable_spec(&c)).unwrap();
        let expect: serde_json::Value = serde_json::from_str(
            r##"{
                "fg": null,
                "bg": null,
                "bold": false,
                "underline": false,
                "intense": false
            }"##,
        )
        .unwrap();
        assert_eq!(actual, expect);
    }
}
