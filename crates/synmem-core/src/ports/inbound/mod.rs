//! # Inbound Ports (Driving)
//!
//! These ports define how external actors can interact with the SynMem system.

mod automation;
mod browser_control;
mod memory_query;
mod scraper;

pub use automation::{
    AutomationError, AutomationPort, AutomationResult, Macro, MacroAction, PlaybackOptions,
    PlaybackResult, RecordOptions,
};
pub use browser_control::{
    BrowserControlError, BrowserControlPort, BrowserControlResult, NavigateOptions, Screenshot,
    ScreenshotOptions,
};
pub use memory_query::{
    ContextToSave, MemoryEntry, MemoryQueryError, MemoryQueryPort, MemoryQueryResult,
    SearchOptions, SearchResult,
};
pub use scraper::{
    ExtractedLink, PageMetadata, ScrapeOptions, ScrapedPage, ScraperError, ScraperPort,
    ScraperResult, SimplifiedDom, SimplifiedDomNode,
};
