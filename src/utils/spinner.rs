use anyhow::Result;
use crossterm::{cursor, queue, style, terminal};
use std::io::{Stdout, Write};

pub struct Spinner {
    index: usize,
    message: String,
    stopped: bool,
}

impl Spinner {
    const DATA: [&'static str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

    pub fn new(message: &str) -> Self {
        Spinner {
            index: 0,
            message: message.to_string(),
            stopped: false,
        }
    }

    pub fn step(&mut self, writer: &mut Stdout) -> Result<()> {
        if self.stopped {
            return Ok(());
        }
        let frame = Self::DATA[self.index % Self::DATA.len()];
        let dots = ".".repeat((self.index / 5) % 4);
        let line = format!("{frame}{}{:<3}", self.message, dots);
        queue!(writer, cursor::MoveToColumn(0), style::Print(line),)?;
        if self.index == 0 {
            queue!(writer, cursor::Hide)?;
        }
        writer.flush()?;
        self.index += 1;
        Ok(())
    }

    pub fn stop(&mut self, writer: &mut Stdout) -> Result<()> {
        if self.stopped {
            return Ok(());
        }
        self.stopped = true;
        queue!(
            writer,
            cursor::MoveToColumn(0),
            terminal::Clear(terminal::ClearType::FromCursorDown),
            cursor::Show
        )?;
        writer.flush()?;
        Ok(())
    }
}
