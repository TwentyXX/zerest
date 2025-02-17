use crate::{message::ServerMessage, server::ServerState};
use axum::{
	extract::{Json, State},
	http::HeaderMap,
};
use chrono::Utc;
use serde_json::Value;

/// メッセージを受け取るハンドラー
pub(crate) async fn handle_message_post(
	headers: HeaderMap,
	State(state): State<ServerState>,
	Json(payload): Json<String>,
) -> Json<String> {
	let json = serde_json::Map::from_iter(headers.iter().map(|(name, value)| {
		(
			name.to_string(),
			Value::String(value.to_str().unwrap_or_default().to_owned()),
		)
	}));
	let header_string = serde_json::to_string_pretty(&json).unwrap();
	let msg = ServerMessage {
		content:   header_string + "\n" + &payload,
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
