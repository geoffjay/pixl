use crate::models::{PixelBook, PixelBookInfo};
use reqwest::Client;
use std::error::Error;

#[derive(serde::Deserialize)]
struct BooksResponse {
    books: Vec<PixelBookInfo>,
}

#[derive(serde::Deserialize)]
struct PathResponse {
    path: String,
}

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
    
    pub async fn list_books(&self) -> Result<Vec<PixelBookInfo>, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/books", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(format!("Server error: {}", response.status()).into());
        }
        
        let books_response: BooksResponse = response.json().await?;
        Ok(books_response.books)
    }
    
    pub async fn get_book(&self, filename: &str) -> Result<PixelBook, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/books/{}", self.base_url, filename);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(format!("Server error: {}", response.status()).into());
        }
        
        let book: PixelBook = response.json().await?;
        Ok(book)
    }
    
    pub async fn get_path(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/path", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(format!("Server error: {}", response.status()).into());
        }
        
        let path_response: PathResponse = response.json().await?;
        Ok(path_response.path)
    }
    
    pub async fn health_check(&self) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/", self.base_url);
        let response = self.client.get(&url).send().await?;
        Ok(response.status().is_success())
    }
} 