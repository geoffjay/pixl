// File dialog service will be implemented in Phase 3
use crate::services::ApiClient;
use std::error::Error;
use rfd::AsyncFileDialog;
use std::path::Path;

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
    
    pub async fn open_pixel_book_dialog(&self) -> Option<String> {
        let file = AsyncFileDialog::new()
            .add_filter("Pixel Books", &["pxl"])
            .add_filter("All Files", &["*"])
            .set_title("Open Pixel Book")
            .pick_file()
            .await;
        
        file.map(|handle| {
            handle.file_name()
        })
    }
    
    pub async fn save_pixel_book_dialog(&self, current_filename: Option<&str>) -> Option<String> {
        let mut dialog = AsyncFileDialog::new()
            .add_filter("Pixel Books", &["pxl"])
            .add_filter("All Files", &["*"])
            .set_title("Save Pixel Book");
            
        if let Some(filename) = current_filename {
            dialog = dialog.set_file_name(filename);
        }
        
        let file = dialog.save_file().await;
        
        file.map(|handle| {
            handle.file_name()
        })
    }
    
    pub fn validate_pixel_book_filename(&self, filename: &str) -> bool {
        if filename.is_empty() {
            return false;
        }
        
        // Check if filename has .pxl extension
        let path = Path::new(filename);
        match path.extension() {
            Some(ext) => ext.to_str() == Some("pxl"),
            None => false,
        }
    }
    
    pub fn ensure_pxl_extension(&self, filename: &str) -> String {
        if self.validate_pixel_book_filename(filename) {
            filename.to_string()
        } else {
            format!("{}.pxl", filename)
        }
    }
} 