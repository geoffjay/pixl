use thiserror::Error;

#[derive(Error, Debug)]
pub enum PixelError {
    #[error("File not found: {filename}")]
    FileNotFound { filename: String },
    
    #[error("Invalid file format: {details}")]
    InvalidFormat { details: String },
    
    #[error("Invalid coordinates: x={x}, y={y} for image size {width}x{height}")]
    InvalidCoordinates { x: u16, y: u16, width: u16, height: u16 },
    
    #[error("Invalid color values: {details}")]
    InvalidColor { details: String },
    
    #[error("Invalid path: {path}")]
    InvalidPath { path: String },
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, PixelError>; 