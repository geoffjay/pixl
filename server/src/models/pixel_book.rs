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
    pub pixels: Vec<u8>, // RGBA bytes: [r, g, b, a, r, g, b, a, ...]
}

impl Frame {
    pub fn new(index: usize, width: u16, height: u16) -> Self {
        let pixel_count = (width as usize) * (height as usize) * 4; // RGBA
        let pixels = vec![0u8; pixel_count]; // Transparent pixels
        Self { index, pixels }
    }
    
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
    
    pub fn set_pixel(&mut self, x: u16, y: u16, width: u16, pixel: Pixel) -> bool {
        let pixel_idx = (y as usize * width as usize + x as usize) * 4;
        if pixel_idx + 3 < self.pixels.len() {
            self.pixels[pixel_idx] = pixel.r;
            self.pixels[pixel_idx + 1] = pixel.g;
            self.pixels[pixel_idx + 2] = pixel.b;
            self.pixels[pixel_idx + 3] = pixel.a;
            true
        } else {
            false
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