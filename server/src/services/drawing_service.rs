use crate::models::{PixelBook, DrawingOperation, ShapeType, LineType, Point, Size, PixelError};

pub struct DrawingService;

impl DrawingService {
    pub fn new() -> Self {
        Self
    }

    pub fn apply_operations(
        &self,
        book: &mut PixelBook,
        operations: Vec<DrawingOperation>,
    ) -> Result<(), PixelError> {
        for operation in operations {
            self.apply_operation(book, operation)?;
        }
        Ok(())
    }

    pub fn apply_operation(
        &self,
        book: &mut PixelBook,
        operation: DrawingOperation,
    ) -> Result<(), PixelError> {
        match operation {
            DrawingOperation::DrawPixel { frame, x, y, color } => {
                self.draw_pixel(book, frame, x, y, color)
            }
            DrawingOperation::SetColor { color: _ } => {
                // SetColor doesn't directly modify the pixel book, it's for setting drawing color
                Ok(())
            }
            DrawingOperation::DrawLine { frame, start, end, line_type, color } => {
                self.draw_line(book, frame, start, end, line_type, color)
            }
            DrawingOperation::DrawShape { frame, shape, position, size, filled, color } => {
                self.draw_shape(book, frame, shape, position, size, filled, color)
            }
            DrawingOperation::DrawPolygon { frame, points, filled, color } => {
                self.draw_polygon(book, frame, points, filled, color)
            }
            DrawingOperation::FillArea { frame, x, y, color } => {
                self.fill_area(book, frame, x, y, color)
            }
        }
    }

    fn draw_pixel(
        &self,
        book: &mut PixelBook,
        frame_idx: usize,
        x: u16,
        y: u16,
        color: [u8; 4],
    ) -> Result<(), PixelError> {
        if frame_idx >= book.frames.len() {
            return Err(PixelError::InvalidCoordinates {
                x, y, width: book.width, height: book.height
            });
        }

        let frame = &mut book.frames[frame_idx];
        if x >= book.width || y >= book.height {
            return Err(PixelError::InvalidCoordinates {
                x, y, width: book.width, height: book.height
            });
        }

        if (y as usize) < frame.pixels.len() && (x as usize) < frame.pixels[y as usize].len() {
            frame.pixels[y as usize][x as usize] = crate::models::Pixel::new(color[0], color[1], color[2], color[3]);
        }

        Ok(())
    }

    fn draw_line(
        &self,
        book: &mut PixelBook,
        frame_idx: usize,
        start: Point,
        end: Point,
        line_type: LineType,
        color: [u8; 4],
    ) -> Result<(), PixelError> {
        match line_type {
            LineType::Straight => self.draw_straight_line(book, frame_idx, start, end, color),
            LineType::Curved => {
                // For now, treat curved lines as straight lines
                // This can be enhanced later with proper curve algorithms
                self.draw_straight_line(book, frame_idx, start, end, color)
            }
        }
    }

    fn draw_straight_line(
        &self,
        book: &mut PixelBook,
        frame_idx: usize,
        start: Point,
        end: Point,
        color: [u8; 4],
    ) -> Result<(), PixelError> {
        // Bresenham's line algorithm
        let mut x0 = start.x as i32;
        let mut y0 = start.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            if x0 >= 0 && y0 >= 0 && x0 < book.width as i32 && y0 < book.height as i32 {
                self.draw_pixel(book, frame_idx, x0 as u16, y0 as u16, color)?;
            }

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }

