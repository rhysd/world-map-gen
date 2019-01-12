extern crate termcolor;

use self::termcolor::{BufferedStandardStream, ColorChoice, ColorSpec, WriteColor};
use crate::board::Board;
use crate::error::Result;
use std::collections::HashMap;
use std::io::Write;

pub fn draw_term(board: &Board, show_altitude: bool) -> Result<()> {
    let stdout = &mut BufferedStandardStream::stdout(ColorChoice::Always);
    let mut prev = ColorSpec::default();
    let mut legends = HashMap::new();
    for row in board.rows() {
        for cell in row.iter() {
            if prev != cell.color {
                stdout.set_color(&cell.color)?;
                prev = cell.color.clone();
            }
            if show_altitude {
                write!(stdout, "{:02}", cell.altitude)?;
            } else {
                write!(stdout, "{}", cell.char)?;
            }
            legends.entry(cell.kind.legend()).or_insert(cell);
        }
        writeln!(stdout)?; // new line
    }

    writeln!(stdout)?; // new line

    // Write legends
    for (legend, cell) in legends.iter() {
        stdout.set_color(&cell.color)?;
        write!(stdout, "  {}", cell.char)?;
        stdout.reset()?;
        writeln!(stdout, " : {}", legend)?;
    }

    Ok(())
}
