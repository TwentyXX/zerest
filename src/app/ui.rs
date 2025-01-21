use ratatui::{
	layout::{self, Constraint, Direction},
	text::Text,
	widgets::{Block, Paragraph},
	Frame,
};

use super::App;

impl App {
	pub(crate) fn draw(&self, frame: &mut Frame) {
		let chunks = layout::Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
			.split(frame.area());

		// 左側のカウンター
		frame.render_widget(self, chunks[0]);

		// 右側のlorem ipsum
		let word_block = Block::bordered().title(" Lorem Ipsum ");
		let word_text = Text::from(self.current_word.as_str());
		frame.render_widget(
			Paragraph::new(word_text)
				.block(word_block)
				.wrap(ratatui::widgets::Wrap { trim: true }),
			chunks[1],
		);
	}
}
