use std::error::Error;

mod app;
mod rendering;
mod models;
mod services;
mod utils;

use app::Viewer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize logging
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    println!("Starting PIXL Viewer...");

    let mut viewer = Viewer::new()?;
    
    // For demo purposes in Phase 1, try to load a demo book if available
    if let Err(e) = viewer.load_demo_book().await {
        println!("Could not load demo book: {}", e);
    }
    
    viewer.run().await?;
    
    println!("PIXL Viewer shutting down.");
    Ok(())
}
