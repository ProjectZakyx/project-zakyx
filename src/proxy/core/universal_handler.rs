// 🌐 UNIVERSAL RESOURCE HANDLER
// Verarbeitung aller externen Ressourcen-Anfragen
// Extrahiert aus proxy_server.rs (Zeilen ~80-300)

// use crate::proxy::{ProxyResult, ProxyError}; // Entfernt - nicht mehr benötigt
use crate::error::ZAKYXBrowserError;
use reqwest;
use std::collections::HashMap;
use warp::Filter;
use crate::proxy::core::response_processor::ProxyResponse;

#[derive(Debug, Clone)]
pub struct UniversalResourceHandler {
    config: HandlerConfig,
}

#[derive(Debug, Clone)]
pub struct HandlerConfig {
    pub timeout_seconds: u64,
    pub max_redirects: usize,
    pub enable_compression: bool,
    pub custom_headers: HashMap<String, String>,
    pub fallback_user_agents: Vec<String>,
}

impl Default for HandlerConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            max_redirects: 5,
            enable_compression: true,
            custom_headers: HashMap::new(),
            fallback_user_agents: vec![
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15".to_string(),
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            ],
        }
    }
}

impl UniversalResourceHandler {
    pub fn new(config: HandlerConfig) -> Self {
        Self { config }
    }

    pub fn new_with_defaults() -> Self {
        Self::new(HandlerConfig::default())
    }

    /// Hauptschnittstelle für universelle Ressourcen-Anfragen
    pub async fn handle_universal_resource(
        &self,
        url: String,
    ) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
        println!("🔄 Universal resource request for: {}", url);
        
