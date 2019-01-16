//! Helper to draw a generated map to terminal screen or as JSON.
//!
//! Terminal must support 256colors. And large map may require much time and CPU usage to render map.

extern crate termcolor;

use self::termcolor::{BufferedStandardStream, ColorChoice, ColorSpec, WriteColor};
use crate::board::Board;
use crate::error::Result;
use std::collections::HashMap;
use std::io::Write;

/// Render the given board to terminal screen. When the `show_altitude` flag is set to true, it
/// renders the altitude value for each cell instead of each cell's characters. This flag is
/// usually enabled for debugging purpose.
/// When writing to terminal fails, it returns an error.
///
/// ```rust
/// use world_map_gen::gen::RandomBoardGen;
/// use world_map_gen::draw::draw_term;
///
/// let mut gen = RandomBoardGen::default();
/// let board = gen.gen_auto(3, 4);
///
/// draw_term(&board, true).unwrap();
/// ```
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
        writeln!(stdout)?;
    }

    // Write legends
    let term_width = board.width * 2; // since a cell consists of half-width character * 2
    let mut width = usize::max_value();
    for (legend, cell) in legends.iter() {
        let legend_len = cell.char.chars().count() + 3 + legend.len();
        if width.saturating_add(legend_len) > term_width {
            write!(stdout, "\n  ")?;
            width = 2 + legend_len;
        } else {
            write!(stdout, ", ")?;
            width += 2 + legend_len;
        }
        stdout.set_color(&cell.color)?;
        write!(stdout, "{}", cell.char)?;
        stdout.reset()?;
        write!(stdout, " : {}", legend)?;
    }
    writeln!(stdout)?;

    Ok(())
}
