pub struct ScalingCalculator;

impl ScalingCalculator {
    pub fn calculate_scale_and_offset(
        image_width: u16,
        image_height: u16,
        window_width: usize,
        window_height: usize,
    ) -> (u32, i32, i32) {
        let scale_x = window_width / image_width as usize;
        let scale_y = window_height / image_height as usize;
        let scale = std::cmp::min(scale_x, scale_y).max(1) as u32;
        
        let scaled_width = (image_width as u32 * scale) as i32;
        let scaled_height = (image_height as u32 * scale) as i32;
        
        let offset_x = (window_width as i32 - scaled_width) / 2;
        let offset_y = (window_height as i32 - scaled_height) / 2;
        
        (scale, offset_x, offset_y)
    }
    
    pub fn pixel_to_screen_coords(
        pixel_x: u16,
        pixel_y: u16,
        scale: u32,
        offset_x: i32,
        offset_y: i32,
    ) -> (i32, i32) {
        let screen_x = offset_x + (pixel_x as u32 * scale) as i32;
        let screen_y = offset_y + (pixel_y as u32 * scale) as i32;
        (screen_x, screen_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_perfect_scale() {
        let (scale, offset_x, offset_y) = ScalingCalculator::calculate_scale_and_offset(
            32, 32, 128, 128
        );
        assert_eq!(scale, 4);
        assert_eq!(offset_x, 0);
        assert_eq!(offset_y, 0);
    }
    
    #[test]
    fn test_non_perfect_scale() {
        let (scale, offset_x, offset_y) = ScalingCalculator::calculate_scale_and_offset(
            32, 32, 100, 100
        );
        assert_eq!(scale, 3);
        assert_eq!(offset_x, 2);
        assert_eq!(offset_y, 2);
    }
} 