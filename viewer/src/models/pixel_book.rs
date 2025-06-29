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
    
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() >= 4 {
            Some(Self::new(bytes[0], bytes[1], bytes[2], bytes[3]))
        } else {
            None
        }
    }
    
    pub fn to_rgba32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
    
    pub fn is_transparent(&self) -> bool {
        self.a < 255
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub index: usize,
    pub pixels: Vec<u8>, // RGBA bytes: [r, g, b, a, r, g, b, a, ...]
}

impl Frame {
    pub fn get_pixel(&self, x: u16, y: u16, width: u16) -> Option<Pixel> {
        let pixel_idx = (y as usize * width as usize + x as usize) * 4;
        if pixel_idx + 3 < self.pixels.len() {
            Some(Pixel::new(
                self.pixels[pixel_idx],
                self.pixels[pixel_idx + 1],
                self.pixels[pixel_idx + 2],
                self.pixels[pixel_idx + 3],
            ))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixelBook {
    pub filename: String,
    pub width: u16,
    pub height: u16,
    pub frames: Vec<Frame>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PixelBookInfo {
    pub filename: String,
    pub size: u64,
    pub created: chrono::DateTime<chrono::Utc>,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub frames: usize,
} 