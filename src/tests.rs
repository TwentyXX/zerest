use super::*;
use crossterm::event::KeyCode;
use ratatui::{
	buffer::Buffer,
	layout::Rect,
	style::{Style, Stylize as _},
	widgets::Widget as _,
};
use tokio::sync::mpsc;

#[test]
fn handle_key_event() -> io::Result<()> {
	let (_, rx) = mpsc::channel(1);
	let mut app = App::new(rx);
	app.handle_key_event(KeyCode::Right.into());
	assert_eq!(app.counter, 1);

	app.handle_key_event(KeyCode::Left.into());
	assert_eq!(app.counter, 0);

	let (_, rx) = mpsc::channel(1);
	let mut app = App::new(rx);
	app.handle_key_event(KeyCode::Char('q').into());
	assert!(app.exit);

	Ok(())
}