        Ok(())
    }

    fn draw_shape(
        &self,
        book: &mut PixelBook,
        frame_idx: usize,
        shape: ShapeType,
        position: Point,
        size: Size,
        filled: bool,
        color: [u8; 4],
    ) -> Result<(), PixelError> {
        match shape {
            ShapeType::Rectangle => self.draw_rectangle(book, frame_idx, position, size, filled, color),
            ShapeType::Circle => self.draw_circle(book, frame_idx, position, size, filled, color),
            ShapeType::Oval => self.draw_oval(book, frame_idx, position, size, filled, color),
            ShapeType::Triangle => self.draw_triangle(book, frame_idx, position, size, filled, color),
        }
    }

    fn draw_rectangle(
        &self,
        book: &mut PixelBook,
        frame_idx: usize,
        position: Point,
        size: Size,
        filled: bool,
        color: [u8; 4],
    ) -> Result<(), PixelError> {
        let x1 = position.x;
        let y1 = position.y;
        let x2 = position.x + size.width.saturating_sub(1);
        let y2 = position.y + size.height.saturating_sub(1);

        if filled {
            for y in y1..=y2.min(book.height - 1) {
                for x in x1..=x2.min(book.width - 1) {
                    self.draw_pixel(book, frame_idx, x, y, color)?;
                }
            }
        } else {
            // Draw outline
            for x in x1..=x2.min(book.width - 1) {
                if y1 < book.height {
                    self.draw_pixel(book, frame_idx, x, y1, color)?;
                }
                if y2 < book.height && y2 != y1 {
                    self.draw_pixel(book, frame_idx, x, y2, color)?;
                }
            }
            for y in y1..=y2.min(book.height - 1) {
                if x1 < book.width {
                    self.draw_pixel(book, frame_idx, x1, y, color)?;
                }
                if x2 < book.width && x2 != x1 {
                    self.draw_pixel(book, frame_idx, x2, y, color)?;
                }
            }
        }

        Ok(())
    }

    fn draw_circle(
        &self,
        book: &mut PixelBook,
        frame_idx: usize,
        position: Point,
        size: Size,
        filled: bool,
        color: [u8; 4],
    ) -> Result<(), PixelError> {
        let cx = position.x as i32 + size.width as i32 / 2;
        let cy = position.y as i32 + size.height as i32 / 2;
        let radius = (size.width.min(size.height) / 2) as i32;

        if filled {
            for y in (cy - radius).max(0)..(cy + radius + 1).min(book.height as i32) {
                for x in (cx - radius).max(0)..(cx + radius + 1).min(book.width as i32) {
                    let dx = x - cx;
                    let dy = y - cy;
                    if dx * dx + dy * dy <= radius * radius {
                        self.draw_pixel(book, frame_idx, x as u16, y as u16, color)?;
                    }
                }
            }
        } else {
            // Midpoint circle algorithm for outline
            let mut x = 0;
            let mut y = radius;
            let mut d = 1 - radius;

            while x <= y {
                // Draw 8 points of symmetry
                self.draw_circle_points(book, frame_idx, cx, cy, x, y, color)?;
                
                if d < 0 {
                    d += 2 * x + 3;
                } else {
                    d += 2 * (x - y) + 5;
                    y -= 1;
                }
                x += 1;
            }
        }

        Ok(())
    }

    fn draw_circle_points(
        &self,
        book: &mut PixelBook,
        frame_idx: usize,
        cx: i32,
        cy: i32,
        x: i32,
        y: i32,
        color: [u8; 4],
    ) -> Result<(), PixelError> {
        let points = [
            (cx + x, cy + y), (cx + x, cy - y),
            (cx - x, cy + y), (cx - x, cy - y),
            (cx + y, cy + x), (cx + y, cy - x),
            (cx - y, cy + x), (cx - y, cy - x),
        ];

        for (px, py) in points {
            if px >= 0 && py >= 0 && px < book.width as i32 && py < book.height as i32 {
                self.draw_pixel(book, frame_idx, px as u16, py as u16, color)?;
            }
        }

        Ok(())
    }

    fn draw_oval(
        &self,
        book: &mut PixelBook,
        frame_idx: usize,
        position: Point,
        size: Size,
        filled: bool,
        color: [u8; 4],
    ) -> Result<(), PixelError> {
        let cx = position.x as i32 + size.width as i32 / 2;
        let cy = position.y as i32 + size.height as i32 / 2;
        let rx = (size.width / 2) as i32;
        let ry = (size.height / 2) as i32;

        if filled {
            for y in (cy - ry).max(0)..(cy + ry + 1).min(book.height as i32) {
                for x in (cx - rx).max(0)..(cx + rx + 1).min(book.width as i32) {
                    let dx = x - cx;
                    let dy = y - cy;
                    if rx * rx * dy * dy + ry * ry * dx * dx <= rx * rx * ry * ry {
                        self.draw_pixel(book, frame_idx, x as u16, y as u16, color)?;
                    }
                }
            }
        } else {
            // Simple ellipse outline using parametric equations
            let steps = ((rx + ry) * 2).max(20);
            for i in 0..steps {
                let angle = 2.0 * std::f64::consts::PI * i as f64 / steps as f64;
                let x = cx + (rx as f64 * angle.cos()) as i32;
                let y = cy + (ry as f64 * angle.sin()) as i32;
                
                if x >= 0 && y >= 0 && x < book.width as i32 && y < book.height as i32 {
                    self.draw_pixel(book, frame_idx, x as u16, y as u16, color)?;
                }
            }
        }

        Ok(())
    }

    fn draw_triangle(
        &self,
        book: &mut PixelBook,
        frame_idx: usize,
        position: Point,
        size: Size,
        filled: bool,
        color: [u8; 4],
    ) -> Result<(), PixelError> {
        // Simple triangle: top vertex at center-top, base at bottom
        let x1 = position.x + size.width / 2;  // Top vertex
        let y1 = position.y;
        let x2 = position.x;  // Bottom left
        let y2 = position.y + size.height.saturating_sub(1);
        let x3 = position.x + size.width.saturating_sub(1);  // Bottom right
        let y3 = position.y + size.height.saturating_sub(1);

        if filled {
            // Simple triangle fill using scanline approach
            for y in y1..=y2.min(book.height - 1) {
                let progress = if y2 == y1 { 0.0 } else { (y - y1) as f32 / (y2 - y1) as f32 };
                let left_x = x1 as f32 + progress * (x2 as f32 - x1 as f32);
                let right_x = x1 as f32 + progress * (x3 as f32 - x1 as f32);
                
                let start_x = (left_x as u16).min(right_x as u16);
                let end_x = (left_x as u16).max(right_x as u16);
                
                for x in start_x..=end_x.min(book.width - 1) {
                    self.draw_pixel(book, frame_idx, x, y, color)?;
                }
            }
        } else {
            // Draw triangle outline
            self.draw_straight_line(book, frame_idx, Point { x: x1, y: y1 }, Point { x: x2, y: y2 }, color)?;
            self.draw_straight_line(book, frame_idx, Point { x: x2, y: y2 }, Point { x: x3, y: y3 }, color)?;
            self.draw_straight_line(book, frame_idx, Point { x: x3, y: y3 }, Point { x: x1, y: y1 }, color)?;
        }

        Ok(())
    }

    fn draw_polygon(
        &self,
        book: &mut PixelBook,
        frame_idx: usize,
        points: Vec<Point>,
        filled: bool,
        color: [u8; 4],
    ) -> Result<(), PixelError> {
        if points.len() < 3 {
            return Ok(()); // Can't draw a polygon with less than 3 points
        }

        if filled {
            // Simple polygon fill using scanline algorithm
            let min_y = points.iter().map(|p| p.y).min().unwrap_or(0);
            let max_y = points.iter().map(|p| p.y).max().unwrap_or(0);

            for y in min_y..=max_y.min(book.height - 1) {
                let mut intersections = Vec::new();
                
                // Find intersections with polygon edges
                for i in 0..points.len() {
                    let p1 = &points[i];
                    let p2 = &points[(i + 1) % points.len()];
                    
                    if (p1.y <= y && p2.y > y) || (p2.y <= y && p1.y > y) {
                        let x_intersect = p1.x as f32 + 
                            (y as f32 - p1.y as f32) * (p2.x as f32 - p1.x as f32) / (p2.y as f32 - p1.y as f32);
                        intersections.push(x_intersect as u16);
                    }
                }
                
                intersections.sort();
                
                // Fill between pairs of intersections
                for chunk in intersections.chunks(2) {
                    if chunk.len() == 2 {
                        let start_x = chunk[0];
                        let end_x = chunk[1];
                        for x in start_x..=end_x.min(book.width - 1) {
                            self.draw_pixel(book, frame_idx, x, y, color)?;
                        }
                    }
                }
            }
        } else {
            // Draw polygon outline
            for i in 0..points.len() {
                let start = points[i].clone();
                let end = points[(i + 1) % points.len()].clone();
                self.draw_straight_line(book, frame_idx, start, end, color)?;
            }
        }

        Ok(())
    }

    fn fill_area(
        &self,
        book: &mut PixelBook,
        frame_idx: usize,
        x: u16,
        y: u16,
        color: [u8; 4],
    ) -> Result<(), PixelError> {
        if frame_idx >= book.frames.len() || x >= book.width || y >= book.height {
            return Err(PixelError::InvalidCoordinates {
                x, y, width: book.width, height: book.height
            });
        }

        // Get target color without borrowing book
        let target_color = {
            let frame = &book.frames[frame_idx];
            if (y as usize) >= frame.pixels.len() || (x as usize) >= frame.pixels[y as usize].len() {
                return Ok(());
            }
            let target_pixel = frame.pixels[y as usize][x as usize];
            [target_pixel.r, target_pixel.g, target_pixel.b, target_pixel.a]
        };

        if target_color == color {
            return Ok(()); // Already the target color
        }

        // Flood fill using a stack-based approach
        let mut stack = vec![(x, y)];
        let mut visited = std::collections::HashSet::new();

        while let Some((cx, cy)) = stack.pop() {
            if visited.contains(&(cx, cy)) {
                continue;
            }
            visited.insert((cx, cy));

            if cx >= book.width || cy >= book.height {
                continue;
            }

            // Check current pixel color without borrowing book mutably
            let current_color = {
                let frame = &book.frames[frame_idx];
                if (cy as usize) >= frame.pixels.len() || (cx as usize) >= frame.pixels[cy as usize].len() {
                    continue;
                }
                let current_pixel = frame.pixels[cy as usize][cx as usize];
                [current_pixel.r, current_pixel.g, current_pixel.b, current_pixel.a]
            };

            if current_color != target_color {
                continue;
            }

            // Fill this pixel
            self.draw_pixel(book, frame_idx, cx, cy, color)?;

            // Add neighboring pixels to stack
            if cx > 0 {
                stack.push((cx - 1, cy));
            }
            if cx + 1 < book.width {
                stack.push((cx + 1, cy));
            }
            if cy > 0 {
                stack.push((cx, cy - 1));
            }
            if cy + 1 < book.height {
                stack.push((cx, cy + 1));
            }
        }

        Ok(())
    }
} 