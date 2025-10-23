// 🚀 BROWSER MODULE - Complete Browser Features System
pub mod tabs;
pub mod bookmarks;
pub mod history;
pub mod suggestions;
pub mod features;
pub mod manager;

// Re-export all public types for easy access (currently unused but kept for API)
#[allow(unused_imports)]
pub use tabs::{BrowserTab, TabManager};
#[allow(unused_imports)]
pub use bookmarks::{Bookmark, BookmarkManager};
#[allow(unused_imports)]
pub use history::{HistoryEntry, HistoryManager};
#[allow(unused_imports)]
pub use suggestions::UrlSuggestionEngine;
#[allow(unused_imports)]
pub use features::AdvancedBrowserFeatures;
#[allow(unused_imports)]
pub use manager::BrowserFeaturesManager; 
