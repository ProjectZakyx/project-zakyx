// 🌐 Navigation Commands
// Alle Commands für Navigation und URL-Handling

use tauri::Emitter;
#[allow(dead_code)] // Navigation API - some functions kept for completeness

use crate::browser_state::BrowserState;
use crate::internal_webview2_navigation::WebViewConfig;
use crate::url_utils::{normalize_problematic_url, should_use_proxy_for_url};
use crate::error::ZAKYXBrowserError;

/// Navigiere zu einer URL
#[tauri::command]
pub async fn navigate_to(
    state: tauri::State<'_, BrowserState>,
    tab_id: String,
    url: String,
    window: tauri::Window,
) -> Result<(), ZAKYXBrowserError> {
    let mut tabs = state.tabs.write().await;
    let mut history = state.history.write().await;
    
    println!("🌐 Backend starting navigation to: {}", url);
    
    let normalized_url = normalize_problematic_url(&url);
    let should_use_proxy = should_use_proxy_for_url(&normalized_url);
    
    if let Some(tab) = tabs.iter_mut().find(|tab| tab.id.to_string() == tab_id) {
        tab.url = normalized_url.clone();
        tab.is_active = true;
        tab.title = "Loading...".to_string();
        
        history.push(normalized_url.clone());
        
        let final_url = if should_use_proxy {
            format!("http://localhost:3030/proxy?url={}", urlencoding::encode(&normalized_url))
        } else {
            normalized_url.clone()
        };
        
        if let Err(e) = window.emit("webview_navigate", &final_url) {
            return Err(ZAKYXBrowserError::ui_error("navigation", &format!("Failed to emit navigation event: {}", e), true));
        }
        
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
            let _ = window.emit("webview_loaded", &serde_json::json!({
                "tab_id": tab_id,
                "url": final_url
            }));
        });
    }
    
    Ok(())
}

/// Interne WebView2-Navigation
#[tauri::command]
pub async fn internal_webview_navigate(
    state: tauri::State<'_, BrowserState>,
    tab_id: String,
    url: String,
    window: tauri::Window,
) -> Result<(), ZAKYXBrowserError> {
    let mut tabs = state.tabs.write().await;
    let mut history = state.history.write().await;
    
    println!("🌐 INTERNAL WEBVIEW NAVIGATION to: {}", url);
    
    if let Some(tab) = tabs.iter_mut().find(|tab| tab.id.to_string() == tab_id) {
        tab.url = url.clone();
        tab.is_active = true;
        tab.title = "Loading...".to_string();
        
        history.push(url.clone());
        
        window.emit("internal_webview_navigate", &serde_json::json!({
            "tab_id": tab_id,
            "url": url
        })).map_err(|e| ZAKYXBrowserError::ui_error("navigation", &format!("Failed to emit internal navigation event: {}", e), true))?;
        
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            let _ = window.emit("internal_webview_loaded", &serde_json::json!({
                "tab_id": tab_id,
                "url": url
            }));
        });
    }
    
    Ok(())
}

/// Navigiere intern
#[tauri::command]
pub async fn navigate_internally(
    state: tauri::State<'_, BrowserState>,
    tab_id: String,
    url: String,
    window: tauri::Window,
) -> Result<(), ZAKYXBrowserError> {
    let mut tabs = state.tabs.write().await;
    let mut history = state.history.write().await;
    
    println!("🔄 Internal navigation to: {}", url);
    
    if let Some(tab) = tabs.iter_mut().find(|tab| tab.id.to_string() == tab_id) {
        tab.url = url.clone();
        tab.is_active = true;
        tab.title = "Loading...".to_string();
        
        history.push(url.clone());
        
        let should_navigate = {
            let webview_navigator = state.webview_navigator.read().await;
            webview_navigator.should_navigate_internally(&url)
        };
        
        if should_navigate {
            println!("✅ Internal navigation successful");
            
            window.emit("navigation_success", &serde_json::json!({
                "tab_id": tab_id,
                "url": url
            })).map_err(|e| ZAKYXBrowserError::ui_error("navigation", &format!("Failed to emit navigation success event: {}", e), true))?;
        } else {
            println!("❌ Internal navigation failed, falling back to external");
            drop(tabs);
            drop(history);
            return navigate_to(state, tab_id, url, window).await;
        }
    }
    
    Ok(())
}

