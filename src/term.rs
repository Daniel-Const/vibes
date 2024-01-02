use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::io::{self, Write};

pub struct Terminal {
    pub stdout: io::Stdout,
    pub rows: u16,
    pub cols: u16,
}

impl Terminal {
    pub fn new() -> Terminal {
        let stdout = io::stdout();
        let size = crossterm::terminal::size();
        let mut cols = 100;
        let mut rows = 200;
        match size {
            Ok(size) => {
                cols = size.0;
                rows = size.1;
            }
            _ => (),
        }
        Terminal { stdout, rows, cols }
    }

    pub fn clear(&mut self) {
        let _ = self
            .stdout
            .execute(terminal::Clear(terminal::ClearType::All));
    }

    pub fn draw_at(&mut self, x: u16, y: u16, ch: char) -> io::Result<()> {
        let _ = self
            .stdout
            .queue(cursor::MoveTo(x, y))?
            .queue(style::PrintStyledContent(ch.green()));

        Ok(())
    }

    pub fn flush(&mut self) {
        let _ = self.stdout.queue(cursor::Hide);
        let _ = self.stdout.flush();
    }
}
