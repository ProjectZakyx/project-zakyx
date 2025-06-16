// 🎯 TAURI COMMANDS

use tauri::{Emitter};
use crate::browser_state::{BrowserState, Tab, Bookmark, BrowserSettings};
use crate::internal_webview2_navigation::WebViewConfig;
use crate::url_utils::{normalize_problematic_url, should_use_proxy_for_url};

// 📑 TAB MANAGEMENT COMMANDS

#[tauri::command]
pub async fn create_new_tab(
    state: tauri::State<'_, BrowserState>,
    url: Option<String>,
) -> Result<Tab, String> {
    let mut tabs = state.tabs.write().await;
    
    // Deactivate all existing tabs
    for tab in tabs.iter_mut() {
        tab.is_active = false;
    }
    
    let new_tab = Tab {
        id: uuid::Uuid::new_v4().to_string().parse::<u32>().unwrap(),
        title: "New Tab".to_string(),
        url: url.unwrap_or_else(|| "about:blank".to_string()),
        is_active: true,
    };
    
    tabs.push(new_tab.clone());
    
    println!("📑 New tab created: {}", new_tab.id);
    Ok(new_tab)
}

#[tauri::command]
pub async fn close_tab(
    state: tauri::State<'_, BrowserState>,
    tab_id: String,
) -> Result<(), String> {
    let mut tabs = state.tabs.write().await;
    
    if let Some(pos) = tabs.iter().position(|tab| tab.id.to_string() == tab_id) {
        tabs.remove(pos);
        println!("❌ Tab closed: {}", tab_id);
        
        // Activate another tab if available
        if !tabs.is_empty() && !tabs.iter().any(|tab| tab.is_active) {
            tabs[0].is_active = true;
        }
    }
    
    Ok(())
}

#[tauri::command]
pub async fn get_tabs(state: tauri::State<'_, BrowserState>) -> Result<Vec<Tab>, String> {
    let tabs = state.tabs.read().await;
    Ok(tabs.clone())
}

#[tauri::command]
pub async fn update_tab_title(
    state: tauri::State<'_, BrowserState>,
    tab_id: String,
    title: String,
) -> Result<(), String> {
    let mut tabs = state.tabs.write().await;
    
    if let Some(tab) = tabs.iter_mut().find(|tab| tab.id.to_string() == tab_id) {
        tab.title = title.clone();
        println!("📝 Tab title updated: {} -> {}", tab_id, title);
    }
    
    Ok(())
}

// 🌐 NAVIGATION COMMANDS

