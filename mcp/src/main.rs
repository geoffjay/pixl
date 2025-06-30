use poem_mcpserver::{content::Text, stdio::stdio, McpServer, Tools};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;



/// The PIXL MCP Server provides tools for creating and manipulating pixel art images.
/// It connects to a running PIXL server instance to perform operations on pixel books.
/// 
/// Server URL can be configured via PIXL_SERVER_URL environment variable (defaults to http://localhost:3000)
struct PixlMcpServer {
    client: Client,
    server_url: String,
}

impl PixlMcpServer {
    fn new() -> Self {
        let server_url = std::env::var("PIXL_SERVER_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());
        
        Self {
            client: Client::new(),
            server_url,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LineType {
    Straight,
    Curved,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ShapeType {
    Rectangle,
    Circle,
    Oval,
    Triangle,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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

#[derive(Serialize)]
struct SetPathRequest {
    path: String,
}

#[derive(Serialize)]
struct CreatePixelBookRequest {
    filename: String,
    width: u16,
    height: u16,
    frames: usize,
}

#[derive(Serialize)]
struct UpdatePixelBookRequest {
    operations: Vec<DrawingOperation>,
}

/// This server provides comprehensive tools for creating and manipulating pixel art images.
/// 
/// The PIXL MCP Server acts as a bridge between AI models and the PIXL API, enabling
/// AI-driven pixel art creation through a rich set of drawing tools and file management
/// capabilities.
#[Tools]
impl PixlMcpServer {
    /// Check if the PIXL server is running and healthy
    async fn health_check(&self) -> Text<String> {
        let message = match self.client
            .get(&format!("{}/", self.server_url))
            .send()
            .await 
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<serde_json::Value>().await {
                        Ok(body) => format!("PIXL server is healthy: {}", 
                            serde_json::to_string_pretty(&body).unwrap_or_else(|_| "{}".to_string())),
                        Err(e) => format!("PIXL server response error: {}", e)
                    }
                } else {
                    format!("PIXL server is not healthy: {}", response.status())
                }
            },
            Err(e) => format!("Failed to connect to PIXL server: {}", e)
        };
        Text(message)
    }

    /// Get the current file system path where pixel books are stored
    async fn get_path(&self) -> Text<String> {
        let message = match self.client
            .get(&format!("{}/path", self.server_url))
            .send()
            .await 
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<serde_json::Value>().await {
                        Ok(body) => format!("Current path: {}", 
                            body["path"].as_str().unwrap_or("unknown")),
                        Err(e) => format!("Failed to parse response: {}", e)
                    }
                } else {
                    format!("Failed to get path: {}", response.status())
                }
            },
            Err(e) => format!("Failed to connect to PIXL server: {}", e)
        };
        Text(message)
    }

    /// Set the file system path where pixel books should be stored
    async fn set_path(&self, path: String) -> Text<String> {
        let request = SetPathRequest { path: path.clone() };
        
        let message = match self.client
            .put(&format!("{}/path", self.server_url))
            .json(&request)
            .send()
            .await 
        {
            Ok(response) => {
                if response.status().is_success() {
                    format!("Path set to: {}", path)
                } else {
                    let status = response.status();
                    match response.text().await {
                        Ok(error_text) => format!("Failed to set path: {}", error_text),
                        Err(_) => format!("Failed to set path: HTTP {}", status)
                    }
                }
            },
            Err(e) => format!("Failed to connect to PIXL server: {}", e)
        };
        Text(message)
    }

