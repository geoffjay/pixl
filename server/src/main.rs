use std::sync::Arc;
use std::path::PathBuf;

use poem::{
    get, handler,
    listener::TcpListener,
    web::Json,
    Route, Server, EndpointExt, 
};
use tokio::sync::RwLock;
use tracing_subscriber;

mod api;
mod models;
mod services;
mod utils;

use services::{FileService, EventService};
use api::{path, books, events};

#[handler]
fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "pixl-server"
    }))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Initialize logging
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "debug");
        }
    }
    tracing_subscriber::fmt::init();

    // Initialize services
    let default_path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let file_service = Arc::new(RwLock::new(FileService::new(default_path)));
    let event_service = Arc::new(RwLock::new(EventService::new()));

    // Build routes
    let app = Route::new()
        .at("/", get(health_check))
        .at("/path", get(path::get_path).put(path::set_path))
        .at("/books", get(books::list_books).post(books::create_book))
        .at("/books/:filename", get(books::get_book).put(books::update_book))
        .at("/books/:filename/events", get(events::pixel_book_events))
        .data(file_service)
        .data(event_service);

    // Start server
    let listener = TcpListener::bind("0.0.0.0:3000");
    println!("PIXL Server starting on http://0.0.0.0:3000");
    
    Server::new(listener)
        .run(app)
        .await
}