use crate::message::ServerMessage;
use axum::{self, http::Method, routing::get, Router};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct ServerState(Arc<Mutex<MessageServer>>);

impl ServerState {
	pub fn new(server: MessageServer) -> Self { Self(Arc::new(Mutex::new(server))) }
}
use tokio::{net::TcpListener, sync::mpsc};

mod handlers;

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
		// CORSの設定
		let cors = CorsLayer::new()
			.allow_origin(tower_http::cors::Any)
			.allow_methods([Method::GET, Method::POST, Method::CONNECT, Method::OPTIONS])
			.allow_headers(tower_http::cors::Any);

		// ルーターの設定にCORSレイヤーを追加
		let app = Router::new()
			.route(
				"/",
				get(handlers::get::handle_message_get).post(handlers::post::handle_message_post),
			)
			.layer(cors)
			.with_state(state);

		// サーバーのアドレスを設定
		let addr = "0.0.0.0:3000";
		println!("Server running on http://{}", addr);

		// リスナーを作成してサーバーを起動
		let listener = TcpListener::bind(addr).await?;
		axum::serve(listener, app).await?;

		Ok(())
	}
}