#[tauri::command]
pub async fn navigate_to(
    state: tauri::State<'_, BrowserState>,
    tab_id: String,
    url: String,
    window: tauri::Window,
) -> Result<(), String> {
    let mut tabs = state.tabs.write().await;
    let mut history = state.history.write().await;
    
    println!("🌐 Backend starting navigation to: {}", url);
    
    // Spezielle Behandlung für problematische URLs
    let normalized_url = normalize_problematic_url(&url);
    let should_use_proxy = should_use_proxy_for_url(&normalized_url);
    
    if let Some(tab) = tabs.iter_mut().find(|tab| tab.id.to_string() == tab_id) {
        tab.url = normalized_url.clone();
        tab.is_active = true;
        tab.title = "Loading...".to_string();
        
        // Add to history
        history.push(normalized_url.clone());
        
        println!("🔄 Backend processing navigation for: {}", normalized_url);
        
        // Lade den Inhalt direkt im Backend
        let final_url = if should_use_proxy {
            println!("🔄 Backend using proxy navigation for: {}", normalized_url);
            format!("http://localhost:3030/proxy?url={}", urlencoding::encode(&normalized_url))
        } else {
            println!("🌐 Backend using direct navigation for: {}", normalized_url);
            normalized_url.clone()
        };
        
        // Sende sofortiges Navigation Event
        println!("📡 Backend sending immediate webview_navigate event: {}", final_url);
        if let Err(e) = window.emit("webview_navigate", &final_url) {
            println!("❌ Failed to emit webview_navigate event: {}", e);
            return Err(format!("Failed to emit navigation event: {}", e));
        }
        
        // Simulate loading completion after a short delay
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

#[tauri::command]
pub async fn internal_webview_navigate(
    state: tauri::State<'_, BrowserState>,
    tab_id: String,
    url: String,
    window: tauri::Window,
) -> Result<(), String> {
    let mut tabs = state.tabs.write().await;
    let mut history = state.history.write().await;
    
    println!("🌐 INTERNAL WEBVIEW NAVIGATION to: {}", url);
    
    if let Some(tab) = tabs.iter_mut().find(|tab| tab.id.to_string() == tab_id) {
        tab.url = url.clone();
        tab.is_active = true;
        tab.title = "Loading...".to_string();
        
        // Add to history
        history.push(url.clone());
        
        // Emit event to frontend to load in internal WebView2
        window.emit("internal_webview_navigate", &serde_json::json!({
            "tab_id": tab_id,
            "url": url
        })).map_err(|e| e.to_string())?;
        
        println!("✅ Internal WebView2 navigation initiated for tab {} to: {}", tab_id, url);
        
        // Simulate loading completion after a short delay
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

#[tauri::command]
pub async fn navigate_internally(
    state: tauri::State<'_, BrowserState>,
    tab_id: String,
    url: String,
    window: tauri::Window,
) -> Result<(), String> {
    let mut tabs = state.tabs.write().await;
    let mut history = state.history.write().await;
    
    println!("🔄 Internal navigation to: {}", url);
    
    if let Some(tab) = tabs.iter_mut().find(|tab| tab.id.to_string() == tab_id) {
        tab.url = url.clone();
        tab.is_active = true;
        tab.title = "Loading...".to_string();
        
        // Add to history
        history.push(url.clone());
        
        // Use internal WebView2 navigation
        let webview_navigator = state.webview_navigator.read().await;
        if webview_navigator.should_navigate_internally(&url) {
            println!("✅ Internal navigation successful");
            
            // Emit success event
            window.emit("internal_navigation_success", &serde_json::json!({
                "tab_id": tab_id,
                "url": url
            })).map_err(|e| e.to_string())?;
        } else {
            println!("❌ Internal navigation not supported for this URL");
            return Err("Internal navigation not supported for this URL".to_string());
        }
    }
    
    Ok(())
}

#[tauri::command]
pub async fn navigate_and_get_content(
    _state: tauri::State<'_, BrowserState>,
    url: String,
) -> Result<String, String> {
    println!("🌐 Backend fetching content for: {}", url);
    
    // Normalisiere die URL
    let normalized_url = normalize_problematic_url(&url);
    let should_use_proxy = should_use_proxy_for_url(&normalized_url);
    
    if should_use_proxy {
        println!("🔄 Using proxy for: {}", normalized_url);
        // Nutze das Smart-Proxy-System
        match crate::smart_proxy::SmartProxy::fetch_and_strip_headers(&normalized_url).await {
            Ok(response) => {
                println!("✅ Successfully fetched content via proxy: {} bytes", response.content.len());
                Ok(response.content)
            }
            Err(e) => {
                println!("❌ Proxy fetch failed: {}", e);
                Err(format!("Failed to fetch content: {}", e))
            }
        }
    } else {
        println!("🌐 Using direct fetch for: {}", normalized_url);
        // Direkter Fetch für sichere URLs
        match load_url_content(&normalized_url).await {
            Ok(content) => {
                println!("✅ Successfully fetched content directly: {} bytes", content.len());
                Ok(content)
            }
            Err(e) => {
                println!("❌ Direct fetch failed: {}", e);
                Err(format!("Failed to fetch content: {}", e))
            }
        }
    }
}

// 🔖 BOOKMARK COMMANDS

#[tauri::command]
pub async fn add_bookmark(
    state: tauri::State<'_, BrowserState>,
    title: String,
    url: String,
) -> Result<Bookmark, String> {
    let mut bookmarks = state.bookmarks.write().await;
    let mut bookmark_manager = state.bookmark_manager.write().await;
    
    let new_bookmark = Bookmark {
        id: uuid::Uuid::new_v4().to_string(),
        title: title.clone(),
        url: url.clone(),
    };
    
    // Add to in-memory bookmarks
    bookmarks.push(new_bookmark.clone());
    
    // Add to persistent bookmark manager
    let bookmark_id = bookmark_manager.add_bookmark(&title, &url);
    println!("🔖 Bookmark added with ID: {}", bookmark_id);
    
    println!("🔖 Bookmark added: {} -> {}", title, url);
    Ok(new_bookmark)
}

#[tauri::command]
pub async fn get_bookmarks(state: tauri::State<'_, BrowserState>) -> Result<Vec<Bookmark>, String> {
    let bookmarks = state.bookmarks.read().await;
    Ok(bookmarks.clone())
}

#[tauri::command]
pub async fn remove_bookmark(
    state: tauri::State<'_, BrowserState>,
    bookmark_id: String,
) -> Result<(), String> {
    let mut bookmarks = state.bookmarks.write().await;
    let bookmark_manager = state.bookmark_manager.read().await;
    
    if let Some(pos) = bookmarks.iter().position(|b| b.id == bookmark_id) {
        let bookmark = &bookmarks[pos];
        
        // Find bookmark by URL in persistent storage
        let all_bookmarks = bookmark_manager.get_bookmarks();
        if let Some(persistent_bookmark) = all_bookmarks.iter().find(|b| b.url == bookmark.url) {
            // Remove from persistent storage using the correct ID
            let mut bookmark_manager_mut = state.bookmark_manager.write().await;
            if !bookmark_manager_mut.remove_bookmark(persistent_bookmark.id) {
                println!("❌ Failed to remove bookmark from persistent storage");
                return Err("Failed to remove bookmark from persistent storage".to_string());
            }
        }
        
        // Remove from in-memory storage
        bookmarks.remove(pos);
        println!("🗑️ Bookmark removed: {}", bookmark_id);
    }
    
    Ok(())
}

// ⚙️ SETTINGS COMMANDS

#[tauri::command]
pub async fn get_settings(state: tauri::State<'_, BrowserState>) -> Result<BrowserSettings, String> {
    let settings = state.settings.read().await;
    Ok(settings.clone())
}

#[tauri::command]
pub async fn update_settings(
    state: tauri::State<'_, BrowserState>,
    new_settings: BrowserSettings,
) -> Result<(), String> {
    let mut settings = state.settings.write().await;
    *settings = new_settings;
    println!("⚙️ Settings updated");
    Ok(())
}

// 📚 HISTORY COMMANDS

#[tauri::command]
pub async fn get_history(state: tauri::State<'_, BrowserState>) -> Result<Vec<String>, String> {
    let history = state.history.read().await;
    Ok(history.clone())
}

// 🔧 UTILITY COMMANDS

#[tauri::command]
pub async fn check_internal_navigation(
    state: tauri::State<'_, BrowserState>,
    url: String,
) -> Result<bool, String> {
    let webview_navigator = state.webview_navigator.read().await;
    Ok(webview_navigator.should_navigate_internally(&url))
}

#[tauri::command]
pub async fn get_webview_config(
    state: tauri::State<'_, BrowserState>,
    url: String,
) -> Result<WebViewConfig, String> {
    let webview_navigator = state.webview_navigator.read().await;
    Ok(webview_navigator.get_config_for_url(&url))
}

#[tauri::command]
pub async fn get_proxy_url(url: String) -> Result<String, String> {
    let proxy_url = format!("http://localhost:3030/proxy?url={}", urlencoding::encode(&url));
    println!("🔄 Generated proxy URL: {}", proxy_url);
    Ok(proxy_url)
}

#[tauri::command]
pub async fn open_external_url(url: String) -> Result<(), String> {
    println!("🌐 Opening external URL: {}", url);
    
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        match Command::new("cmd")
            .args(&["/C", "start", &url])
            .spawn()
        {
            Ok(_) => {
                println!("✅ External URL opened successfully");
                Ok(())
            }
            Err(e) => {
                println!("❌ Failed to open external URL: {}", e);
                Err(format!("Failed to open URL: {}", e))
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        match Command::new("open")
            .arg(&url)
            .spawn()
        {
            Ok(_) => {
                println!("✅ External URL opened successfully");
                Ok(())
            }
            Err(e) => {
                println!("❌ Failed to open external URL: {}", e);
                Err(format!("Failed to open URL: {}", e))
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        match Command::new("xdg-open")
            .arg(&url)
            .spawn()
        {
            Ok(_) => {
                println!("✅ External URL opened successfully");
                Ok(())
            }
            Err(e) => {
                println!("❌ Failed to open external URL: {}", e);
                Err(format!("Failed to open URL: {}", e))
            }
        }
    }
}

// 🔧 HELPER FUNCTIONS

async fn load_url_content(url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    println!("🌐 Loading URL content with enhanced cookie handling: {}", url);
    
    // Erstelle einen Client mit realistischen Headers (ohne cookie_store da nicht verfügbar)
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;
    
    // Erste Anfrage mit vollständigen Headers
    let response = client
        .get(url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8")
        .header("Accept-Language", "de-DE,de;q=0.9,en;q=0.8")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("DNT", "1")
        .header("Connection", "keep-alive")
        .header("Upgrade-Insecure-Requests", "1")
        .header("Sec-Fetch-Dest", "document")
        .header("Sec-Fetch-Mode", "navigate")
        .header("Sec-Fetch-Site", "none")
        .header("Sec-Fetch-User", "?1")
        .send()
        .await?;
    
    let mut content = response.text().await?;
    
    // Cookie-Banner automatisch akzeptieren durch JavaScript-Injection
    content = inject_cookie_acceptance_script(&content);
    
    println!("✅ Content loaded with cookie handling: {} bytes", content.len());
    Ok(content)
}

fn inject_cookie_acceptance_script(html: &str) -> String {
    let cookie_script = r#"
<script>
// 🍪 AUTOMATISCHE COOKIE-BANNER BEHANDLUNG
(function() {
    console.log('🍪 Ora Browser: Cookie-Banner Auto-Handler gestartet');
    
    // Häufige Cookie-Banner Selektoren
    const cookieSelectors = [
        // Allgemeine Selektoren
        '[id*="cookie" i][id*="accept" i]',
        '[class*="cookie" i][class*="accept" i]',
        '[id*="consent" i][id*="accept" i]',
        '[class*="consent" i][class*="accept" i]',
        
        // Spezifische Selektoren für bekannte Cookie-Banner
        '#onetrust-accept-btn-handler',
        '.onetrust-close-btn-handler',
        '[data-testid="uc-accept-all-button"]',
        '[data-testid="cookie-accept-all"]',
        '.cookie-consent-accept',
        '.gdpr-accept-all',
        '.consent-accept-all',
        '.cookie-banner-accept',
        
        // Text-basierte Selektoren
        'button[aria-label*="Accept" i]',
        'button[title*="Accept" i]',
        'button:contains("Accept All")',
        'button:contains("Alle akzeptieren")',
        'button:contains("Akzeptieren")',
        'button:contains("Zustimmen")',
        'button:contains("OK")',
        'button:contains("Agree")',
        
        // Weitere häufige Patterns
        '.cc-allow',
        '.cc-dismiss',
        '.cookie-notice-accept',
        '.privacy-accept',
        '.gdpr-banner-accept'
    ];
    
    function findAndClickCookieButton() {
        for (const selector of cookieSelectors) {
            try {
                const elements = document.querySelectorAll(selector);
                for (const element of elements) {
                    if (element && element.offsetParent !== null) { // Element ist sichtbar
                        console.log('🍪 Cookie-Button gefunden:', selector, element);
                        element.click();
                        return true;
                    }
                }
            } catch (e) {
                // Ignoriere Fehler bei ungültigen Selektoren
            }
        }
        return false;
    }
    
    // Sofort versuchen
    setTimeout(() => {
        if (findAndClickCookieButton()) {
            console.log('✅ Cookie-Banner automatisch akzeptiert');
        }
    }, 500);
    
    // Nochmal nach 2 Sekunden versuchen (für langsam ladende Banner)
    setTimeout(() => {
        if (findAndClickCookieButton()) {
            console.log('✅ Cookie-Banner automatisch akzeptiert (verzögert)');
        }
    }, 2000);
    
    // Observer für dynamisch geladene Cookie-Banner
    const observer = new MutationObserver((mutations) => {
        for (const mutation of mutations) {
            if (mutation.type === 'childList' && mutation.addedNodes.length > 0) {
                setTimeout(() => {
                    if (findAndClickCookieButton()) {
                        console.log('✅ Cookie-Banner automatisch akzeptiert (dynamisch)');
                    }
                }, 100);
            }
        }
    });
    
    observer.observe(document.body, {
        childList: true,
        subtree: true
    });
    
    // Observer nach 30 Sekunden stoppen
    setTimeout(() => {
        observer.disconnect();
        console.log('🍪 Cookie-Banner Observer gestoppt');
    }, 30000);
})();
</script>
"#;
    
    // Füge das Script vor dem schließenden </body> Tag ein
    if let Some(body_end) = html.rfind("</body>") {
        let mut result = html.to_string();
        result.insert_str(body_end, cookie_script);
        result
    } else if let Some(html_end) = html.rfind("</html>") {
        let mut result = html.to_string();
        result.insert_str(html_end, &format!("<body>{}</body>", cookie_script));
        result
    } else {
        // Fallback: Füge das Script am Ende hinzu
        format!("{}{}", html, cookie_script)
    }
}

#[allow(dead_code)]
#[allow(dead_code)]
fn get_title_from_url(url: &str) -> String {
    if let Ok(parsed_url) = url::Url::parse(url) {
        if let Some(host) = parsed_url.host_str() {
            // Capitalize first letter and remove common prefixes
            let clean_host = host
                .strip_prefix("www.")
                .unwrap_or(host)
                .split('.')
                .next()
                .unwrap_or(host);
            
            let mut chars: Vec<char> = clean_host.chars().collect();
            if !chars.is_empty() {
                chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
            }
            chars.into_iter().collect()
        } else {
            "New Tab".to_string()
        }
    } else {
        "New Tab".to_string()
    }
}

// 🔌 PLUGIN COMMANDS

#[tauri::command]
pub async fn get_all_plugins(
    state: tauri::State<'_, BrowserState>,
) -> Result<Vec<crate::plugin_manager::PluginInfo>, String> {
    let plugin_manager = state.plugin_manager.read().await;
    Ok(plugin_manager.get_all_plugins().into_iter().cloned().collect())
}

#[tauri::command]
pub async fn get_loaded_plugins(
    state: tauri::State<'_, BrowserState>,
) -> Result<Vec<crate::plugin_manager::PluginInfo>, String> {
    let plugin_manager = state.plugin_manager.read().await;
    Ok(plugin_manager.get_loaded_plugins().into_iter().cloned().collect())
}

#[tauri::command]
pub async fn enable_plugin(
    state: tauri::State<'_, BrowserState>,
    plugin_id: String,
) -> Result<(), String> {
    let mut plugin_manager = state.plugin_manager.write().await;
    plugin_manager.enable_plugin(&plugin_id)
}

#[tauri::command]
pub async fn disable_plugin(
    state: tauri::State<'_, BrowserState>,
    plugin_id: String,
) -> Result<(), String> {
    let mut plugin_manager = state.plugin_manager.write().await;
    plugin_manager.disable_plugin(&plugin_id)
}

#[tauri::command]
pub async fn load_plugin(
    state: tauri::State<'_, BrowserState>,
    plugin_id: String,
) -> Result<(), String> {
    let mut plugin_manager = state.plugin_manager.write().await;
    plugin_manager.load_plugin(&plugin_id)
}

#[tauri::command]
pub async fn unload_plugin(
    state: tauri::State<'_, BrowserState>,
    plugin_id: String,
) -> Result<(), String> {
    let mut plugin_manager = state.plugin_manager.write().await;
    plugin_manager.unload_plugin(&plugin_id)
}
