use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum PixelBookEvent {
    #[serde(rename = "update")]
    Update {
        frame: usize,
        x: u16,
        y: u16,
        color: [u8; 4],
    },
    #[serde(rename = "frame_added")]
    FrameAdded {
        frame: usize,
    },
    #[serde(rename = "frame_removed")]
    FrameRemoved {
        frame: usize,
    },
    #[serde(rename = "book_updated")]
    BookUpdated,
} 