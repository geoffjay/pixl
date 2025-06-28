// Events API will be implemented in Phase 3
use poem::{handler, web::Path, web::sse::SSE};

#[handler]
pub async fn pixel_book_events(
    _filename: Path<String>,
) -> SSE {
    // TODO: Implement SSE in Phase 3
    SSE::new(futures_util::stream::empty())
} 