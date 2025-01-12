use axum::{
    routing::get,
    Router,
    extract::Json,
};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use crate::message::ServerMessage;
use chrono::Utc;

/// メッセージを受け取るハンドラー
async fn handle_message(
    Json(payload): Json<String>,
    tx: axum::extract::State<mpsc::Sender<ServerMessage>>,
) -> &'static str {
    let msg = ServerMessage {
        content: payload,
        timestamp: Utc::now(),
    };
    
    if let Err(e) = tx.send(msg).await {
        eprintln!("Failed to send message: {}", e);
        return "Error processing message";
    }
    
    "Message received"
}

/// サーバーを起動する関数
pub async fn run_server(tx: mpsc::Sender<ServerMessage>) -> color_eyre::Result<()> {
    // ルーターの設定
    let app = Router::new()
        .route("/message", get(handle_message))
        .with_state(tx);

    // サーバーのアドレスを設定
    let addr = "127.0.0.1:3000";
    println!("Server running on http://{}", addr);

    // リスナーを作成してサーバーを起動
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
