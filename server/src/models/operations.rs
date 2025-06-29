use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DrawingOperation {
    #[serde(rename = "draw_pixel")]
    DrawPixel {
        frame: usize,
        x: u16,
        y: u16,
        color: [u8; 4],
    },
    #[serde(rename = "set_color")]
    SetColor {
        color: [u8; 4],
    },
    #[serde(rename = "draw_line")]
    DrawLine {
        frame: usize,
        start: Point,
        end: Point,
        line_type: LineType,
        color: [u8; 4],
    },
    #[serde(rename = "draw_shape")]
    DrawShape {
        frame: usize,
        shape: ShapeType,
        position: Point,
        size: Size,
        filled: bool,
        color: [u8; 4],
    },
    #[serde(rename = "draw_polygon")]
    DrawPolygon {
        frame: usize,
        points: Vec<Point>,
        filled: bool,
        color: [u8; 4],
    },
    #[serde(rename = "fill_area")]
    FillArea {
        frame: usize,
        x: u16,
        y: u16,
        color: [u8; 4],
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineType {
    #[serde(rename = "straight")]
    Straight,
    #[serde(rename = "curved")]
    Curved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShapeType {
    #[serde(rename = "rectangle")]
    Rectangle,
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "oval")]
    Oval,
    #[serde(rename = "triangle")]
    Triangle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePixelBookRequest {
    pub operations: Vec<DrawingOperation>,
} 