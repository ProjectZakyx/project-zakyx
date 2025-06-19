// 🌐 INTERNAL WEBVIEW2 NAVIGATION SYSTEM
// Handles internal navigation within the app instead of opening external browser
// Copyright © 2024 Ora Browser Team

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebViewConfig {
    pub enable_javascript: bool,
    pub enable_cookies: bool,
    pub enable_local_storage: bool,
    pub user_agent: String,
    pub disable_web_security: bool,
    pub allow_running_insecure_content: bool,
}

impl Default for WebViewConfig {
    fn default() -> Self {
        Self {
            enable_javascript: true,
            enable_cookies: true,
            enable_local_storage: true,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 OraBrowser/1.0".to_string(),
            disable_web_security: false,
            allow_running_insecure_content: false,
        }
    }
}

#[derive(Debug)]
pub struct InternalWebView2Navigator {
    site_specific_configs: HashMap<String, WebViewConfig>,
    blocked_urls: Vec<String>,
}

impl InternalWebView2Navigator {
    pub fn new() -> Self {
        let mut navigator = Self {
            site_specific_configs: HashMap::new(),
            blocked_urls: vec![],
        };
        
        navigator.setup_site_specific_configs();
        navigator
    }
    
    fn setup_site_specific_configs(&mut self) {
        // Google.com specific configuration
        let google_config = WebViewConfig {
            enable_javascript: true,
            enable_cookies: true,
            enable_local_storage: true,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            disable_web_security: false,
            allow_running_insecure_content: false,
        };
        
        self.site_specific_configs.insert("google.com".to_string(), google_config.clone());
        self.site_specific_configs.insert("www.google.com".to_string(), google_config.clone());
        self.site_specific_configs.insert("google.de".to_string(), google_config.clone());
        self.site_specific_configs.insert("www.google.de".to_string(), google_config);
        
        // GitHub.com configuration
        let github_config = WebViewConfig {
            enable_javascript: true,
            enable_cookies: true,
            enable_local_storage: true,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            disable_web_security: false,
            allow_running_insecure_content: false,
        };
        
        self.site_specific_configs.insert("github.com".to_string(), github_config.clone());
        self.site_specific_configs.insert("www.github.com".to_string(), github_config);
        
        // YouTube.com configuration - ERWEITERT FÜR CORS-KOMPATIBILITÄT
        let youtube_config = WebViewConfig {
            enable_javascript: true,
            enable_cookies: true,
            enable_local_storage: true,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            disable_web_security: true, // CORS-Probleme beheben
            allow_running_insecure_content: true, // Für Video-Playback und Mixed Content
        };
        
        self.site_specific_configs.insert("youtube.com".to_string(), youtube_config.clone());
        self.site_specific_configs.insert("www.youtube.com".to_string(), youtube_config.clone());
        self.site_specific_configs.insert("m.youtube.com".to_string(), youtube_config.clone());
        self.site_specific_configs.insert("music.youtube.com".to_string(), youtube_config.clone());
        
        // Weitere Video-Plattformen mit ähnlichen Einstellungen
        let video_config = WebViewConfig {
            enable_javascript: true,
            enable_cookies: true,
            enable_local_storage: true,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            disable_web_security: true,
            allow_running_insecure_content: true,
        };
        
        // Video-Plattformen
        self.site_specific_configs.insert("vimeo.com".to_string(), video_config.clone());
        self.site_specific_configs.insert("www.vimeo.com".to_string(), video_config.clone());
        self.site_specific_configs.insert("twitch.tv".to_string(), video_config.clone());
        self.site_specific_configs.insert("www.twitch.tv".to_string(), video_config.clone());
        self.site_specific_configs.insert("dailymotion.com".to_string(), video_config.clone());
        self.site_specific_configs.insert("www.dailymotion.com".to_string(), video_config.clone());
    }
    
    pub fn get_config_for_url(&self, url: &str) -> WebViewConfig {
        if let Ok(parsed_url) = url::Url::parse(url) {
            if let Some(domain) = parsed_url.domain() {
                if let Some(config) = self.site_specific_configs.get(domain) {
                    return config.clone();
                }
            }
        }
        
        WebViewConfig::default()
    }
    
