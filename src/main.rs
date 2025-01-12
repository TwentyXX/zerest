mod app;
mod server;
mod widget;
mod message;

use crate::app::App;
use server::run_server;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // チャネルの作成
    let (tx, rx) = tokio::sync::mpsc::channel(32);
    
    // サーバーの設定
    let mut server = MessageServer::new(tx);
    
    // メッセージハンドラーの登録
    server.on_message(|msg| {
        println!("Received message: {:?}", msg);
    });
    
    // サーバーを別タスクで起動
    tokio::spawn(async move {
        if let Err(e) = server.run().await {
            eprintln!("Server error: {}", e);
        }
    });

    // UIアプリケーションの起動
    let mut terminal = ratatui::init();
    let mut app = App::new(rx);
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    
    app_result
}

#[cfg(test)] mod tests;
