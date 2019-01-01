extern crate termcolor;

use self::termcolor::WriteColor;
use crate::board::Board;
use crate::error::Result;

struct TermDrawer<W: WriteColor> {
    stdout: W,
}

impl<W: WriteColor> TermDrawer<W> {
    fn draw(&mut self, board: &Board) -> Result<()> {
        for row in board.rows() {
            for cell in row.cols() {
                self.stdout.set_color(&cell.color)?;
                write!(&mut self.stdout, "{}", cell.char)?;
            }
            writeln!(&mut self.stdout, "")?;
        }
        Ok(())
    }
}

pub fn draw_term(board: &Board) -> Result<()> {
    let mut drawer = TermDrawer {
        stdout: termcolor::StandardStream::stdout(termcolor::ColorChoice::Always),
    };
    drawer.draw(board)
}
