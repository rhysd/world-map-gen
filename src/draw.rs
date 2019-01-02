extern crate termcolor;

use self::termcolor::WriteColor;
use crate::board::Board;
use crate::error::Result;

struct TermDrawer<W: WriteColor> {
    stdout: W,
    show_altitude: bool,
}

impl<W: WriteColor> TermDrawer<W> {
    fn draw(&mut self, board: &Board) -> Result<()> {
        for row in board.rows() {
            for cell in row.cols() {
                self.stdout.set_color(&cell.color)?;
                if self.show_altitude {
                    write!(&mut self.stdout, "{:02}", cell.altitude)?;
                } else {
                    write!(&mut self.stdout, "{}", cell.char)?;
                }
            }
            writeln!(&mut self.stdout, "")?;
        }
        Ok(())
    }
}

pub fn draw_term(board: &Board, show_altitude: bool) -> Result<()> {
    let mut drawer = TermDrawer {
        stdout: termcolor::StandardStream::stdout(termcolor::ColorChoice::Always),
        show_altitude,
    };
    drawer.draw(board)
}
