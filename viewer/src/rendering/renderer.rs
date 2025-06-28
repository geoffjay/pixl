use crate::models::{Frame, Pixel};
use crate::rendering::{ScalingCalculator, CheckerboardPattern};

pub struct Renderer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    checkerboard: CheckerboardPattern,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
            checkerboard: CheckerboardPattern::new(),
        }
    }
    
    pub fn update_size(&mut self, width: usize, height: usize) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.buffer.resize(width * height, 0);
        }
    }
    
    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }
    
    pub fn clear(&mut self) {
        self.buffer.fill(0x000000); // Black
    }
    
    pub fn render_frame(&mut self, frame: &Frame, image_width: u16, image_height: u16) {
        self.clear();
        
        let (scale, offset_x, offset_y) = ScalingCalculator::calculate_scale_and_offset(
            image_width,
            image_height,
            self.width,
            self.height,
        );
        
        for (y, row) in frame.pixels.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                self.render_pixel(x as u16, y as u16, pixel, scale, offset_x, offset_y);
            }
        }
    }
    
    fn render_pixel(&mut self, x: u16, y: u16, pixel: &Pixel, scale: u32, offset_x: i32, offset_y: i32) {
        let (screen_x, screen_y) = ScalingCalculator::pixel_to_screen_coords(x, y, scale, offset_x, offset_y);
        
        // Check bounds
        if screen_x < 0 || screen_y < 0 {
            return;
        }
        
        let screen_x = screen_x as usize;
        let screen_y = screen_y as usize;
        
        if screen_x + scale as usize > self.width || screen_y + scale as usize > self.height {
            return;
        }
        
        // Render the scaled pixel
        for dy in 0..scale {
            for dx in 0..scale {
                let px = screen_x + dx as usize;
                let py = screen_y + dy as usize;
                
                if px < self.width && py < self.height {
                    let index = py * self.width + px;
                    
                    let color = if pixel.is_transparent() {
                        // Blend with checkerboard
                        let bg_color = self.checkerboard.get_color_at(px as u32, py as u32, scale);
                        self.blend_colors(bg_color, pixel.to_rgba32(), pixel.a)
                    } else {
                        pixel.to_rgba32()
                    };
                    
                    self.buffer[index] = color;
                }
            }
        }
    }
    
    fn blend_colors(&self, background: u32, foreground: u32, alpha: u8) -> u32 {
        if alpha == 255 {
            return foreground;
        }
        if alpha == 0 {
            return background;
        }
        
        let alpha_f = alpha as f32 / 255.0;
        let inv_alpha = 1.0 - alpha_f;
        
        let bg_r = ((background >> 16) & 0xFF) as f32;
        let bg_g = ((background >> 8) & 0xFF) as f32;
        let bg_b = (background & 0xFF) as f32;
        
        let fg_r = ((foreground >> 16) & 0xFF) as f32;
        let fg_g = ((foreground >> 8) & 0xFF) as f32;
        let fg_b = (foreground & 0xFF) as f32;
        
        let r = (fg_r * alpha_f + bg_r * inv_alpha) as u32;
        let g = (fg_g * alpha_f + bg_g * inv_alpha) as u32;
        let b = (fg_b * alpha_f + bg_b * inv_alpha) as u32;
        
        (r << 16) | (g << 8) | b
    }
} 