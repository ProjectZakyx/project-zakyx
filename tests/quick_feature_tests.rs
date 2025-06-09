//! Schnelle, fokussierte Tests für einzelne Browser-Features

use anyhow::Result;
use ora_browser::*;
use windows::Win32::Foundation::HWND;

#[test]
fn test_webview2_basic_navigation() -> Result<()> {
    println!("🧭 Testing WebView2 basic navigation...");
    
    let controller = WebView2Controller::new(HWND(0))?;
    assert_eq!(controller.get_current_url(), "gui");
    
    controller.navigate_to_url("https://google.com")?;
    assert_eq!(controller.get_current_url(), "https://google.com");
    
    println!("✅ WebView2 navigation works!");
    Ok(())
}

#[test]
fn test_tab_manager_basics() -> Result<()> {
    println!("📂 Testing Tab Manager basics...");
    
    let mut tab_manager = SimpleTabManager::new();
    assert_eq!(tab_manager.tab_count, 1);
    
    let new_tab = tab_manager.create_new_tab();
    assert_eq!(tab_manager.tab_count, 2);
    assert_eq!(new_tab, 2);
    
    println!("✅ Tab Manager works!");
    Ok(())
}

#[test]
fn test_bookmark_system() -> Result<()> {
    println!("⭐ Testing Bookmark System...");
    
    let mut bookmarks = SimpleBookmarkManager::new();
    let initial_count = bookmarks.bookmark_count;
    
    bookmarks.add_bookmark("Test", "https://test.com");
    assert_eq!(bookmarks.bookmark_count, initial_count + 1);
    
    println!("✅ Bookmark System works!");
    Ok(())
}

#[test]
fn test_history_tracking() -> Result<()> {
    println!("📚 Testing History Tracking...");
    
    let mut history = SimpleHistoryManager::new();
    assert_eq!(history.history_count, 0);
    
    history.add_entry("Test Site", "https://test.com");
    assert_eq!(history.history_count, 1);
    
    println!("✅ History Tracking works!");
    Ok(())
}

#[tokio::test]
async fn test_performance_optimizer() -> Result<()> {
    println!("⚡ Testing Performance Optimizer...");
    
    let optimizer = PerformanceOptimizer::new()?;
    
    optimizer.start_startup_timer();
    let startup_time = optimizer.finish_startup_timer();
    assert!(startup_time.as_nanos() > 0);
    
    println!("✅ Performance Optimizer works!");
    Ok(())
}

#[test]
fn test_browser_features_integration() -> Result<()> {
    println!("🔗 Testing Browser Features Integration...");
    
    let mut features = SimpleBrowserFeaturesManager::new(HWND(0))?;
    
    features.navigate_to_url("https://test.com")?;
    assert_eq!(features.tab_manager.current_url, "https://test.com");
    
    let status = features.get_status_info();
    assert!(status.contains_key("current_url"));
    
    println!("✅ Browser Features Integration works!");
    Ok(())
}

#[test]
fn test_gui_html_content() -> Result<()> {
    println!("🎨 Testing GUI HTML Content...");
    
    let controller = WebView2Controller::new(HWND(0))?;
    let html_content = controller.get_html_content();
    
    assert!(!html_content.is_empty());
    assert!(html_content.contains("html") || html_content.contains("HTML"));
    
    println!("✅ GUI HTML Content works!");
    Ok(())
} 