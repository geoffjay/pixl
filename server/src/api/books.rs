use crate::models::{PixelBook, PixelBookInfo, CreatePixelBookRequest, UpdatePixelBookRequest, PixelError};
use crate::services::{FileService, DrawingService, EventService};
use crate::utils::validation;
use poem::{handler, web::{Json, Path}, Result, Error, http::StatusCode};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(serde::Serialize)]
struct BooksResponse {
    books: Vec<PixelBookInfo>,
}

#[handler]
pub async fn list_books(
    file_service: poem::web::Data<&Arc<RwLock<FileService>>>,
) -> Result<Json<BooksResponse>> {
    let service = file_service.read().await;
    let books = service.list_books()
        .map_err(|e| Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;
    
    Ok(Json(BooksResponse { books }))
}

#[handler]
pub async fn get_book(
    file_service: poem::web::Data<&Arc<RwLock<FileService>>>,
    filename: Path<String>,
) -> Result<Json<PixelBook>> {
    let service = file_service.read().await;
    
    if !validation::validate_filename(&filename) {
        return Err(Error::from_string(
            "Invalid filename",
            poem::http::StatusCode::BAD_REQUEST,
        ));
    }
    
    let book = service.load_book(&filename)
        .map_err(|e| match e {
            crate::models::PixelError::FileNotFound { .. } => 
                Error::from_string(e.to_string(), poem::http::StatusCode::NOT_FOUND),
            _ => Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR),
        })?;
    
    Ok(Json(book))
}

#[handler]
pub async fn create_book(
    file_service: poem::web::Data<&Arc<RwLock<FileService>>>,
    request: Json<CreatePixelBookRequest>,
) -> Result<Json<serde_json::Value>> {
    if !validation::validate_filename(&request.filename) {
        return Err(Error::from_string(
            "Invalid filename",
            poem::http::StatusCode::BAD_REQUEST,
        ));
    }
    
    if !validation::validate_dimensions(request.width, request.height) {
        return Err(Error::from_string(
            "Invalid dimensions",
            poem::http::StatusCode::BAD_REQUEST,
        ));
    }
    
    if request.frames == 0 || request.frames > 1000 {
        return Err(Error::from_string(
            "Frame count must be between 1 and 1000",
            poem::http::StatusCode::BAD_REQUEST,
        ));
    }
    
    let service = file_service.read().await;
    let book = service.create_book(&request.filename, request.width, request.height, request.frames)
        .map_err(|e| Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR))?;
    
    let full_path = service.get_path().join(&request.filename);
    
    Ok(Json(json!({
        "success": true,
        "filename": book.filename,
        "path": full_path.to_string_lossy()
    })))
}

#[handler]
pub async fn update_book(
    file_service: poem::web::Data<&Arc<RwLock<FileService>>>,
    event_service: poem::web::Data<&Arc<RwLock<EventService>>>,
    filename: Path<String>,
    request: Json<UpdatePixelBookRequest>,
) -> Result<Json<serde_json::Value>> {
    println!("🚨 UPDATE_BOOK called for: {} with {} operations", filename.as_str(), request.operations.len());
    
    if !validation::validate_filename(&filename) {
        return Err(Error::from_string(
            "Invalid filename",
            poem::http::StatusCode::BAD_REQUEST,
        ));
    }

    let mut service = file_service.write().await;
    
    // Load the pixel book
    let mut book = service.load_book(&filename)
        .map_err(|e| match e {
            crate::models::PixelError::FileNotFound { .. } => 
                Error::from_string(e.to_string(), poem::http::StatusCode::NOT_FOUND),
            _ => Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR),
        })?;

    // Apply drawing operations
    println!("🎨 Applying {} drawing operations...", request.operations.len());
    let drawing_service = DrawingService::new();
    drawing_service.apply_operations(&mut book, request.operations.clone())
        .map_err(|e| {
            println!("❌ Drawing operation failed: {}", e);
            Error::from_string(e.to_string(), poem::http::StatusCode::BAD_REQUEST)
        })?;

    // Save the updated book
    println!("💾 Saving pixel book to disk...");
    service.save_book(&book)
        .map_err(|e| {
            println!("❌ Save failed: {}", e);
            Error::from_string(e.to_string(), poem::http::StatusCode::INTERNAL_SERVER_ERROR)
        })?;
    println!("✅ Book saved successfully!");

    // Emit events for each drawing operation
    let event_svc = event_service.read().await;
    for operation in &request.operations {
        println!("🎨 Emitting drawing operation event for: {}", filename.as_str());
        event_svc.on_drawing_operation(&filename, operation.clone()).await;
    }
    
    // Emit book saved event
    println!("💾 Emitting book saved event for: {}", filename.as_str());
    event_svc.on_book_saved(&filename).await;

    Ok(Json(json!({
        "success": true,
        "operations_applied": request.operations.len(),
        "filename": filename.to_string()
    })))
}

 