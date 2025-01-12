use crate::message::ServerMessage;
use axum::{
    extract::{Json, State},
    routing::get,
    Router,
};
use chrono::Utc;
use tokio::{net::TcpListener, sync::mpsc};
use std::sync::Arc;

/// メッセージを受け取るハンドラー
async fn handle_message(
	State(tx): State<mpsc::Sender<ServerMessage>>,
	Json(payload): Json<String>,
) -> &'static str {
	let msg = ServerMessage {
		content:   payload,
		timestamp: Utc::now(),
	};

	if let Err(e) = tx.send(msg).await {
		eprintln!("Failed to send message: {}", e);
		return "Error processing message";
	}

	"Message received"
}

/// サーバーを起動する関数
pub async fn run_server(
    tx: mpsc::Sender<ServerMessage>,
    callback: impl Fn(&ServerMessage) + Send + 'static,
) -> color_eyre::Result<()> {
    // コールバック付きのStateを作成
    #[derive(Clone)]
    struct ServerState {
        tx: mpsc::Sender<ServerMessage>,
        callback: Arc<dyn Fn(&ServerMessage) + Send + Sync>,
    }

    let state = ServerState {
        tx,
        callback: Arc::new(callback),
    };

    // ルーターの設定
    let app = Router::new()
        .route("/message", get(
            |State(state): State<ServerState>, Json(payload): Json<String>| async move {
                let msg = ServerMessage {
                    content: payload,
                    timestamp: Utc::now(),
                };
                
                // コールバックを実行
                (state.callback)(&msg);
                
                // メッセージを送信
                if let Err(e) = state.tx.send(msg).await {
                    eprintln!("Failed to send message: {}", e);
                    return "Error processing message";
                }
                
                "Message received"
            }
        ))
        .with_state(state);

    // サーバーのアドレスを設定
    let addr = "127.0.0.1:3000";
    println!("Server running on http://{}", addr);

    // リスナーを作成してサーバーを起動
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