        match self.fetch_resource(&url).await {
            Ok(response) => {
                println!("✅ Resource loaded: {} bytes", response.content.len());
                
                // 🎯 INTELLIGENTE CONTENT-TYPE BESTIMMUNG
                let (content_type, enhanced_content) = if response.content_type.contains("text/html") || 
                                        response.content.trim_start().starts_with("<!DOCTYPE") ||
                                        response.content.trim_start().starts_with("<html") ||
                                        (!url.contains('.') && response.content.contains("<html")) {
                    // HTML-Seite: Injiziere Sicherheitsfixes
                    println!("🌐 Detected HTML content, injecting security fixes");
                    let enhanced = self.inject_cors_and_security_fixes(&response.content, &url);
                    ("text/html; charset=utf-8", enhanced)
                } else {
                    // Verwende ursprünglichen Content-Type oder bestimme basierend auf URL
                    let content_type = if response.content_type != "text/plain" {
                        response.content_type.as_str()
                    } else {
                        self.determine_content_type(&url)
                    };
                    (content_type, response.content)
                };
                
                Ok(Box::new(self.build_response(enhanced_content, content_type, response.status_code)))
            }
            Err(e) => {
                println!("❌ Resource loading failed: {}", e);
                Ok(Box::new(self.build_fallback_response(&url)))
            }
        }
    }

    /// Fetch-Ressource mit erweiterten Strategien
    pub async fn fetch_resource(&self, url: &str) -> Result<ProxyResponse, ZAKYXBrowserError> {
        println!("🔄 Fetching resource with advanced strategies: {}", url);
        
        // 1. Versuch: Standard-Fetch
        match self.try_standard_fetch(url).await {
            Ok(response) => {
                println!("✅ Standard fetch succeeded for: {}", url);
                return Ok(response);
            }
            Err(e) => {
                println!("❌ Standard fetch failed for {}: {}", url, e);
            }
        }

        // 2. Versuch: Mit verschiedenen User-Agents
        for user_agent in &self.config.fallback_user_agents {
            match self.try_fetch_with_user_agent(url, user_agent).await {
                Ok(response) => {
                    println!("✅ User-Agent fetch succeeded for: {}", url);
                    return Ok(response);
                }
                Err(e) => {
                    println!("❌ User-Agent fetch failed for {}: {}", url, e);
                }
            }
        }

        // 3. Versuch: Minimaler Fetch
        match self.try_minimal_resource_fetch(url).await {
            Ok(response) => {
                println!("✅ Minimal fetch succeeded for: {}", url);
                Ok(response)
            }
            Err(e) => {
                println!("❌ All fetch strategies failed for {}: {}", url, e);
                Err(ZAKYXBrowserError::network_error(&format!("All fetch strategies failed: {}", e), Some(url)))
            }
        }
    }

    /// Standard-Fetch-Versuch
    async fn try_standard_fetch(&self, url: &str) -> Result<ProxyResponse, ZAKYXBrowserError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(self.config.timeout_seconds))
            .redirect(reqwest::redirect::Policy::limited(self.config.max_redirects))
            .gzip(self.config.enable_compression)
            .build()
            .map_err(|e| ZAKYXBrowserError::network_error(&format!("Failed to create HTTP client: {}", e), Some(url)))?;

        let mut request = client.get(url);

        for (key, value) in &self.config.custom_headers {
            request = request.header(key, value);
        }

        let response = request.send().await
            .map_err(|e| ZAKYXBrowserError::network_error(&format!("HTTP request failed: {}", e), Some(url)))?;
        self.parse_response(response).await
    }

    /// Fetch mit spezifischem User-Agent
    async fn try_fetch_with_user_agent(&self, url: &str, user_agent: &str) -> Result<ProxyResponse, ZAKYXBrowserError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(self.config.timeout_seconds))
            .redirect(reqwest::redirect::Policy::limited(self.config.max_redirects))
            .user_agent(user_agent)
            .gzip(self.config.enable_compression)
            .build()
            .map_err(|e| ZAKYXBrowserError::network_error(&format!("Failed to create HTTP client with user agent: {}", e), Some(url)))?;

        let response = client.get(url).send().await
            .map_err(|e| ZAKYXBrowserError::network_error(&format!("HTTP request with user agent failed: {}", e), Some(url)))?;
        self.parse_response(response).await
    }

    /// Minimaler Fetch ohne erweiterte Features
    async fn try_minimal_resource_fetch(&self, url: &str) -> Result<ProxyResponse, ZAKYXBrowserError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|e| ZAKYXBrowserError::network_error(&format!("Failed to create minimal HTTP client: {}", e), Some(url)))?;

        let response = client.get(url).send().await
            .map_err(|e| ZAKYXBrowserError::network_error(&format!("Minimal HTTP request failed: {}", e), Some(url)))?;
        self.parse_response(response).await
    }

    /// Parse HTTP-Response zu ProxyResponse
    async fn parse_response(&self, response: reqwest::Response) -> Result<ProxyResponse, ZAKYXBrowserError> {
        let status_code = response.status().as_u16();
        let content_type = response.headers()
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("text/plain")
            .to_string();
        
        let content = response.text().await
            .map_err(|e| ZAKYXBrowserError::network_error(&format!("Failed to read response body: {}", e), None))?;
        
        Ok(ProxyResponse::new(content, content_type, status_code))
    }

    /// Bestimme Content-Type basierend auf URL
    fn determine_content_type(&self, url: &str) -> &'static str {
        if url.contains(".js") || url.contains("javascript") {
            "application/javascript"
        } else if url.contains(".css") {
            "text/css"
        } else if url.contains(".json") {
            "application/json"
        } else if url.contains(".xml") {
            "application/xml"
        } else if url.contains(".html") || url.contains(".htm") {
            "text/html; charset=utf-8"
        } else if url.contains(".png") {
            "image/png"
        } else if url.contains(".jpg") || url.contains(".jpeg") {
            "image/jpeg"
        } else if url.contains(".gif") {
            "image/gif"
        } else if url.contains(".webp") {
            "image/webp"
        } else if url.contains(".svg") {
            "image/svg+xml"
        } else if url.contains(".woff") || url.contains(".woff2") {
            "font/woff2"
        } else if url.contains(".ttf") {
            "font/ttf"
        } else if url.contains(".ico") {
            "image/x-icon"
        } else if !url.contains('.') && (url.contains("yandex") || url.contains("google") || url.contains("bing") || url.contains("search")) {
            "text/html; charset=utf-8"
        } else {
            "text/plain"
        }
    }

    /// Injiziere CORS und Sicherheitsfixes in HTML
    fn inject_cors_and_security_fixes(&self, html: &str, _url: &str) -> String {
        // Vereinfachte Version der ursprünglichen inject_cors_and_security_fixes Funktion
        let mut enhanced_html = html.to_string();
        
        // Basis CORS-Fix: Injiziere Meta-Tags für bessere Kompatibilität
        if enhanced_html.contains("<head>") {
            let meta_injection = r#"
    <meta name="zakyx-proxy-enhanced" content="true">
    <meta http-equiv="Content-Security-Policy" content="default-src * 'unsafe-inline' 'unsafe-eval' data: blob:;">
    <meta name="referrer" content="no-referrer">
"#;
            enhanced_html = enhanced_html.replace("<head>", &format!("<head>{}", meta_injection));
        }
        
        enhanced_html
    }

    /// Erstelle HTTP-Response mit CORS-Headern
    fn build_response(&self, content: String, content_type: &str, status_code: u16) -> warp::http::Response<String> {
        let mut response_builder = warp::http::Response::builder()
            .status(status_code)
            .header("Content-Type", content_type);

        if true { // CORS always enabled
            response_builder = response_builder
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS, HEAD, PATCH")
                .header("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With, Accept, Origin, Cache-Control, Pragma")
                .header("Access-Control-Allow-Credentials", "true")
                .header("Cross-Origin-Resource-Policy", "cross-origin");
        }

        response_builder
            .header("Cache-Control", "public, max-age=3600")
            .body(content)
            .unwrap()
    }

    /// Erstelle Fallback-Response für fehlgeschlagene Anfragen
    fn build_fallback_response(&self, url: &str) -> warp::http::Response<String> {
        // Für JavaScript-Dateien: Rückgabe eines leeren Skripts
        if url.contains(".js") {
            let fallback_content = format!("// Fallback for {}\nconsole.log('Resource {} could not be loaded');", url, url);
            return self.build_response(fallback_content, "application/javascript", 200);
        }
        
        // Für CSS-Dateien: Rückgabe eines leeren Stylesheets
        if url.contains(".css") {
            let fallback_content = format!("/* Fallback for {} */\n/* Resource could not be loaded */", url);
            return self.build_response(fallback_content, "text/css", 200);
        }
        
        // Für andere Ressourcen: 404
        let fallback_content = format!("Resource not found: {}", url);
        self.build_response(fallback_content, "text/plain", 404)
    }

    /// Get handler status (for monitoring/debugging)
    pub fn get_status(&self) -> HandlerStatus {
        HandlerStatus {
            timeout_seconds: self.config.timeout_seconds,
            max_redirects: self.config.max_redirects,
            compression_enabled: self.config.enable_compression,
            user_agents_count: self.config.fallback_user_agents.len(),
            custom_headers_count: self.config.custom_headers.len(),
        }
    }
}

