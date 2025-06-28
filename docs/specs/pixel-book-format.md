# Pixel Book Format PRD

## Overview
Pixel books (`.pxl` files) are binary files that store pixel art data with support for multiple frames. The format is designed for efficient storage and fast loading while maintaining simplicity.

## File Format Specification

### File Structure
```
[Header]
[Frame Count]
[Frame Metadata]
[Frame Data...]
```

### Binary Layout

#### Header (16 bytes)
```
Offset | Size | Type   | Description
-------|------|--------|-------------
0      | 4    | u32    | Magic number: 0x504958 ("PIX")
4      | 2    | u16    | Format version: 1
6      | 2    | u16    | Width in pixels
8      | 2    | u16    | Height in pixels
10     | 2    | u16    | Frame count
12     | 4    | u32    | Reserved (must be 0)
```

#### Frame Metadata (per frame, 8 bytes each)
```
Offset | Size | Type   | Description
-------|------|--------|-------------
0      | 4    | u32    | Frame data offset from file start
4      | 4    | u32    | Frame data size in bytes
```

#### Frame Data (per frame)
```
Raw RGBA pixel data, row by row
Each pixel: 4 bytes (R, G, B, A)
Total size: width × height × 4 bytes
```

### Endianness
All multi-byte values are stored in little-endian format.

## Data Types

### Pixel Format
- **Red**: 0-255 (u8)
- **Green**: 0-255 (u8)  
- **Blue**: 0-255 (u8)
- **Alpha**: 0-255 (u8, where 0 = transparent, 255 = opaque)

### Constraints
- **Maximum Width**: 65,535 pixels
- **Maximum Height**: 65,535 pixels
- **Maximum Frames**: 65,535 frames
- **File Size Limit**: 4GB (practical limit much smaller)

## Serialization Format

### Rust Data Structures
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixelBook {
    pub width: u16,
    pub height: u16,
    pub frames: Vec<Frame>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub pixels: Vec<Vec<Pixel>>, // [y][x] indexing
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
```

### JSON Representation (API)
For API communication, pixel books are represented in JSON:

```json
{
  "filename": "character.pxl",
  "width": 32,
  "height": 32,
  "frames": [
    {
      "index": 0,
      "pixels": [
        [[255, 0, 0, 255], [0, 255, 0, 255], ...],
        [[0, 0, 255, 255], [255, 255, 0, 255], ...],
        ...
      ]
    }
  ]
}
```

## File Operations

### Loading Process
1. Read and validate header
2. Verify magic number and version
3. Read frame metadata
4. Load frame data on demand or all at once
5. Convert to internal representation

### Saving Process
1. Calculate frame offsets and sizes
2. Write header with metadata
3. Write frame metadata table
4. Write frame pixel data sequentially
5. Flush and sync file

### Validation Rules

#### Header Validation
- Magic number must be 0x504958
- Version must be supported (currently 1)
- Width and height must be > 0
- Frame count must be > 0
- Reserved field must be 0

#### Frame Validation
- Frame offsets must be within file bounds
- Frame sizes must match width × height × 4
- All frames must have identical dimensions
- Pixel data must be complete

#### Data Integrity
- File size must match expected size from metadata
- No overlapping frame data regions
- All frames must be accessible

## Error Handling

### File Format Errors
- **Invalid Magic Number**: File is not a pixel book
- **Unsupported Version**: Format version not supported
- **Corrupted Header**: Invalid or inconsistent header data
- **Truncated File**: File smaller than expected
- **Invalid Frame Data**: Frame data doesn't match metadata

### Recovery Strategies
- Validate header before processing any data
- Check file size against expected size
- Verify frame offsets are reasonable
- Provide detailed error messages for debugging

## Performance Considerations

### Memory Usage
- Load frames on demand for large files
- Use memory mapping for very large pixel books
- Implement frame caching with LRU eviction

### I/O Optimization
- Sequential frame data layout for efficient loading
- Batch read operations when possible
- Use buffered I/O for better performance

### Compression (Future)
- Consider RLE compression for large uniform areas
- PNG-style filtering for better compression
- Optional compression flag in header

## Compatibility

### Version History
- **Version 1**: Initial format with basic RGBA frames

### Migration Strategy
- Maintain backward compatibility with previous versions
- Provide conversion utilities if format changes
- Use version field to handle different formats

### External Tool Support
- Document format for third-party tool development
- Provide reference implementation
- Consider standardization if format becomes popular

## Example Files

### Minimal Pixel Book (1x1, 1 frame)
```
Header: PIX + version 1 + 1×1 dimensions + 1 frame
Frame Metadata: offset 16, size 4
Frame Data: [255, 0, 0, 255] (red pixel)
Total Size: 28 bytes
```

### Multi-Frame Animation (32x32, 4 frames)
```
Header: PIX + version 1 + 32×32 dimensions + 4 frames
Frame Metadata: 4 entries with offsets and sizes
Frame Data: 4 × (32 × 32 × 4) = 16,384 bytes of pixel data
Total Size: 16,464 bytes
``` 