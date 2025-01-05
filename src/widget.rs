use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Direction, Layout, Rect},
	style::{Color, Style, Stylize},
	text::{Line, Text},
	widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Widget},
};

use crate::app::{App, FocusedWidget};

impl Widget for &App {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([
				Constraint::Length(3),  // Counter
				Constraint::Length(3),  // Checkbox
				Constraint::Length(3),  // Slider
				Constraint::Length(3),  // Input
				Constraint::Length(10), // List
				Constraint::Min(0),     // Tree
			])
			.split(area);

		self.render_counter(chunks[0], buf);
		self.render_checkbox(chunks[1], buf);
		self.render_slider(chunks[2], buf);
		self.render_input(chunks[3], buf);
		self.render_list_widget(chunks[4], buf);
		self.render_tree_widget(chunks[5], buf);
	}
}

impl App {
	fn render_list_widget(&self, area: Rect, buf: &mut Buffer) {
		let items: Vec<ListItem> = self
			.list_items
			.iter()
			.enumerate()
			.map(|(i, item)| {
				let style = if Some(i) == self.selected_item {
					Style::default().fg(Color::Yellow)
				} else {
					Style::default()
				};
				ListItem::new(item.as_str()).style(style)
			})
			.collect();

		let list_block = Block::default()
			.borders(Borders::ALL)
			.title(Line::from(" List ").centered())
			.border_style(Style::default().fg(self.focus_color(FocusedWidget::List)));

		List::new(items)
			.block(list_block)
			.highlight_style(Style::default().fg(Color::Yellow))
			.render(area, buf);
	}

	fn render_tree_widget(&self, area: Rect, buf: &mut Buffer) {
		let tree_block = Block::default()
			.borders(Borders::ALL)
			.title(Line::from(" Tree ").centered())
			.border_style(Style::default().fg(self.focus_color(FocusedWidget::Tree)));

		let tree_text = if self.tree_state {
			vec![
				Line::from("└── Root"),
				Line::from("    ├── Branch 1"),
				Line::from("    │   ├── Leaf 1.1"),
				Line::from("    │   └── Leaf 1.2"),
				Line::from("    └── Branch 2"),
				Line::from("        ├── Leaf 2.1"),
				Line::from("        └── Leaf 2.2"),
			]
		} else {
			vec![Line::from("└── Root")]
		};

		Paragraph::new(tree_text)
			.block(tree_block)
			.render(area, buf);
	}
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
