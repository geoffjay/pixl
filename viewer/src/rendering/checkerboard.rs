pub struct CheckerboardPattern {
    light_color: u32,
    dark_color: u32,
    square_size: u32,
}

impl CheckerboardPattern {
    pub fn new() -> Self {
        Self {
            light_color: 0xF0F0F0, // Light gray (240, 240, 240)
            dark_color: 0xC8C8C8,  // Dark gray (200, 200, 200)
            square_size: 8,
        }
    }
    
    pub fn get_color_at(&self, x: u32, y: u32, scale: u32) -> u32 {
        let checker_size = self.square_size * scale;
        let checker_x = x / checker_size;
        let checker_y = y / checker_size;
        
        if (checker_x + checker_y) % 2 == 0 {
            self.light_color
        } else {
            self.dark_color
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_checkerboard_pattern() {
        let pattern = CheckerboardPattern::new();
        
        // Test at scale 1
        assert_eq!(pattern.get_color_at(0, 0, 1), 0xF0F0F0);
        assert_eq!(pattern.get_color_at(8, 0, 1), 0xC8C8C8);
        assert_eq!(pattern.get_color_at(0, 8, 1), 0xC8C8C8);
        assert_eq!(pattern.get_color_at(8, 8, 1), 0xF0F0F0);
    }
} 