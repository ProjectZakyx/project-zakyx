// 📚 Bookmark System Module
// Modulare Bookmark-Verwaltung für ZAKYX Browser

pub mod manager;
pub mod html_generator;
pub mod toolbar;

// Windows-spezifische Implementation nur auf Windows verfügbar
#[cfg(target_os = "windows")]
pub mod windows_impl;

// Re-exports für einfachen Zugriff
pub use manager::{BookmarkEntry, BookmarkManager};
pub use html_generator::BookmarkHtmlGenerator;
pub use toolbar::{HorizontalBookmarkToolbar, BookmarkToolbarManager};

#[cfg(target_os = "windows")]
pub use windows_impl::WindowsToolbarImpl;

// Convenience-Funktionen
pub fn create_bookmark_toolbar(parent_hwnd: windows::Win32::Foundation::HWND) -> anyhow::Result<HorizontalBookmarkToolbar> {
    HorizontalBookmarkToolbar::new(parent_hwnd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bookmark_entry_creation() {
        let bookmark = BookmarkEntry::new(
            "Test Bookmark".to_string(),
            "https://example.com".to_string()
        );
        
        assert_eq!(bookmark.title, "Test Bookmark");
        assert_eq!(bookmark.url, "https://example.com");
        assert!(!bookmark.id.is_empty());
    }

    #[test]
    fn test_bookmark_manager() {
        let mut manager = BookmarkManager::new();
        
        // Test adding bookmark
        let id = manager.add_bookmark("Test", "https://test.com").unwrap();
        assert_eq!(manager.get_bookmarks().len(), 1);
        
        // Test finding bookmark
        let bookmark = manager.find_bookmark(&id).unwrap();
        assert_eq!(bookmark.title, "Test");
        
        // Test removing bookmark
        let removed = manager.remove_bookmark(&id);
        assert!(removed);
        assert_eq!(manager.get_bookmarks().len(), 0);
    }

    #[test]
    fn test_html_generator() {
        let generator = BookmarkHtmlGenerator::new();
        let bookmarks = vec![
            BookmarkEntry::new("Test".to_string(), "https://test.com".to_string())
        ];
        
        let html = generator.generate_toolbar_html(&bookmarks);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Test"));
        assert!(html.contains("https://test.com"));
    }

    #[test]
    fn test_bookmark_search() {
        let mut manager = BookmarkManager::new();
        manager.add_bookmark("Google Search", "https://google.com").unwrap();
        manager.add_bookmark("YouTube Videos", "https://youtube.com").unwrap();
        
        let results = manager.search_bookmarks("google");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Google Search");
        
        let results = manager.search_bookmarks("video");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "YouTube Videos");
    }

    #[test]
    fn test_favicon_detection() {
        let manager = BookmarkManager::new();
        
        // Test various URL patterns for favicon detection
        let test_cases = vec![
            ("https://google.com", "🔍"),
            ("https://youtube.com", "📺"),
            ("https://github.com", "💼"),
            ("https://example.com", "🔖"), // default
        ];
        
        for (url, expected_favicon) in test_cases {
            let bookmark = BookmarkEntry::new("Test".to_string(), url.to_string());
            // Note: This test would need access to the private method
            // In a real implementation, we might make this method public for testing
        }
    }
}