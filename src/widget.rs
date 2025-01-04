use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Direction, Layout, Rect},
	style::{Color, Style, Stylize},
	symbols::border,
	text::{Line, Text},
	widgets::{Block, Borders, Gauge, Paragraph, Widget},
};

use crate::app::{App, FocusedWidget};

impl Widget for &App {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([
				Constraint::Length(3), // Counter
				Constraint::Length(3), // Checkbox
				Constraint::Length(3), // Slider
				Constraint::Length(3), // Input
				Constraint::Min(0),    // Remaining space
			])
			.split(area);

		self.render_counter(chunks[0], buf);
		self.render_checkbox(chunks[1], buf);
		self.render_slider(chunks[2], buf);
		self.render_input(chunks[3], buf);
	}
}

impl App {
	fn render_counter(&self, area: Rect, buf: &mut Buffer) {
		let counter_text = Text::from(vec![Line::from(vec![
			"Counter: ".into(),
			self.counter.to_string().yellow(),
		])]);

		let counter_block = Block::default()
			.borders(Borders::ALL)
			.title(Line::from(" Counter ").centered())
			.border_style(Style::default().fg(self.focus_color(FocusedWidget::Counter)));

		Paragraph::new(counter_text)
			.block(counter_block)
			.render(area, buf);
	}

	fn render_checkbox(&self, area: Rect, buf: &mut Buffer) {
		let checkbox_text = format!(
			"[{}] Checkbox Option",
			if self.checkbox_state { "x" } else { " " }
		);

		let checkbox_block = Block::default()
			.borders(Borders::ALL)
			.title(Line::from(" Checkbox ").centered())
			.border_style(Style::default().fg(self.focus_color(FocusedWidget::Checkbox)));

		Paragraph::new(checkbox_text)
			.block(checkbox_block)
			.render(area, buf);
	}

	fn render_slider(&self, area: Rect, buf: &mut Buffer) {
		let slider_block = Block::default()
			.borders(Borders::ALL)
			.title(Line::from(" Slider ").centered())
			.border_style(Style::default().fg(self.focus_color(FocusedWidget::Slider)));

		Gauge::default()
			.block(slider_block)
			.gauge_style(Style::default().fg(Color::Yellow))
			.ratio(f64::from(self.slider_value) / 100.0)
			.label(format!("{}%", self.slider_value))
			.render(area, buf);
	}

	fn render_input(&self, area: Rect, buf: &mut Buffer) {
		let input_text = if self.input_text.is_empty() {
			"<type here>".dim().to_string()
		} else {
			self.input_text.clone()
		};

		let input_block = Block::default()
			.borders(Borders::ALL)
			.title(Line::from(" Input ").centered())
			.border_style(Style::default().fg(self.focus_color(FocusedWidget::Input)));

		Paragraph::new(input_text)
			.block(input_block)
			.render(area, buf);
	}

	fn focus_color(&self, widget: FocusedWidget) -> Color {
		if self.focused_widget == widget {
			Color::Cyan
		} else {
			Color::White
		}
	}
}
