mod app;
mod message;
mod server;
mod widget;

use crate::app::App;
use chrono::Utc;
use server::MessageServer;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
	// チャネルの作成
	let (tx, rx) = tokio::sync::mpsc::channel(32);
	let t2 = tx.clone();

	// サーバーの設定
	let mut server = MessageServer::new(tx);

	// メッセージハンドラーの登録
	server.on_message(move |msg| {
		t2.send(message::ServerMessage {
			content:   "hello".to_string(),
			timestamp: Utc::now(),
		});
	});

	// サーバーを別タスクで起動
	tokio::spawn(async move {
		if let Err(e) = server.run().await {
			eprintln!("Server error: {}", e);
		}
	});

	// UIアプリケーションの起動
	let mut terminal = ratatui::init()?;
	// マウスキャプチャを有効化
	crossterm::execute!(
		std::io::stdout(),
		crossterm::event::EnableMouseCapture
	)?;
	let mut app = App::new(rx);
	let app_result = app.run(&mut terminal);
	// 終了時にマウスキャプチャを無効化
	crossterm::execute!(
		std::io::stdout(),
		crossterm::event::DisableMouseCapture
	)?;
	ratatui::restore();

	app_result
}

#[cfg(test)] mod tests;
