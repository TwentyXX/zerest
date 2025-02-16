use chrono::Utc;

use crate::message::ServerMessage;

use serde_json::Value;

use axum::extract::Json;

use crate::server::ServerState;

use axum::extract::State;

use axum::http::HeaderMap;

/// メッセージを受け取るハンドラー
pub(crate) async fn handle_message_get(
	headers: HeaderMap,
	State(state): State<ServerState>,
) -> Json<String> {
	let json = serde_json::Map::from_iter(headers.iter().map(|(name, value)| {
		(
			name.to_string(),
			Value::String(value.to_str().unwrap_or_default().to_owned()),
		)
	}));

	let header_string = serde_json::to_string_pretty(&json).unwrap();
	let msg = ServerMessage {
		content:   header_string,
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
