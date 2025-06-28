// Validation utilities will be expanded as needed
pub fn validate_filename(filename: &str) -> bool {
    !filename.is_empty() && filename.ends_with(".pxl")
}

pub fn validate_dimensions(width: u16, height: u16) -> bool {
    width > 0 && height > 0 && width <= 4096 && height <= 4096
}

pub fn validate_color(color: &[u8; 4]) -> bool {
    true // RGBA values are always valid as u8
} 