    pub fn should_navigate_internally(&self, url: &str) -> bool {
        // Check if URL is explicitly blocked
        for blocked in &self.blocked_urls {
            if url.contains(blocked) {
                return false;
            }
        }
        
        // Parse URL and check domain
        if let Ok(parsed_url) = url::Url::parse(url) {
            if let Some(domain) = parsed_url.domain() {
                let domain_lower = domain.to_lowercase();
                
                // Block domains that are known to have X-Frame-Options issues
                let problematic_domains = [
                    // These domains typically block iframe embedding
                    "facebook.com", "www.facebook.com",
                    "twitter.com", "www.twitter.com", "x.com", "www.x.com",
                    "instagram.com", "www.instagram.com",
                    "linkedin.com", "www.linkedin.com",
                    "amazon.com", "www.amazon.com", "amazon.de", "www.amazon.de",
                    "ebay.com", "www.ebay.com", "ebay.de", "www.ebay.de",
                    "paypal.com", "www.paypal.com",
                    "netflix.com", "www.netflix.com",
                    "spotify.com", "www.spotify.com",
                    "twitch.tv", "www.twitch.tv"
                ];
                
                for blocked_domain in &problematic_domains {
                    if domain_lower == *blocked_domain || domain_lower.ends_with(&format!(".{}", blocked_domain)) {
                        println!("🚫 Domain {} blocked due to X-Frame-Options restrictions", domain_lower);
                        return false;
                    }
                }
            }
        }
        
        // Default to internal navigation for all HTTP/HTTPS URLs
        // This is the new behavior: try internal first, fallback to external if needed
        if url.starts_with("https://") || url.starts_with("http://") {
            println!("✅ URL {} will be handled internally", url);
            return true;
        }
        
        // Handle special URLs
        if url == "gui" || url.starts_with("about:") || url.starts_with("data:") {
            return true;
        }
        
        // Default to external for unknown protocols
        println!("❌ URL {} will be handled externally (unknown protocol)", url);
        false
    }
    
    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn prepare_navigation_html(&self, url: &str, _proxy_url: &str, _user_agent: &str) -> String {
        let config = self.get_config_for_url(url);
        
        format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ora Browser - Loading {}</title>
    <style>
        body, html {{
            margin: 0;
            padding: 0;
            width: 100%;
            height: 100%;
            overflow: hidden;
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        }}
        
        .webview-frame {{
            width: 100%;
            height: 100%;
            border: none;
            display: block;
        }}
        
        .loading-overlay {{
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: #f5f5f5;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            z-index: 9999;
            font-size: 16px;
            color: #333;
        }}
        
        .loading-spinner {{
            width: 40px;
            height: 40px;
            border: 4px solid #e0e0e0;
            border-top: 4px solid #4285f4;
            border-radius: 50%;
            animation: spin 1s linear infinite;
            margin-bottom: 20px;
        }}
        
        @keyframes spin {{
            0% {{ transform: rotate(0deg); }}
            100% {{ transform: rotate(360deg); }}
        }}
        
        .url-info {{
            margin-top: 10px;
            color: #666;
            font-size: 14px;
        }}
    </style>
</head>
<body>
    <div class="loading-overlay" id="loadingOverlay">
        <div class="loading-spinner"></div>
        <div>Loading {}</div>
        <div class="url-info">Connecting to site...</div>
    </div>
    
    <iframe 
        id="webviewFrame"
        class="webview-frame"
        src="{}"
        sandbox="allow-scripts allow-forms allow-popups allow-top-navigation allow-downloads allow-pointer-lock allow-presentation allow-modals allow-same-origin"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; microphone; camera; fullscreen">
    </iframe>
    
    <script>
        console.log('🌐 Internal WebView2 Navigation initialized for: {}');
        console.log('🔧 Configuration:', {});
        
        const frame = document.getElementById('webviewFrame');
        const overlay = document.getElementById('loadingOverlay');
        
        // Hide loading overlay when iframe loads
        frame.onload = function() {{
            console.log('✅ WebView frame loaded successfully');
            overlay.style.display = 'none';
            
            // Try to focus the iframe content
            try {{
                frame.contentWindow.focus();
            }} catch (e) {{
                console.log('ℹ️ Cannot focus iframe content (normal for cross-origin)');
            }}
        }};
        
        // Handle iframe load errors
        frame.onerror = function() {{
            console.error('❌ WebView frame failed to load');
            overlay.innerHTML = `
                <div style="color: #d73a49;">
                    <h3>❌ Failed to load</h3>
                    <p>Unable to load: {}</p>
                    <p style="font-size: 12px; margin-top: 20px;">This may be due to the site's security policies.</p>
                </div>
            `;
        }};
        
        // Timeout for loading
        setTimeout(() => {{
            if (overlay.style.display !== 'none') {{
                console.log('⏱️ Loading timeout - showing frame anyway');
                overlay.style.display = 'none';
            }}
        }}, 10000);
        
        // Tauri event listeners
        if (window.__TAURI__) {{
            console.log('✅ Tauri available - setting up event listeners');
        }}
    </script>
</body>
</html>
"#, url, url, url, url, serde_json::to_string_pretty(&config).unwrap_or_default(), url)
    }
    
    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn create_webview2_html_with_redirect(&self, target_url: &str, proxy_url: &str, user_agent: &str) -> String {
        format!(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Redirecting to {}</title>
    <style>
        body {{
            font-family: 'Segoe UI', sans-serif;
            margin: 0;
            padding: 20px;
            background: #f5f5f5;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
        }}
        .redirect-container {{
            background: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            text-align: center;
            max-width: 500px;
        }}
        .loading {{
            width: 30px;
            height: 30px;
            border: 3px solid #ddd;
            border-top: 3px solid #4285f4;
            border-radius: 50%;
            animation: spin 1s linear infinite;
            margin: 20px auto;
        }}
        @keyframes spin {{
            0% {{ transform: rotate(0deg); }}
            100% {{ transform: rotate(360deg); }}
        }}
    </style>
</head>
<body>
    <div class="redirect-container">
        <h2>🌐 Ora Browser</h2>
        <div class="loading"></div>
        <p>Redirecting to <strong>{}</strong></p>
        <p><small>If you are not redirected automatically, <a href="{}" target="_self">click here</a>.</small></p>
    </div>
    
    <script>
        console.log('🔄 WebView2 redirect page loaded for: {}');
        
        // Immediate redirect
        setTimeout(() => {{
            console.log('➡️ Performing redirect to: {}');
            window.location.href = '{}';
        }}, 1000);
        
        // Fallback redirect
        setTimeout(() => {{
            if (window.location.href.includes('about:')) {{
                console.log('🔄 Fallback redirect attempt');
                window.open('{}', '_self');
            }}
        }}, 3000);
    </script>
</body>
</html>
"#, target_url, target_url, target_url, target_url, proxy_url, proxy_url, user_agent)
    }
}

impl Default for InternalWebView2Navigator {
    fn default() -> Self {
        Self::new()
    }
}