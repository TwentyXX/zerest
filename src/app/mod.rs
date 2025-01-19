mod event;
mod ui;
mod word;

use crate::message::ServerMessage;
use ratatui::DefaultTerminal;
use std::{io, time::Instant};
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct App {
	pub counter:                 u8,
	pub(crate) exit:             bool,
	pub(crate) current_word:     String,
	pub(crate) last_update:      Instant,
	pub(crate) checkbox_state:   bool,
	pub(crate) slider_value:     u8,
	pub(crate) input_text:       String,
	pub(crate) focused_widget:   FocusedWidget,
	pub(crate) list_items:       Vec<String>,
	pub(crate) selected_item:    Option<usize>,
	pub(crate) tree_state:       bool,
	pub(crate) message_receiver: Receiver<ServerMessage>,
	pub(crate) messages:         Vec<ServerMessage>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum FocusedWidget {
	Counter,
	Checkbox,
	Slider,
	Input,
	List,
	Tree,
}

impl App {
	pub fn new(message_receiver: Receiver<ServerMessage>) -> Self {
		Self {
			counter: 0,
			exit: false,
			current_word: String::from("Lorem"),
			last_update: Instant::now(),
			checkbox_state: false,
			slider_value: 50,
			input_text: String::new(),
			focused_widget: FocusedWidget::Counter,
			list_items: vec![
				"Item 1".to_string(),
				"Item 2".to_string(),
				"Item 3".to_string(),
			],
			selected_item: None,
			tree_state: false,
			message_receiver,
			messages: Vec::new(),
		}
	}
}

impl App {
	fn next_focus(&mut self) {
		self.focused_widget = match self.focused_widget {
			FocusedWidget::Counter => FocusedWidget::Checkbox,
			FocusedWidget::Checkbox => FocusedWidget::Slider,
			FocusedWidget::Slider => FocusedWidget::Input,
			FocusedWidget::Input => FocusedWidget::List,
			FocusedWidget::List => FocusedWidget::Tree,
			FocusedWidget::Tree => FocusedWidget::Counter,
		};
	}

	fn previous_focus(&mut self) {
		self.focused_widget = match self.focused_widget {
			FocusedWidget::Counter => FocusedWidget::Tree,
			FocusedWidget::Checkbox => FocusedWidget::Counter,
			FocusedWidget::Slider => FocusedWidget::Checkbox,
			FocusedWidget::Input => FocusedWidget::Slider,
			FocusedWidget::List => FocusedWidget::Input,
			FocusedWidget::Tree => FocusedWidget::List,
		};
	}
}

impl App {
	/// runs the application's main loop until the user quits
	pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
		while !self.exit {
			// メッセージの確認
			if let Ok(message) = self.message_receiver.try_recv() {
				self.current_word.push_str(&message.content);
				self.messages.push(message);
			}

			terminal.draw(|frame| self.draw(frame))?;
			self.handle_events()?;
		}
		Ok(())
	}

	pub(crate) fn exit(&mut self) { self.exit = true; }

	pub(crate) fn increment_counter(&mut self) { self.counter += 1; }

	pub(crate) fn decrement_counter(&mut self) { self.counter -= 1; }
}
