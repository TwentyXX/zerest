use ratatui::{
    buffer::Buffer,
    layout::{Rect, Layout, Direction, Constraint},
    style::{Stylize, Style, Color},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Borders, Gauge, List, ListItem},
};

use crate::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(" Control Panel ".bold())
            .border_set(border::THICK);

        // Create a layout for the widgets
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Counter
                Constraint::Length(3),  // Checkbox
                Constraint::Length(3),  // Slider
                Constraint::Length(3),  // Input
                Constraint::Min(0),     // Remaining space
            ])
            .split(area);

        // Render the main block
        block.render(area, buf);

        // Counter
        let counter_text = Text::from(vec![Line::from(vec![
            "Counter: ".into(),
            self.counter.to_string().yellow(),
        ])]);
        let counter_block = Block::default()
            .borders(Borders::ALL)
            .title(" Counter ")
            .border_style(Style::default().fg(if self.focused_widget == FocusedWidget::Counter {
                Color::Cyan
            } else {
                Color::White
            }));
        Paragraph::new(counter_text)
            .block(counter_block)
            .render(chunks[0], buf);

        // Checkbox
        let checkbox_text = format!("[{}] Checkbox Option", if self.checkbox_state { "x" } else { " " });
        let checkbox_block = Block::default()
            .borders(Borders::ALL)
            .title(" Checkbox ")
            .border_style(Style::default().fg(if self.focused_widget == FocusedWidget::Checkbox {
                Color::Cyan
            } else {
                Color::White
            }));
        Paragraph::new(checkbox_text)
            .block(checkbox_block)
            .render(chunks[1], buf);

        // Slider/Gauge
        let slider_block = Block::default()
            .borders(Borders::ALL)
            .title(" Slider ")
            .border_style(Style::default().fg(if self.focused_widget == FocusedWidget::Slider {
                Color::Cyan
            } else {
                Color::White
            }));
        Gauge::default()
            .block(slider_block)
            .gauge_style(Style::default().fg(Color::Yellow))
            .ratio(f64::from(self.slider_value) / 100.0)
            .label(format!("{}%", self.slider_value))
            .render(chunks[2], buf);

        // Text Input
        let input_text = if self.input_text.is_empty() {
            "<type here>".dim().to_string()
        } else {
            self.input_text.clone()
        };
        let input_block = Block::default()
            .borders(Borders::ALL)
            .title(" Input ")
            .border_style(Style::default().fg(if self.focused_widget == FocusedWidget::Input {
                Color::Cyan
            } else {
                Color::White
            }));
        Paragraph::new(input_text)
            .block(input_block)
            .render(chunks[3], buf);
    }
}
