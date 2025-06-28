use crate::models::Pixel;

pub fn pixel_to_minifb_color(pixel: &Pixel) -> u32 {
    ((pixel.r as u32) << 16) | ((pixel.g as u32) << 8) | (pixel.b as u32)
}

pub fn rgba_to_minifb_color(r: u8, g: u8, b: u8, _a: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
} 