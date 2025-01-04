use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::{
	io,
	time::{Duration, Instant},
};

use super::{App, FocusedWidget};

macro_rules! guard {
	($value:expr, $($pattern:pat => $result:expr $(,)?)*) => {
		match $value {
			$($pattern => $result,)*
			_ => {}
		}
	};
}

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
		let key_code = key_event.code;
		guard!(key_code,
			KeyCode::Up => self.previous_focus()
			KeyCode::Down => self.next_focus()
		);
		match self.focused_widget {
			FocusedWidget::Counter => match key_code {
				KeyCode::Left => self.decrement_counter(),
				KeyCode::Right => self.increment_counter(),
				KeyCode::Char('q') => self.exit(),
				KeyCode::Char(' ') => self.checkbox_state = !self.checkbox_state,
				KeyCode::Tab => self.next_focus(),
				KeyCode::Char(c) => self.input_text.push(c),
				KeyCode::Backspace => {
					self.input_text.pop();
				}
				_ => {}
			},
			FocusedWidget::Checkbox => match key_code {
				KeyCode::Char(' ') => self.checkbox_state = !self.checkbox_state,
				_ => {}
			},
			FocusedWidget::Slider => match key_code {
				KeyCode::Left => self.slider_value = self.slider_value.saturating_sub(5),
				KeyCode::Right => self.slider_value = self.slider_value.saturating_add(5).min(100),
				_ => {}
			},
			FocusedWidget::Input => match key_code {
				KeyCode::Char(c) => self.input_text.push(c),
				KeyCode::Backspace => {
					self.input_text.pop();
				}
				_ => {}
			},
		}
	}
}