/// Navigiere und hole Content
#[tauri::command]
pub async fn navigate_and_get_content(
    _state: tauri::State<'_, BrowserState>,
    url: String,
) -> Result<String, ZAKYXBrowserError> {
    println!("🔍 Loading content from: {}", url);
    
    match load_url_content(&url).await {
        Ok(content) => {
            println!("✅ Successfully loaded content from: {}", url);
            let enhanced_content = inject_cookie_acceptance_script(&content);
            Ok(enhanced_content)
        }
        Err(e) => {
            println!("❌ Failed to load content from {}: {}", url, e);
            let error_page = format!(
                r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Loading Error</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 0;
            padding: 40px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            text-align: center;
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }}
        .error-container {{
            background: rgba(255, 255, 255, 0.1);
            padding: 40px;
            border-radius: 20px;
            backdrop-filter: blur(10px);
            max-width: 500px;
        }}
        h1 {{ margin-bottom: 20px; }}
        .url {{ color: #ffeb3b; word-break: break-all; margin: 20px 0; }}
        .error {{ color: #ff5722; font-size: 14px; }}
        button {{
            background: white;
            color: #667eea;
            border: none;
            padding: 12px 24px;
            border-radius: 25px;
            cursor: pointer;
            margin-top: 20px;
            font-weight: bold;
        }}
    </style>
</head>
<body>
    <div class="error-container">
        <h1>🚫 Could not load page</h1>
        <div class="url">{}</div>
        <div class="error">Error: {}</div>
        <button onclick="window.location.reload()">🔄 Try Again</button>
        <button onclick="history.back()">⬅️ Go Back</button>
    </div>
</body>
</html>"#,
                url, e
            );
            
            Ok(error_page)
        }
    }
}

/// Prüfe interne Navigation
#[tauri::command]
pub async fn check_internal_navigation(
    state: tauri::State<'_, BrowserState>,
    url: String,
) -> Result<bool, ZAKYXBrowserError> {
    let webview_navigator = state.webview_navigator.read().await;
    Ok(webview_navigator.should_navigate_internally(&url))
}

/// Hole WebView-Konfiguration
#[tauri::command]
pub async fn get_webview_config(
    state: tauri::State<'_, BrowserState>,
    url: String,
) -> Result<WebViewConfig, ZAKYXBrowserError> {
    let webview_navigator = state.webview_navigator.read().await;
    Ok(webview_navigator.get_config_for_url(&url))
}

/// Hole Proxy-URL
#[tauri::command]
pub async fn get_proxy_url(url: String) -> Result<String, ZAKYXBrowserError> {
    let proxy_url = format!("http://localhost:3030/proxy?url={}", urlencoding::encode(&url));
    Ok(proxy_url)
}

/// Öffne externe URL
#[tauri::command]
pub async fn open_external_url(url: String) -> Result<(), ZAKYXBrowserError> {
    println!("🌐 Opening external URL: {}", url);
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", &url])
            .spawn()
            .map_err(|e| ZAKYXBrowserError::ui_error("navigation", &format!("Failed to open URL on Windows: {}", e), false))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| ZAKYXBrowserError::ui_error("navigation", &format!("Failed to open URL on macOS: {}", e), false))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| ZAKYXBrowserError::ui_error("navigation", &format!("Failed to open URL on Linux: {}", e), false))?;
    }
    
    println!("✅ External URL opened successfully");
    Ok(())
}

/// Lade URL-Content
async fn load_url_content(url: &str) -> Result<String, ZAKYXBrowserError> {
    println!("🔄 Loading URL content: {}", url);
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()
        .map_err(|e| ZAKYXBrowserError::network_error(&format!("Failed to create HTTP client: {}", e), Some(url)))?;
    
    let response = client.get(url).send().await
        .map_err(|e| ZAKYXBrowserError::network_error(&format!("Failed to fetch URL: {}", e), Some(url)))?;
    
    let content = response.text().await
        .map_err(|e| ZAKYXBrowserError::network_error(&format!("Failed to read response body: {}", e), Some(url)))?;
    
    println!("✅ Successfully loaded {} characters from {}", content.len(), url);
    Ok(content)
}

/// Injiziere Cookie-Akzeptanz-Script
fn inject_cookie_acceptance_script(html: &str) -> String {
    let cookie_script = r#"
<script>
// 🍪 Enhanced Cookie Acceptance & Site Compatibility
console.log('🍪 Enhanced Cookie Script Loaded');

// Auto-accept common cookie banners
setTimeout(() => {
    // Common cookie banner selectors
    const cookieSelectors = [
        '[data-testid="cookie-banner"] button[data-testid="accept-all"]',
        '.cookie-banner .accept-all',
        '.cookie-consent .accept',
        '#cookie-banner .accept',
        '.gdpr-banner .accept-all',
        '[class*="cookie"] [class*="accept"]',
        '[id*="cookie"] [id*="accept"]'
    ];
    
    for (const selector of cookieSelectors) {
        const button = document.querySelector(selector);
        if (button) {
            console.log('🍪 Auto-accepting cookies via:', selector);
            button.click();
            break;
        }
    }
}, 1000);

// Enhance form compatibility
document.addEventListener('DOMContentLoaded', () => {
    // Fix common input issues
    const inputs = document.querySelectorAll('input[type="text"], input[type="email"], input[type="password"]');
    inputs.forEach(input => {
        input.addEventListener('focus', () => {
            // Remove any autocomplete restrictions
            input.removeAttribute('autocomplete');
        });
    });
});
</script>
"#;
    
    // Inject script before closing head tag
    if let Some(head_end) = html.find("</head>") {
        let mut result = html.to_string();
        result.insert_str(head_end, cookie_script);
        result
    } else {
        // If no head tag, inject at the beginning
        format!("{}\n{}", cookie_script, html)
    }
}

/// Navigation Utilities
pub struct NavigationUtils;

impl NavigationUtils {
    /// Prüfe ob URL gültig ist
    pub fn is_valid_url(url: &str) -> bool {
        url::Url::parse(url).is_ok()
    }
    
    /// Extrahiere Domain aus URL
    pub fn extract_domain(url: &str) -> Option<String> {
        url::Url::parse(url)
            .ok()?
            .host_str()
            .map(|s| s.to_string())
    }
    
    /// Erstelle sichere URL
    pub fn create_safe_url(url: &str) -> String {
        if Self::is_valid_url(url) {
            url.to_string()
        } else if !url.starts_with("http://") && !url.starts_with("https://") {
            format!("https://{}", url)
        } else {
            "about:blank".to_string()
        }
    }
    
    /// Hole Titel aus URL
    pub fn get_title_from_url(url: &str) -> String {
        if let Ok(parsed) = url::Url::parse(url) {
            if let Some(host) = parsed.host_str() {
                let mut chars = host.chars();
                match chars.next() {
                    None => "Unknown".to_string(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            } else {
                "Unknown".to_string()
            }
        } else {
            "Unknown".to_string()
        }
    }
} 
