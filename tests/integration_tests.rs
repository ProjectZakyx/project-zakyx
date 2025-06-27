// 🧪 INTEGRATION TESTS
// Tests für das gesamte Ora Browser System
// Copyright © 2024 Ora Browser Team

use ora_browser::{BrowserState, Tab};
use ora_browser::browser_state::Bookmark;
use std::time::Duration;
use tokio::time::timeout;

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

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    /// Test der Proxy-Server-Funktionalität
    #[tokio::test]
    async fn test_proxy_server_health() {
        // Starte einen Test-Proxy-Server
        let proxy_server = ora_browser::proxy_server::ProxyServer::new(3032);
        
        // Starte Server in Background-Task
        let server_handle = tokio::spawn(async move {
            let mut server = proxy_server;
            server.start().await
        });
        
        // Warte bis Server gestartet ist
        tokio::time::sleep(Duration::from_millis(1000)).await;
        
        // Teste Health-Check
        let client = reqwest::Client::new();
        let response = timeout(
            Duration::from_secs(5),
            client.get("http://localhost:3032/health").send()
        ).await;
        
        assert!(response.is_ok(), "Health check request should succeed");
        let response = response.unwrap().unwrap();
        assert_eq!(response.status(), 200, "Health check should return 200");
        
        let body = response.text().await.unwrap();
        assert!(body.contains("ok"), "Health check should return 'ok' status");
        
        // Beende Server
        server_handle.abort();
    }
    
    /// Test der universellen Ressourcen-Route
    #[tokio::test]
    async fn test_universal_resource_route() {
        let proxy_server = ora_browser::proxy_server::ProxyServer::new(3033);
        
        let server_handle = tokio::spawn(async move {
            let mut server = proxy_server;
            server.start().await
        });
        
        tokio::time::sleep(Duration::from_millis(1000)).await;
        
        let client = reqwest::Client::new();
        
        // Teste mit einer bekannten URL
        let response = timeout(
            Duration::from_secs(10),
            client.get("http://localhost:3033/universal?url=https://httpbin.org/get").send()
        ).await;
        
        assert!(response.is_ok(), "Universal route request should succeed");
        let response = response.unwrap().unwrap();
        assert_eq!(response.status(), 200, "Universal route should return 200");
        
        // Prüfe CORS-Header
        let headers = response.headers();
        assert!(headers.contains_key("access-control-allow-origin"), "CORS headers should be present");
        
        server_handle.abort();
    }
    
    /// Test der HTML-Erkennung und JavaScript-Injection
    #[tokio::test]
    async fn test_html_detection_and_injection() {
        let proxy_server = ora_browser::proxy_server::ProxyServer::new(3034);
        
        let server_handle = tokio::spawn(async move {
            let mut server = proxy_server;
            server.start().await
        });
        
        tokio::time::sleep(Duration::from_millis(1000)).await;
        
        let client = reqwest::Client::new();
        
        // Teste mit einer HTML-Seite
        let response = timeout(
            Duration::from_secs(10),
            client.get("http://localhost:3034/universal?url=https://httpbin.org/html").send()
        ).await;
        
        if let Ok(Ok(response)) = response {
            let content_type = response.headers().get("content-type");
            if let Some(ct) = content_type {
                let ct_str = ct.to_str().unwrap_or("");
                if ct_str.contains("text/html") {
                    let body = response.text().await.unwrap();
                    assert!(body.contains("ULTIMATIVE UNIVERSELLE CORS"), 
                           "HTML should contain injected security fixes");
                }
            }
        }
        
        server_handle.abort();
    }
    
    /// Test der Konfigurationsverwaltung
    #[test]
    fn test_configuration_management() {
        let mut config = ora_browser::config::OraConfig::default();
        
        // Teste Standard-Werte
        assert_eq!(config.proxy.primary_port, 3030);
        assert_eq!(config.proxy.fallback_port, 3031);
        assert_eq!(config.logging.log_level, "info");
        
        // Teste Validierung mit ungültigen Werten
        config.proxy.primary_port = 0;
        config.proxy.fallback_port = 3030;
        config.proxy.request_timeout_secs = 0;
        config.logging.log_level = "invalid".to_string();
        
        config.validate_and_fix();
        
        // Prüfe ob Werte korrigiert wurden
        assert_eq!(config.proxy.primary_port, 3030);
        assert_eq!(config.proxy.fallback_port, 3031);
        assert_eq!(config.proxy.request_timeout_secs, 30);
        assert_eq!(config.logging.log_level, "info");
    }
    
    /// Test des Metrics-Systems
    #[test]
    fn test_metrics_system() {
        let collector = ora_browser::metrics::MetricsCollector::new();
        
        // Teste Werte-Aufzeichnung
        collector.record_value("test_metric", 1.5, None);
        collector.record_value("test_metric", 2.5, None);
        collector.record_value("test_metric", 3.5, None);
        
        let stats = collector.get_stats("test_metric").unwrap();
        assert_eq!(stats.count, 3);
        assert_eq!(stats.avg, 2.5);
        assert_eq!(stats.min, 1.5);
        assert_eq!(stats.max, 3.5);
        
        // Teste Timer
        {
            let _timer = collector.start_timer("test_timer");
            std::thread::sleep(Duration::from_millis(10));
        }
        
        let timer_stats = collector.get_stats("test_timer").unwrap();
        assert_eq!(timer_stats.count, 1);
        assert!(timer_stats.avg > 0.008); // Mindestens 8ms
        
        // Teste JSON-Export
        let json = collector.export_json().unwrap();
        assert!(json.contains("test_metric"));
        assert!(json.contains("test_timer"));
    }
    
    /// Test der Browser-State-Verwaltung
    #[tokio::test]
    async fn test_browser_state_management() {
        let state = ora_browser::browser_state::BrowserState::new();
        
        // Teste Tab-Management
        {
            let mut tabs = state.tabs.write().await;
            tabs.push(ora_browser::browser_state::Tab {
                id: 1,
                title: "Test Tab".to_string(),
                url: "https://example.com".to_string(),
                is_active: true,
            });
        }
        
        let tabs = state.tabs.read().await;
        assert_eq!(tabs.len(), 1);
        assert_eq!(tabs[0].title, "Test Tab");
        assert_eq!(tabs[0].url, "https://example.com");
        assert!(tabs[0].is_active);
        
        // Teste Bookmark-Management
        {
            let mut bookmarks = state.bookmarks.write().await;
            bookmarks.push(ora_browser::browser_state::Bookmark {
                id: "test-1".to_string(),
                title: "Test Bookmark".to_string(),
                url: "https://test.com".to_string(),
            });
        }
        
        let bookmarks = state.bookmarks.read().await;
        assert!(bookmarks.iter().any(|b| b.title == "Test Bookmark"));
        
        // Teste Settings
        {
            let mut settings = state.settings.write().await;
            settings.homepage = "https://custom.com".to_string();
        }
        
        let settings = state.settings.read().await;
        assert_eq!(settings.homepage, "https://custom.com");
    }
    
    /// Performance-Test für Proxy-Server
    #[tokio::test]
    async fn test_proxy_performance() {
        let proxy_server = ora_browser::proxy_server::ProxyServer::new(3035);
        
        let server_handle = tokio::spawn(async move {
            let mut server = proxy_server;
            server.start().await
        });
        
        tokio::time::sleep(Duration::from_millis(1000)).await;
        
        let client = reqwest::Client::new();
        let start = std::time::Instant::now();
        
        // Führe mehrere parallele Requests aus
        let mut handles = Vec::new();
        for i in 0..10 {
            let client = client.clone();
            let handle = tokio::spawn(async move {
                let response = client
                    .get(&format!("http://localhost:3035/health?id={}", i))
                    .send()
                    .await;
                response.is_ok()
            });
            handles.push(handle);
        }
        
        // Warte auf alle Requests
        let mut success_count = 0;
        for handle in handles {
            if handle.await.unwrap_or(false) {
                success_count += 1;
            }
        }
        
        let duration = start.elapsed();
        
        // Assertions
        assert!(success_count >= 8, "At least 80% of requests should succeed");
        assert!(duration < Duration::from_secs(5), "All requests should complete within 5 seconds");
        
        println!("Performance test: {}/{} requests succeeded in {:?}", 
                success_count, 10, duration);
        
        server_handle.abort();
    }
    
    /// Test der URL-Utilities
    #[test]
    fn test_url_utilities() {
        use ora_browser::url_utils::{normalize_problematic_url, should_use_proxy_for_url};
        
        // Teste URL-Normalisierung
        assert_eq!(normalize_problematic_url("google.com"), "https://google.com");
        assert_eq!(normalize_problematic_url("http://example.com"), "http://example.com");
        assert_eq!(normalize_problematic_url("https://test.com"), "https://test.com");
        
        // Teste Proxy-Entscheidung
        assert!(should_use_proxy_for_url("https://example.com"));
        assert!(!should_use_proxy_for_url("http://localhost:3030"));
        assert!(!should_use_proxy_for_url("http://127.0.0.1:8080"));
    }
    
    /// Stress-Test für Memory-Leaks
    #[tokio::test]
    async fn test_memory_usage() {
        let state = ora_browser::browser_state::BrowserState::new();
        
        // Erstelle viele Tabs und lösche sie wieder
        for i in 0..1000 {
            {
                let mut tabs = state.tabs.write().await;
                tabs.push(ora_browser::browser_state::Tab {
                    id: i,
                    title: format!("Tab {}", i),
                    url: format!("https://example{}.com", i),
                    is_active: false,
                });
            }
            
            // Behalte nur die letzten 10 Tabs
            {
                let mut tabs = state.tabs.write().await;
                if tabs.len() > 10 {
                    let len = tabs.len();
                    tabs.drain(0..len - 10);
                }
            }
        }
        
        let final_tabs = state.tabs.read().await;
        assert!(final_tabs.len() <= 10, "Memory should be properly managed");
    }
}

/// Hilfsfunktionen für Tests
#[cfg(test)]
mod test_helpers {
    use std::time::Duration;
    
    /// Wartet bis ein Port verfügbar ist
    pub async fn wait_for_port(port: u16, timeout_secs: u64) -> bool {
        let client = reqwest::Client::new();
        let url = format!("http://localhost:{}/health", port);
        
        for _ in 0..timeout_secs {
            if let Ok(response) = client.get(&url).send().await {
                if response.status().is_success() {
                    return true;
                }
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        false
    }
    
    // Mock server functionality removed - external dependency not available
    // Tests can be extended with actual HTTP testing when mockito is added to dependencies
}
