use std::{io, time::{Duration, Instant}};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use rand::seq::SliceRandom;
use ratatui::{
    Frame,
    layout::{self, Constraint, Direction},
    widgets::{Block, Paragraph},
    text::Text,
    DefaultTerminal,
};

#[derive(Debug)]
pub struct App {
    pub(crate) counter: u8,
    pub(crate) exit: bool,
    pub(crate) current_word: String,
    pub(crate) last_update: Instant,
}

impl Default for App {
    fn default() -> Self {
        Self {
            counter: 0,
            exit: false,
            current_word: String::from("Lorem"),
            last_update: Instant::now(),
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

    pub(crate) fn draw(&self, frame: &mut Frame) {
        let chunks = layout::Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(frame.size());

        // 左側のカウンター
        frame.render_widget(self, chunks[0]);

        // 右側のlorem ipsum
        let word_block = Block::bordered().title(" Lorem Ipsum ");
        let word_text = Text::from(self.current_word.as_str());
        frame.render_widget(
            Paragraph::new(word_text).block(word_block).centered(),
            chunks[1],
        );
    }

    /// updates the application's state based on user input
    pub(crate) fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            }
        }

        // 1秒ごとに単語を更新
        if self.last_update.elapsed() >= Duration::from_millis(100) {
            self.update_word();
            self.last_update = Instant::now();
        }
        Ok(())
    }

    pub(crate) fn update_word(&mut self) {
        const LOREM_WORDS: &[&str] = &[
            "Lorem", "ipsum", "dolor", "sit", "amet", "consectetur",
            "adipiscing", "elit", "sed", "do", "eiusmod", "tempor",
            "incididunt", "ut", "labore", "et", "dolore", "magna", "aliqua",
        ];
        
        if let Some(word) = LOREM_WORDS.choose(&mut rand::thread_rng()) {
            self.current_word += word;
            self.current_word += " ";
        }
    }

    pub(crate) fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
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
