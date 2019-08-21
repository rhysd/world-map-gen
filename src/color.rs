use serde;
use serde_json;
use termcolor;
use termcolor::ColorSpec;
use termcolor2rgb;

pub struct SerializableColorSpec<'a>(pub &'a ColorSpec);

impl<'a> serde::Serialize for SerializableColorSpec<'a> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        use termcolor2rgb::ColorExt;

        let mut map = serializer.serialize_map(Some(4))?;

        for (key, value) in &[("fg", self.0.fg()), ("bg", self.0.bg())] {
            match value {
                None => map.serialize_entry(key, &serde_json::Value::Null)?,
                Some(c) => {
                    let (r, g, b) = c.to_rgb();
                    map.serialize_entry(key, &format_args!("#{:02x}{:02x}{:02x}", r, g, b))?;
                }
            }
        }
        map.serialize_entry("bold", &self.0.bold())?;
        map.serialize_entry("underline", &self.0.underline())?;
        map.serialize_entry("intense", &self.0.intense())?;

        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use termcolor::Color;

    #[test]
    fn serialize_spec_8bit_and_256bit() {
        let mut c = ColorSpec::new();
        c.set_fg(Some(Color::Ansi256(156)));
        c.set_bg(Some(Color::Magenta));
        c.set_bold(true);
        c.set_intense(true);
        let actual = serde_json::to_value(SerializableColorSpec(&c)).unwrap();
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
        let actual = serde_json::to_value(SerializableColorSpec(&c)).unwrap();
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
        let actual = serde_json::to_value(SerializableColorSpec(&c)).unwrap();
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
