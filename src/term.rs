use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::{
    io::{self, Write},
    time::Duration,
};

pub struct TermCoords(pub u16, pub u16);

pub struct Terminal {
    pub stdout: io::Stdout,
    pub rows: u16,
    pub cols: u16,
    pub ch: char,
}

impl Terminal {
    pub fn new(ch: char) -> Terminal {
        let size = crossterm::terminal::size();
        let mut stdout = io::stdout();
        let mut cols = 100;
        let mut rows = 200;
        let _ = stdout.queue(cursor::Hide);
        match size {
            Ok(size) => {
                cols = size.0;
                rows = size.1;
            }
            _ => (),
        }
        Terminal {
            stdout,
            rows,
            cols,
            ch,
        }
    }

    pub fn clear(&mut self) {
        let _ = self
            .stdout
            .execute(terminal::Clear(terminal::ClearType::All));
    }

    pub fn draw_at(&mut self, x: u16, y: u16) -> io::Result<()> {
        let _ = self
            .stdout
            .queue(cursor::MoveTo(x, y))?
            .queue(style::PrintStyledContent(self.ch.green()));

        Ok(())
    }

    pub fn draw_from_coords(&mut self, coords: Vec<TermCoords>) {
        for coord in coords.iter() {
            let _ = self.draw_at(coord.0, coord.1);
        }
    }

    pub fn flush(&mut self) {
        let _ = self.stdout.flush();
    }
}

pub enum TermState {
    Continue,
    Quit,
}

pub fn handle_events() -> io::Result<TermState> {
    if poll(Duration::from_millis(500))? {
        let event = read()?;

        if event == Event::Key(KeyCode::Char('q').into()) {
            return Ok(TermState::Quit);
        }
    }

    return Ok(TermState::Continue);
}
