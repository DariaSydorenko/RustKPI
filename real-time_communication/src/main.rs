use actix_web::{web, App, HttpServer, middleware::Logger}; 
use actix_session::CookieSession;
use std::sync::Arc;
use crate::websocket::WsManager;

mod handlers;
mod websocket;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let ws_manager = Arc::new(WsManager::new());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(CookieSession::private(&[0; 32]))
            .app_data(web::Data::new(ws_manager.clone()))
            .route("/", web::get().to(handlers::index))
            .route("/login", web::get().to(handlers::login_page))
            .route("/register", web::get().to(handlers::register_page))
            .route("/login", web::post().to(handlers::login))
            .route("/register", web::post().to(handlers::register))
            .route("/chat", web::get().to(handlers::chat))
            .route("/send_message", web::post().to(handlers::send_message))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}