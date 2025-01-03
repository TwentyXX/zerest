use std::{io, time::{Duration, Instant}};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use rand::seq::SliceRandom;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
pub struct App {
    counter: u8,
    exit: bool,
    current_word: String,
    last_update: Instant,
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

    fn draw(&self, frame: &mut Frame) {
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                ratatui::layout::Constraint::Percentage(50),
                ratatui::layout::Constraint::Percentage(50),
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
    fn handle_events(&mut self) -> io::Result<()> {
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

    fn update_word(&mut self) {
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

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests;
