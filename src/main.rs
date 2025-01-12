mod app;
mod server;
mod widget;

use crate::app::App;
use server::run_server;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
	let mut terminal = ratatui::init();
	let app_result = App::default().run(&mut terminal);
	ratatui::restore();
	run_server().await.unwrap();
	app_result
}

#[cfg(test)] mod tests;
