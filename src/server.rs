use crate::message::ServerMessage;
use axum::{
	debug_handler, extract::{Json, State}, routing::get, Router
};
use chrono::Utc;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ServerState(Arc<Mutex<MessageServer>>);

impl ServerState {
	pub fn new(server: MessageServer) -> Self { Self(Arc::new(Mutex::new(server))) }
}
use tokio::{net::TcpListener, sync::mpsc};

/// メッセージを受け取るハンドラー
async fn handle_message(
    State(state): State<ServerState>,
    Json(payload): Json<String>,
) -> Json<String> {
    let msg = ServerMessage {
        content: payload,
        timestamp: Utc::now(),
    };

    // MutexGuardのスコープを最小限に
    let tx = {
        let mut server = state.0.lock().unwrap();
        // ハンドラーを実行
        for handler in &mut server.handlers {
            handler(&msg);
        }
        server.tx.clone()
    };

    // ロックを解放した後でメッセージを送信
    match tx.send(msg).await {
        Ok(_) => Json("Message received".to_string()),
        Err(_) => Json("Server Error".to_string()),
    }
}

pub struct MessageServer {
	tx:       mpsc::Sender<ServerMessage>,
	handlers: Vec<Box<dyn FnMut(&ServerMessage) + Send + Sync>>,
}

impl MessageServer {
	pub fn new(tx: mpsc::Sender<ServerMessage>) -> Self {
		Self {
			tx,
			handlers: Vec::new(),
		}
	}

	pub fn on_message<F>(&mut self, handler: F)
	where
		F: FnMut(&ServerMessage) + Send + Sync + 'static, {
		self.handlers.push(Box::new(handler));
	}


	pub async fn run(self) -> color_eyre::Result<()> {
		let state = ServerState::new(self);

		// ルーターの設定
		let app = Router::new()
			.route("/message", get(handle_message).post(handle_message))
			.with_state(state);

		// サーバーのアドレスを設定
		let addr = "127.0.0.1:3000";
		println!("Server running on http://{}", addr);

		// リスナーを作成してサーバーを起動
		let listener = TcpListener::bind(addr).await?;
		axum::serve(listener, app).await?;

		Ok(())
	}
}
