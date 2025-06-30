use crate::models::{PixelBook, Frame, PixelBookInfo, Result, PixelError};
use std::fs::{File, OpenOptions, read_dir};
use std::path::{Path, PathBuf};
use std::io::{Read, Write, Seek, SeekFrom, BufWriter};
use chrono::{DateTime, Utc};

const MAGIC_NUMBER: u32 = 0x504958; // "PIX"
const FORMAT_VERSION: u16 = 1;

pub struct FileService {
    base_path: PathBuf,
}

impl FileService {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }
    
    pub fn set_path(&mut self, path: PathBuf) -> Result<()> {
        if !path.exists() || !path.is_dir() {
            return Err(PixelError::InvalidPath { 
                path: path.to_string_lossy().to_string() 
            });
        }
        self.base_path = path;
        Ok(())
    }
    
    pub fn get_path(&self) -> &Path {
        &self.base_path
    }
    
    pub fn list_books(&self) -> Result<Vec<PixelBookInfo>> {
        let mut books = Vec::new();
        
        for entry in read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("pxl") {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    let metadata = entry.metadata()?;
                    let size = metadata.len();
                    
                    // Get creation and modification times
                    let created = metadata.created()
                        .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                    let modified = metadata.modified()
                        .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                    
                    let created: DateTime<Utc> = created.into();
                    let modified: DateTime<Utc> = modified.into();
                    
                    // Try to read frame count from file header
                    let frames = self.get_frame_count(&path).unwrap_or(1);
                    
                    books.push(PixelBookInfo {
                        filename: filename.to_string(),
                        size,
                        created,
                        modified,
                        frames,
                    });
                }
            }
        }
        
        Ok(books)
    }
    
    fn get_frame_count(&self, path: &Path) -> Result<usize> {
        let mut file = File::open(path)?;
        let mut header = [0u8; 16];
        file.read_exact(&mut header)?;
        
        // Validate magic number
        let magic = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
        if magic != MAGIC_NUMBER {
            return Err(PixelError::InvalidFormat { 
                details: "Invalid magic number".to_string() 
            });
        }
        
        // Read frame count
        let frame_count = u16::from_le_bytes([header[10], header[11]]);
        Ok(frame_count as usize)
    }
    
    pub fn load_book(&self, filename: &str) -> Result<PixelBook> {
        let path = self.base_path.join(filename);
        let mut file = File::open(&path)?;
        
        // Read and validate header
        let mut header = [0u8; 16];
        file.read_exact(&mut header)?;
        
        let magic = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
        if magic != MAGIC_NUMBER {
            return Err(PixelError::InvalidFormat { 
                details: "Invalid magic number".to_string() 
            });
        }
        
        let version = u16::from_le_bytes([header[4], header[5]]);
        if version != FORMAT_VERSION {
            return Err(PixelError::InvalidFormat { 
                details: format!("Unsupported version: {}", version) 
            });
        }
        
        let width = u16::from_le_bytes([header[6], header[7]]);
        let height = u16::from_le_bytes([header[8], header[9]]);
        let frame_count = u16::from_le_bytes([header[10], header[11]]);
        
        if width == 0 || height == 0 || frame_count == 0 {
            return Err(PixelError::InvalidFormat { 
                details: "Invalid dimensions or frame count".to_string() 
            });
        }
        
        // Read frame metadata
        let mut frame_offsets = Vec::new();
        let mut frame_sizes = Vec::new();
        
        for _ in 0..frame_count {
            let mut metadata = [0u8; 8];
            file.read_exact(&mut metadata)?;
            
            let offset = u32::from_le_bytes([metadata[0], metadata[1], metadata[2], metadata[3]]);
            let size = u32::from_le_bytes([metadata[4], metadata[5], metadata[6], metadata[7]]);
            
            frame_offsets.push(offset);
            frame_sizes.push(size);
        }
        
        // Read frame data
        let mut frames = Vec::new();
        let expected_frame_size = (width as u32 * height as u32 * 4) as usize;
        
        for (i, (&offset, &size)) in frame_offsets.iter().zip(frame_sizes.iter()).enumerate() {
            if size as usize != expected_frame_size {
                return Err(PixelError::InvalidFormat { 
                    details: format!("Invalid frame size for frame {}", i) 
                });
            }
            
            file.seek(SeekFrom::Start(offset as u64))?;
            
            let mut pixel_data = vec![0u8; size as usize];
            file.read_exact(&mut pixel_data)?;
            
            // Store raw pixel data directly
            frames.push(Frame { index: i, pixels: pixel_data });
        }
        
        Ok(PixelBook {
            filename: filename.to_string(),
            width,
            height,
            frames,
        })
    }
    
    pub fn save_book(&self, book: &PixelBook) -> Result<()> {
        let path = self.base_path.join(&book.filename);
        let mut file = BufWriter::new(OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)?);
        
        let frame_count = book.frames.len() as u16;
        let frame_size = (book.width as u32 * book.height as u32 * 4) as u32;
        
        // Calculate frame offsets
        let header_size = 16u32;
        let metadata_size = frame_count as u32 * 8;
        let mut current_offset = header_size + metadata_size;
        
        // Write header
        file.write_all(&MAGIC_NUMBER.to_le_bytes())?;
        file.write_all(&FORMAT_VERSION.to_le_bytes())?;
        file.write_all(&book.width.to_le_bytes())?;
        file.write_all(&book.height.to_le_bytes())?;
        file.write_all(&frame_count.to_le_bytes())?;
        file.write_all(&[0u8; 4])?; // Reserved
        
        // Write frame metadata
        for _ in 0..frame_count {
            file.write_all(&current_offset.to_le_bytes())?;
            file.write_all(&frame_size.to_le_bytes())?;
            current_offset += frame_size;
        }
        
        // Write frame data
        for frame in &book.frames {
            file.write_all(&frame.pixels)?;
        }
        
        file.flush()?;
        Ok(())
    }
    
    pub fn create_book(&self, filename: &str, width: u16, height: u16, frames: usize) -> Result<PixelBook> {
        if width == 0 || height == 0 || frames == 0 {
            return Err(PixelError::InvalidFormat { 
                details: "Width, height, and frame count must be greater than 0".to_string() 
            });
        }
        
        let book = PixelBook::new(filename.to_string(), width, height, frames);
        self.save_book(&book)?;
        Ok(book)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_create_and_load_pixel_book() {
        let temp_dir = TempDir::new().unwrap();
        let file_service = FileService::new(temp_dir.path().to_path_buf());
        
        // Create a pixel book
        let book = file_service.create_book("test.pxl", 4, 4, 2).unwrap();
        assert_eq!(book.width, 4);
        assert_eq!(book.height, 4);
        assert_eq!(book.frames.len(), 2);
        
        // Load it back
        let loaded_book = file_service.load_book("test.pxl").unwrap();
        assert_eq!(loaded_book.width, 4);
        assert_eq!(loaded_book.height, 4);
        assert_eq!(loaded_book.frames.len(), 2);
        assert_eq!(loaded_book.filename, "test.pxl");
    }
    
    #[test]
    fn test_list_books() {
        let temp_dir = TempDir::new().unwrap();
        let file_service = FileService::new(temp_dir.path().to_path_buf());
        
        // Create some pixel books
        file_service.create_book("book1.pxl", 8, 8, 1).unwrap();
        file_service.create_book("book2.pxl", 16, 16, 3).unwrap();
        
        let books = file_service.list_books().unwrap();
        assert_eq!(books.len(), 2);
        
        let book1 = books.iter().find(|b| b.filename == "book1.pxl").unwrap();
        assert_eq!(book1.frames, 1);
        
        let book2 = books.iter().find(|b| b.filename == "book2.pxl").unwrap();
        assert_eq!(book2.frames, 3);
    }
} 