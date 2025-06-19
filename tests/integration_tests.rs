// 🧪 INTEGRATION TESTS
// Tests für das gesamte Ora Browser System
// Copyright © 2024 Ora Browser Team

use ora_browser::{BrowserState, Tab};
use ora_browser::browser_state::Bookmark;
use std::sync::Arc;

#[tokio::test]
async fn test_browser_state_integration() {
    let state = BrowserState::new();
    
    // Test initial state
    let tabs = state.tabs.read().await;
    assert_eq!(tabs.len(), 0);
    drop(tabs);
    
    let bookmarks = state.bookmarks.read().await;
    assert!(bookmarks.len() >= 4); // Default bookmarks from BookmarkManager
    drop(bookmarks);
    
    let settings = state.settings.read().await;
    assert_eq!(settings.homepage, "https://google.com");
    drop(settings);
    
    let history = state.history.read().await;
    assert_eq!(history.len(), 0);
    drop(history);
}

#[tokio::test]
async fn test_tab_management_workflow() {
    let state = BrowserState::new();
    
    // Add a tab
    {
        let mut tabs = state.tabs.write().await;
        tabs.push(Tab {
            id: 1,
            title: "Test Tab".to_string(),
            url: "https://example.com".to_string(),
            is_active: true,
        });
    }
    
    // Verify tab was added
    {
        let tabs = state.tabs.read().await;
        assert_eq!(tabs.len(), 1);
        assert_eq!(tabs[0].title, "Test Tab");
        assert_eq!(tabs[0].url, "https://example.com");
        assert!(tabs[0].is_active);
    }
    
    // Update tab
    {
        let mut tabs = state.tabs.write().await;
        tabs[0].title = "Updated Tab".to_string();
        tabs[0].url = "https://updated.com".to_string();
    }
    
    // Verify update
    {
        let tabs = state.tabs.read().await;
        assert_eq!(tabs[0].title, "Updated Tab");
        assert_eq!(tabs[0].url, "https://updated.com");
    }
    
    // Remove tab
    {
        let mut tabs = state.tabs.write().await;
        tabs.clear();
    }
    
    // Verify removal
    {
        let tabs = state.tabs.read().await;
        assert_eq!(tabs.len(), 0);
    }
}

#[tokio::test]
async fn test_bookmark_management_workflow() {
    let state = BrowserState::new();
    
    // Get initial bookmark count (should have default bookmarks)
    let initial_count = {
        let bookmarks = state.bookmarks.read().await;
        bookmarks.len()
    };
    
    // Add a bookmark through the bookmark manager
    {
        let mut bookmark_manager = state.bookmark_manager.write().await;
        let bookmark_id = bookmark_manager.add_bookmark("Test Site", "https://test.com");
        let _ = bookmark_manager.save_bookmarks();
        println!("Added bookmark with ID: {}", bookmark_id);
    }
    
    // Sync bookmarks to update in-memory storage
    {
        let mut bookmarks = state.bookmarks.write().await;
        let bookmark_manager = state.bookmark_manager.read().await;
        
        let persistent_bookmarks: Vec<Bookmark> = bookmark_manager.get_bookmarks()
            .iter()
            .map(|b| Bookmark {
                id: b.id.to_string(),
                title: b.title.clone(),
                url: b.url.clone(),
            })
            .collect();
        
        *bookmarks = persistent_bookmarks;
    }
    
    // Verify bookmark was added
    {
        let bookmarks = state.bookmarks.read().await;
        assert_eq!(bookmarks.len(), initial_count + 1);
        
        // Find the test bookmark
        let test_bookmark = bookmarks.iter().find(|b| b.title == "Test Site");
        assert!(test_bookmark.is_some());
        assert_eq!(test_bookmark.unwrap().url, "https://test.com");
    }
    
    // Remove the test bookmark
    {
        let bookmarks = state.bookmarks.read().await;
        let test_bookmark = bookmarks.iter().find(|b| b.title == "Test Site").unwrap();
        let bookmark_id: u32 = test_bookmark.id.parse().unwrap();
        
        drop(bookmarks); // Release the read lock
        
        let mut bookmark_manager = state.bookmark_manager.write().await;
        let removed = bookmark_manager.remove_bookmark(bookmark_id);
        assert!(removed);
        let _ = bookmark_manager.save_bookmarks();
    }
    
    // Sync bookmarks again
    {
        let mut bookmarks = state.bookmarks.write().await;
        let bookmark_manager = state.bookmark_manager.read().await;
        
        let persistent_bookmarks: Vec<Bookmark> = bookmark_manager.get_bookmarks()
            .iter()
            .map(|b| Bookmark {
                id: b.id.to_string(),
                title: b.title.clone(),
                url: b.url.clone(),
            })
            .collect();
        
        *bookmarks = persistent_bookmarks;
    }
    
    // Verify removal (back to initial count)
    {
        let bookmarks = state.bookmarks.read().await;
        assert_eq!(bookmarks.len(), initial_count);
    }
}

#[test]
fn test_url_utils_integration() {
    use ora_browser::url_utils::{normalize_problematic_url, should_use_proxy_for_url};
    
    // Test the complete URL processing workflow
    let test_urls = vec![
        "google.com",
        "github.com",
        "example.com",
        "https://www.google.com",
        "dzen.ru",
    ];
    
    for url in test_urls {
        let normalized = normalize_problematic_url(url);
        let should_proxy = should_use_proxy_for_url(&normalized);
        
        // Verify that normalized URLs are valid
        assert!(normalized.starts_with("http://") || normalized.starts_with("https://"));
        
        // Verify proxy decision is consistent
        if normalized.contains("google.com") || normalized.contains("github.com") || normalized.contains("dzen.ru") {
            assert!(should_proxy, "URL {} should use proxy", normalized);
        }
    }
}