    /// List all available pixel books in the current directory
    async fn list_books(&self) -> Text<String> {
        let message = match self.client
            .get(&format!("{}/books", self.server_url))
            .send()
            .await 
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<serde_json::Value>().await {
                        Ok(body) => format!("Available pixel books:\n{}", 
                            serde_json::to_string_pretty(&body).unwrap_or_else(|_| "{}".to_string())),
                        Err(e) => format!("Failed to parse response: {}", e)
                    }
                } else {
                    format!("Failed to list books: {}", response.status())
                }
            },
            Err(e) => format!("Failed to connect to PIXL server: {}", e)
        };
        Text(message)
    }

    /// Create a new pixel book with specified dimensions and frame count
    async fn create_book(
        &self,
        filename: String,
        width: u16,
        height: u16,
        frames: usize,
    ) -> Text<String> {
        let request = CreatePixelBookRequest {
            filename: filename.clone(),
            width,
            height,
            frames,
        };
        
        let message = match self.client
            .post(&format!("{}/books", self.server_url))
            .json(&request)
            .send()
            .await 
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<serde_json::Value>().await {
                        Ok(body) => format!("Created pixel book '{}' ({}x{}, {} frames): {}", 
                            filename, width, height, frames,
                            serde_json::to_string_pretty(&body).unwrap_or_else(|_| "{}".to_string())),
                        Err(e) => format!("Created pixel book '{}' but failed to parse response: {}", filename, e)
                    }
                } else {
                    let status = response.status();
                    match response.text().await {
                        Ok(error_text) => format!("Failed to create book: {}", error_text),
                        Err(_) => format!("Failed to create book: HTTP {}", status)
                    }
                }
            },
            Err(e) => format!("Failed to connect to PIXL server: {}", e)
        };
        Text(message)
    }

    /// Get information about a specific pixel book
    async fn get_book(&self, filename: String) -> Text<String> {
        let message = match self.client
            .get(&format!("{}/books/{}", self.server_url, filename))
            .send()
            .await 
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<serde_json::Value>().await {
                        Ok(body) => format!("Pixel book '{}' details:\n{}", 
                            filename, serde_json::to_string_pretty(&body).unwrap_or_else(|_| "{}".to_string())),
                        Err(e) => format!("Failed to parse response: {}", e)
                    }
                } else {
                    let status = response.status();
                    match response.text().await {
                        Ok(error_text) => format!("Failed to get book '{}': {}", filename, error_text),
                        Err(_) => format!("Failed to get book '{}': HTTP {}", filename, status)
                    }
                }
            },
            Err(e) => format!("Failed to connect to PIXL server: {}", e)
        };
        Text(message)
    }

    /// Draw a single pixel at specified coordinates with a given color
    async fn draw_pixel(
        &self,
        filename: String,
        frame: usize,
        x: u16,
        y: u16,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> Text<String> {
        let operation = DrawingOperation::DrawPixel {
            frame,
            x,
            y,
            color: [r, g, b, a],
        };
        
        self.apply_operations(filename, vec![operation]).await
    }

    /// Set the current drawing color (for tools that use current color)
    async fn set_color(
        &self,
        filename: String,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> Text<String> {
        let operation = DrawingOperation::SetColor {
            color: [r, g, b, a],
        };
        
        self.apply_operations(filename, vec![operation]).await
    }

    /// Draw a line between two points
    async fn draw_line(
        &self,
        filename: String,
        frame: usize,
        start_x: u16,
        start_y: u16,
        end_x: u16,
        end_y: u16,
        line_type: String,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> Text<String> {
        let line_type = match line_type.to_lowercase().as_str() {
            "straight" => LineType::Straight,
            "curved" => LineType::Curved,
            _ => return Text("Invalid line type. Use 'straight' or 'curved'".to_string()),
        };
        
        let operation = DrawingOperation::DrawLine {
            frame,
            start: Point { x: start_x, y: start_y },
            end: Point { x: end_x, y: end_y },
            line_type,
            color: [r, g, b, a],
        };
        
        self.apply_operations(filename, vec![operation]).await
    }

    /// Draw a shape (rectangle, circle, oval, or triangle)
    async fn draw_shape(
        &self,
        filename: String,
        frame: usize,
        shape_type: String,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        filled: bool,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> Text<String> {
        let shape = match shape_type.to_lowercase().as_str() {
            "rectangle" => ShapeType::Rectangle,
            "circle" => ShapeType::Circle,
            "oval" => ShapeType::Oval,
            "triangle" => ShapeType::Triangle,
            _ => return Text("Invalid shape type. Use 'rectangle', 'circle', 'oval', or 'triangle'".to_string()),
        };
        
        let operation = DrawingOperation::DrawShape {
            frame,
            shape,
            position: Point { x, y },
            size: Size { width, height },
            filled,
            color: [r, g, b, a],
        };
        
        self.apply_operations(filename, vec![operation]).await
    }

    /// Draw a polygon from a list of points
    async fn draw_polygon(
        &self,
        filename: String,
        frame: usize,
        points_json: String,
        filled: bool,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> Text<String> {
        let points: Vec<Point> = match serde_json::from_str(&points_json) {
            Ok(points) => points,
            Err(e) => return Text(format!("Invalid points JSON: {}. Expected format: [{{\"x\": 10, \"y\": 20}}, ...]", e))
        };
        
        if points.len() < 3 {
            return Text("Polygon must have at least 3 points".to_string());
        }
        
        let operation = DrawingOperation::DrawPolygon {
            frame,
            points,
            filled,
            color: [r, g, b, a],
        };
        
        self.apply_operations(filename, vec![operation]).await
    }

    /// Fill an area starting from the specified point with the given color (flood fill)
    async fn fill_area(
        &self,
        filename: String,
        frame: usize,
        x: u16,
        y: u16,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> Text<String> {
        let operation = DrawingOperation::FillArea {
            frame,
            x,
            y,
            color: [r, g, b, a],
        };
        
        self.apply_operations(filename, vec![operation]).await
    }

    /// Apply multiple drawing operations in a single batch
    async fn batch_operations(
        &self,
        filename: String,
        operations_json: String,
    ) -> Text<String> {
        let operations: Vec<DrawingOperation> = match serde_json::from_str(&operations_json) {
            Ok(operations) => operations,
            Err(e) => return Text(format!("Invalid operations JSON: {}", e))
        };
        
        self.apply_operations(filename, operations).await
    }

    /// Helper method to apply operations to a pixel book
    async fn apply_operations(
        &self,
        filename: String,
        operations: Vec<DrawingOperation>,
    ) -> Text<String> {
        let request = UpdatePixelBookRequest { operations: operations.clone() };
        
        let message = match self.client
            .put(&format!("{}/books/{}", self.server_url, filename))
            .json(&request)
            .send()
            .await 
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<serde_json::Value>().await {
                        Ok(body) => format!("Applied {} operation(s) to '{}': {}", 
                            operations.len(), filename,
                            serde_json::to_string_pretty(&body).unwrap_or_else(|_| "{}".to_string())),
                        Err(e) => format!("Applied {} operation(s) to '{}' but failed to parse response: {}", 
                            operations.len(), filename, e)
                    }
                } else {
                    let status = response.status();
                    match response.text().await {
                        Ok(error_text) => format!("Failed to apply operations to '{}': {}", filename, error_text),
                        Err(_) => format!("Failed to apply operations to '{}': HTTP {}", filename, status)
                    }
                }
            },
            Err(e) => format!("Failed to connect to PIXL server: {}", e)
        };
        Text(message)
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    let server = PixlMcpServer::new();
    
    stdio(McpServer::new().tools(server)).await
}
