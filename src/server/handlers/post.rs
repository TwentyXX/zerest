use chrono::Utc;

use crate::{message::ServerMessage, server::ServerState};

use axum::extract::Json;

use axum::extract::State;

/// メッセージを受け取るハンドラー
pub(crate) async fn handle_message_post(
	State(state): State<ServerState>,
	Json(payload): Json<String>,
) -> Json<String> {
	let msg = ServerMessage {
		content:   payload,
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
