mod ui;
mod event;
mod word;

use std::io;
use std::time::Instant;
use ratatui::DefaultTerminal;

#[derive(Debug)]
pub struct App {
    pub counter: u8,
    pub(crate) exit: bool,
    pub(crate) current_word: String,
    pub(crate) last_update: Instant,
    pub(crate) checkbox_state: bool,
    pub(crate) slider_value: u8,
    pub(crate) input_text: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            counter: 0,
            exit: false,
            current_word: String::from("Lorem"),
            last_update: Instant::now(),
            checkbox_state: false,
            slider_value: 50,
            input_text: String::new(),
        }
    }
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub(crate) fn exit(&mut self) {
        self.exit = true;
    }

    pub(crate) fn increment_counter(&mut self) {
        self.counter += 1;
    }

    pub(crate) fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}
