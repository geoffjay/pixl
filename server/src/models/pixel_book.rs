use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn transparent() -> Self {
        Self { r: 0, g: 0, b: 0, a: 0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub index: usize,
    pub pixels: Vec<Vec<Pixel>>, // [y][x] indexing
}

impl Frame {
    pub fn new(index: usize, width: u16, height: u16) -> Self {
        let pixels = vec![vec![Pixel::transparent(); width as usize]; height as usize];
        Self { index, pixels }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixelBook {
    pub filename: String,
    pub width: u16,
    pub height: u16,
    pub frames: Vec<Frame>,
}

impl PixelBook {
    pub fn new(filename: String, width: u16, height: u16, frame_count: usize) -> Self {
        let frames = (0..frame_count)
            .map(|i| Frame::new(i, width, height))
            .collect();
        
        Self {
            filename,
            width,
            height,
            frames,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PixelBookInfo {
    pub filename: String,
    pub size: u64,
    pub created: chrono::DateTime<chrono::Utc>,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub frames: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePixelBookRequest {
    pub filename: String,
    pub width: u16,
    pub height: u16,
    pub frames: usize,
} 