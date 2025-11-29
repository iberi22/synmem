//! Domain entities representing core business objects

mod extracted_content;
mod page;

pub use extracted_content::{ExtractedContent, ImageInfo, LinkInfo, StructuredData};
pub use page::Page;