#[derive(Debug)]
pub struct HandlerStatus {
    pub timeout_seconds: u64,
    pub max_redirects: usize,
    pub compression_enabled: bool,
    pub user_agents_count: usize,
    pub custom_headers_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_content_type() {
        let handler = UniversalResourceHandler::new_with_defaults();
        
        assert_eq!(handler.determine_content_type("test.js"), "application/javascript");
        assert_eq!(handler.determine_content_type("style.css"), "text/css");
        assert_eq!(handler.determine_content_type("data.json"), "application/json");
        assert_eq!(handler.determine_content_type("page.html"), "text/html; charset=utf-8");
        assert_eq!(handler.determine_content_type("image.png"), "image/png");
        assert_eq!(handler.determine_content_type("search/google"), "text/html; charset=utf-8");
    }

    #[test]
    fn test_config_creation() {
        let config = HandlerConfig::default();
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.max_redirects, 5);
        assert!(config.enable_compression);
        assert_eq!(config.fallback_user_agents.len(), 3);
    }

    #[tokio::test]
    async fn test_handler_creation() {
        let handler = UniversalResourceHandler::new_with_defaults();
        assert_eq!(handler.config.timeout_seconds, 30);
    }

    #[test]
    fn test_handler_status() {
        let handler = UniversalResourceHandler::new_with_defaults();
        let status = handler.get_status();
        
        assert_eq!(status.timeout_seconds, 30);
        assert_eq!(status.max_redirects, 5);
        assert!(status.compression_enabled);
        assert!(status.user_agents_count > 0);
    }
} 
