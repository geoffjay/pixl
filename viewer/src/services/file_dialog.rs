// File dialog service will be implemented in Phase 3
use crate::services::ApiClient;
use std::error::Error;

pub struct FileDialogService {
    #[allow(dead_code)]
    api_client: ApiClient,
}

impl FileDialogService {
    pub fn new(api_client: ApiClient) -> Self {
        Self { api_client }
    }
    
    pub async fn show_open_dialog(&self) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
        // TODO: Implement file dialog in Phase 3
        Ok(None)
    }
} 