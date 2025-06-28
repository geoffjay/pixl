use crate::services::FileService;
use poem::{handler, web::Json, Result, Error};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize)]
pub struct SetPathRequest {
    pub path: String,
}

#[derive(Serialize)]
pub struct PathResponse {
    pub path: String,
}

#[handler]
pub async fn get_path(
    file_service: poem::web::Data<&Arc<RwLock<FileService>>>,
) -> Result<Json<PathResponse>> {
    let service = file_service.read().await;
    let path = service.get_path().to_string_lossy().to_string();
    
    Ok(Json(PathResponse { path }))
}

#[handler]
pub async fn set_path(
    file_service: poem::web::Data<&Arc<RwLock<FileService>>>,
    request: Json<SetPathRequest>,
) -> Result<Json<PathResponse>> {
    let mut service = file_service.write().await;
    let new_path = std::path::PathBuf::from(&request.path);
    
    service.set_path(new_path)
        .map_err(|e| Error::from_string(e.to_string(), poem::http::StatusCode::BAD_REQUEST))?;
    
    Ok(Json(PathResponse { 
        path: request.path.clone() 
    }))
} 