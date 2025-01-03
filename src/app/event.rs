use std::time::Instant;
use std::{io, time::Duration};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use super::App;

impl App {
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

    pub(crate) fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            KeyCode::Char(' ') => self.checkbox_state = !self.checkbox_state,
            KeyCode::Tab => self.next_focus(),
            KeyCode::Up => {
                match self.focused_widget {
                    FocusedWidget::Slider => self.slider_value = self.slider_value.saturating_add(5).min(100),
                    _ => self.previous_focus(),
                }
            },
            KeyCode::Down => {
                match self.focused_widget {
                    FocusedWidget::Slider => self.slider_value = self.slider_value.saturating_sub(5),
                    _ => self.next_focus(),
                }
            },
            KeyCode::Char(c) => self.input_text.push(c),
            KeyCode::Backspace => { self.input_text.pop(); }
            _ => {}
        }
    }
}
