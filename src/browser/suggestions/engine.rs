// 🎯 URL SUGGESTIONS SYSTEM - Suggestion Engine
use crate::browser::bookmarks::BookmarkManager;
use crate::browser::history::HistoryManager;

#[derive(Debug)]
#[allow(dead_code)]
pub struct UrlSuggestionEngine {
    bookmark_manager: *const BookmarkManager,
    history_manager: *const HistoryManager,
}

#[allow(dead_code)]
impl UrlSuggestionEngine {
    #[allow(dead_code)]
    pub fn new(bookmark_manager: &BookmarkManager, history_manager: &HistoryManager) -> Self {
        UrlSuggestionEngine {
            bookmark_manager,
            history_manager,
        }
    }

    #[allow(dead_code)]
    pub fn get_suggestions(&self, query: &str) -> Vec<String> {
        let mut suggestions = Vec::new();

        if query.is_empty() {
            return suggestions;
        }

        // Add static suggestions
        let static_suggestions = vec![
            "google.com",
            "github.com", 
            "wikipedia.org",
            "gui",
            "webview"
        ];

        for suggestion in static_suggestions {
            if suggestion.contains(query) {
                suggestions.push(suggestion.to_string());
            }
        }

        unsafe {
            // Add bookmark suggestions
            if !self.bookmark_manager.is_null() {
                let bookmarks = (*self.bookmark_manager).get_bookmarks();
                for bookmark in bookmarks {
                    if bookmark.url.to_lowercase().contains(&query.to_lowercase()) ||
                       bookmark.title.to_lowercase().contains(&query.to_lowercase()) {
                        if !suggestions.contains(&bookmark.url) {
                            suggestions.push(bookmark.url.clone());
                        }
                    }
                }
            }

            // Add history suggestions
            if !self.history_manager.is_null() {
                let recent_history = (*self.history_manager).get_recent_history(10);
                for entry in recent_history {
                    if entry.url.to_lowercase().contains(&query.to_lowercase()) ||
                       entry.title.to_lowercase().contains(&query.to_lowercase()) {
                        if !suggestions.contains(&entry.url) {
                            suggestions.push(entry.url.clone());
                        }
                    }
                }
            }
        }

        suggestions.truncate(5); // Limit to 5 suggestions
        suggestions
    }
} 
