// 🚀 BROWSER MODULE - Complete Browser Features System
pub mod tabs;
pub mod bookmarks;
pub mod history;
pub mod suggestions;
pub mod manager;

// Re-export all public types for easy access
pub use tabs::{BrowserTab, TabManager};
pub use bookmarks::{Bookmark, BookmarkManager};
pub use history::{HistoryEntry, HistoryManager};
pub use suggestions::UrlSuggestionEngine;
pub use manager::BrowserFeaturesManager; 