//! Umfassende Tests für Browser-Kernfunktionalität
//! 
//! Testet alle wichtigen Browser-Features systematisch

use anyhow::Result;
use ora_browser::*;
use std::collections::HashMap;
use windows::Win32::Foundation::HWND;

#[tokio::test]
async fn test_webview2_controller_creation() -> Result<()> {
    println!("🧪 Testing WebView2Controller creation...");
    
    let controller = WebView2Controller::new(HWND(0))?;
    assert!(!controller.get_current_url().is_empty());
    
    println!("✅ WebView2Controller creation test passed");
    Ok(())
}

#[tokio::test]
async fn test_navigation_functionality() -> Result<()> {
    println!("🧪 Testing navigation functionality...");
    
    let controller = WebView2Controller::new(HWND(0))?;
    
    // Test standard navigation
    controller.navigate_to_url("https://google.com")?;
    assert_eq!(controller.get_current_url(), "https://google.com");
    
    // Test GUI navigation
    controller.navigate_to_url("gui")?;
    assert_eq!(controller.get_current_url(), "gui");
    
    println!("✅ Navigation functionality test passed");
    Ok(())
}

#[tokio::test]
async fn test_tab_management() -> Result<()> {
    println!("🧪 Testing tab management...");
    
    let mut tab_manager = SimpleTabManager::new();
    
    // Test initial state
    assert_eq!(tab_manager.active_tab_id, 1);
    assert_eq!(tab_manager.tab_count, 1);
    
    // Test new tab creation
    let new_tab_id = tab_manager.create_new_tab();
    assert_eq!(new_tab_id, 2);
    assert_eq!(tab_manager.tab_count, 2);
    
    // Test navigation
    tab_manager.navigate_to("https://github.com");
    assert!(tab_manager.current_url.contains("github"));
    assert!(tab_manager.can_go_back);
    
    println!("✅ Tab management test passed");
    Ok(())
}

#[tokio::test]
async fn test_bookmark_functionality() -> Result<()> {
    println!("🧪 Testing bookmark functionality...");
    
    let mut bookmark_manager = SimpleBookmarkManager::new();
    
    // Test initial bookmarks
    assert!(bookmark_manager.bookmark_count > 0);
    
    // Test adding new bookmark
    let initial_count = bookmark_manager.bookmark_count;
    bookmark_manager.add_bookmark("🧪 Test Site", "https://test.com");
    assert_eq!(bookmark_manager.bookmark_count, initial_count + 1);
    
    // Test bookmark display
    let display = bookmark_manager.get_bookmarks_display();
    assert!(!display.is_empty());
    assert!(display.iter().any(|line| line.contains("🧪 Test Site")));
    
    println!("✅ Bookmark functionality test passed");
    Ok(())
}

#[tokio::test]
async fn test_history_tracking() -> Result<()> {
    println!("🧪 Testing history tracking...");
    
    let mut history_manager = SimpleHistoryManager::new();
    
    // Test initial state  
    assert_eq!(history_manager.history_count, 0);
    
    // Test adding entries
    history_manager.add_entry("🔍 Google", "https://google.com");
    history_manager.add_entry("👨‍💻 GitHub", "https://github.com");
    assert_eq!(history_manager.history_count, 2);
    
    // Test history display
    let display = history_manager.get_history_display();
    assert!(!display.is_empty());
    assert!(display.iter().any(|line| line.contains("👨‍💻 GitHub")));
    
    println!("✅ History tracking test passed");
    Ok(())
}

#[tokio::test]
async fn test_performance_optimizer() -> Result<()> {
    println!("🧪 Testing performance optimizer...");
    
    let optimizer = PerformanceOptimizer::new()?;
    
    // Test startup timing
    optimizer.start_startup_timer();
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    let startup_time = optimizer.finish_startup_timer();
    assert!(startup_time > tokio::time::Duration::from_millis(5));
    
    // Test tab lazy loading
    optimizer.lazy_load_tab("test_tab", "https://test.com", HWND(0)).await?;
    optimizer.activate_tab("test_tab").await?;
    
    // Test memory management
    optimizer.manage_memory().await?;
    
    println!("✅ Performance optimizer test passed");
    Ok(())
}

#[tokio::test]
async fn test_browser_features_integration() -> Result<()> {
    println!("🧪 Testing browser features integration...");
    
    let mut features_manager = SimpleBrowserFeaturesManager::new(HWND(0))?;
    
    // Test navigation with history tracking
    features_manager.navigate_to_url("https://test.com")?;
    assert_eq!(features_manager.tab_manager.current_url, "https://test.com");
    assert_eq!(features_manager.history_manager.history_count, 1);
    
    // Test new tab creation
    let tab_id = features_manager.create_new_tab();
    assert!(tab_id > 1);
    
    // Test status info
    let status = features_manager.get_status_info();
    assert!(status.contains_key("current_url"));
    assert!(status.contains_key("tab_count"));
    
    println!("✅ Browser features integration test passed");
    Ok(())
}

#[tokio::test]
async fn test_gui_loading() -> Result<()> {
    println!("🧪 Testing GUI loading...");
    
    let controller = WebView2Controller::new(HWND(0))?;
    
    // Test HTML GUI loading
    controller.load_html_gui()?;
    
    // Test HTML content access
    let html_content = controller.get_html_content();
    assert!(!html_content.is_empty());
    assert!(html_content.contains("html"));
    
    println!("✅ GUI loading test passed");
    Ok(())
}

#[tokio::test]
async fn test_security_features() -> Result<()> {
    println!("🧪 Testing security features...");
    
    // Test URL validation (basic test)
    let test_urls = vec![
        "https://google.com",
        "http://test.com", 
        "gui",
        "file:///test.html"
    ];
    
    for url in test_urls {
        // URL should be processable without panicking
        let controller = WebView2Controller::new(HWND(0))?;
        controller.navigate_to_url(url)?;
        assert!(!controller.get_current_url().is_empty());
    }
    
    println!("✅ Security features test passed");
    Ok(())
}

#[tokio::test] 
async fn test_comprehensive_browser_flow() -> Result<()> {
    println!("🧪 Testing comprehensive browser flow...");
    
    // Simulate complete browser session
    let mut features_manager = SimpleBrowserFeaturesManager::new(HWND(0))?;
    let optimizer = PerformanceOptimizer::new()?;
    
    // 1. Startup optimization
    optimizer.start_startup_timer();
    
    // 2. Navigate to sites
    features_manager.navigate_to_url("https://google.com")?;
    features_manager.navigate_to_url("https://github.com")?;
    
    // 3. Create new tabs
    let tab1 = features_manager.create_new_tab();
    let tab2 = features_manager.create_new_tab();
    assert!(tab1 > 0 && tab2 > tab1);
    
    // 4. Add bookmarks
    features_manager.bookmark_manager.add_bookmark("🧪 Test", "https://test.com");
    
    // 5. Performance optimization
    optimizer.lazy_load_tab("tab1", "https://site1.com", HWND(0)).await?;
    optimizer.activate_tab("tab1").await?;
    optimizer.manage_memory().await?;
    
    // 6. Verify state
    assert!(features_manager.history_manager.history_count >= 2);
    assert!(features_manager.bookmark_manager.bookmark_count >= 5);
    assert!(features_manager.tab_manager.tab_count >= 3);
    
    // 7. Cleanup
    features_manager.cleanup()?;
    optimizer.cleanup().await?;
    
    let startup_time = optimizer.finish_startup_timer();
    println!("⚡ Complete browser flow finished in: {:?}", startup_time);
    
    println!("✅ Comprehensive browser flow test passed");
    Ok(())
} 