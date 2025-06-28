# Server API PRD

## Overview
The PIXL server provides a REST API for managing pixel books and performing pixel art operations. All operations are performed on files within a configurable file system path.

## Base Configuration
- **Port**: 3000 (configurable)
- **Content-Type**: `application/json` for JSON endpoints
- **File Storage**: File system based, no database
- **Default Path**: User home directory

## API Endpoints

### Path Management

#### PUT /path
Set the file system location for pixel book storage.

**Request Body:**
```json
{
  "path": "/path/to/pixel/books"
}
```

**Response:**
```json
{
  "success": true,
  "path": "/path/to/pixel/books"
}
```

#### GET /path
Get the current file system location.

**Response:**
```json
{
  "path": "/path/to/pixel/books"
}
```

### Pixel Book Management

#### GET /books
List all available pixel books in the configured path.

**Response:**
```json
{
  "books": [
    {
      "filename": "character.pxl",
      "size": 1024,
      "created": "2024-01-01T00:00:00Z",
      "modified": "2024-01-01T12:00:00Z",
      "frames": 4
    }
  ]
}
```

#### GET /books/{filename}
Get pixel book data for the specified filename.

**Response:**
```json
{
  "filename": "character.pxl",
  "width": 32,
  "height": 32,
  "frames": [
    {
      "index": 0,
      "pixels": [[255, 0, 0, 255], [0, 255, 0, 255]]
    }
  ]
}
```

#### POST /books
Create a new pixel book.

**Request Body:**
```json
{
  "filename": "new-character.pxl",
  "width": 32,
  "height": 32,
  "frames": 1
}
```

**Response:**
```json
{
  "success": true,
  "filename": "new-character.pxl",
  "path": "/full/path/to/new-character.pxl"
}
```

#### GET /books/{filename}/events
Server-Sent Events stream for real-time updates to a pixel book.

**Response Headers:**
```
Content-Type: text/event-stream
Cache-Control: no-cache
```

**Event Format:**
```
event: update
data: {"frame": 0, "x": 10, "y": 15, "color": [255, 0, 0, 255]}

event: frame_added
data: {"frame": 1}
```

#### PUT /books/{filename}
Perform operations on a pixel book.

**Request Body:**
```json
{
  "operations": [
    {
      "type": "draw_pixel",
      "frame": 0,
      "x": 10,
      "y": 15,
      "color": [255, 0, 0, 255]
    }
  ]
}
```

## Drawing Operations

### Draw Pixel
```json
{
  "type": "draw_pixel",
  "frame": 0,
  "x": 10,
  "y": 15,
  "color": [255, 0, 0, 255]
}
```

### Set Color (for subsequent operations)
```json
{
  "type": "set_color",
  "color": [255, 0, 0, 255]
}
```

### Draw Line
```json
{
  "type": "draw_line",
  "frame": 0,
  "start": {"x": 0, "y": 0},
  "end": {"x": 10, "y": 10},
  "line_type": "straight",
  "color": [255, 0, 0, 255]
}
```

### Draw Shape
```json
{
  "type": "draw_shape",
  "frame": 0,
  "shape": "rectangle",
  "position": {"x": 5, "y": 5},
  "size": {"width": 10, "height": 8},
  "filled": false,
  "color": [255, 0, 0, 255]
}
```

**Supported Shapes:**
- `rectangle`
- `circle` 
- `oval`
- `triangle`

### Draw Polygon
```json
{
  "type": "draw_polygon",
  "frame": 0,
  "points": [
    {"x": 0, "y": 0},
    {"x": 10, "y": 5},
    {"x": 5, "y": 15}
  ],
  "filled": false,
  "color": [255, 0, 0, 255]
}
```

### Fill Area
```json
{
  "type": "fill_area",
  "frame": 0,
  "x": 10,
  "y": 15,
  "color": [255, 0, 0, 255]
}
```

## Error Handling

All endpoints return appropriate HTTP status codes:
- `200 OK`: Successful operation
- `400 Bad Request`: Invalid request format
- `404 Not Found`: File not found
- `422 Unprocessable Entity`: Invalid operation parameters
- `500 Internal Server Error`: Server error

**Error Response Format:**
```json
{
  "error": "File not found",
  "code": "FILE_NOT_FOUND",
  "details": "The file 'nonexistent.pxl' was not found in the configured path"
}
```

## Data Validation

### Coordinate Validation
- X, Y coordinates must be within image bounds
- Frame index must exist in pixel book

### Color Validation
- RGBA values must be 0-255
- Array must contain exactly 4 values

### File Validation
- Filename must end with `.pxl`
- Path must be within configured directory
- File must be readable/writable 