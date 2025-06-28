use crate::models::{PixelBook, PixelBookInfo, CreatePixelBookRequest};
use crate::services::FileService;
use crate::utils::validation;
use poem::{handler, web::{Json, Path}, Result, Error};
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