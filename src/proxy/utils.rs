// 🛠️ UTILS MODULE - Placeholder für die Kompilierung
// Wird in den späteren Milestones implementiert

pub struct UrlUtils {
    // Placeholder
}

impl UrlUtils {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for UrlUtils {
    fn default() -> Self {
        Self::new()
    }
}

// 🛠️ Proxy Utilities
// Hilfsfunktionen für das Proxy-System

use std::collections::HashMap;

/// Proxy URL-Utilities  
pub struct ProxyUrlUtils;

impl ProxyUrlUtils {
    /// Extrahiere Domain aus URL
    pub fn extract_domain(url: &str) -> String {
        if let Ok(parsed) = url::Url::parse(url) {
            parsed.host_str().unwrap_or("unknown").to_lowercase()
        } else {
            url.to_lowercase()
        }
    }

    /// Extrahiere Base-URL
    pub fn extract_base_url(url: &str) -> Option<String> {
        if let Ok(parsed) = url::Url::parse(url) {
            if let Some(host) = parsed.host_str() {
                let scheme = parsed.scheme();
                return Some(format!("{}://{}", scheme, host));
            }
        }
        None
    }

    /// Prüfe ob URL gültig ist
    pub fn is_valid_url(url: &str) -> bool {
        url::Url::parse(url).is_ok()
    }

    /// Normalisiere URL
    pub fn normalize_url(url: &str) -> String {
        if let Ok(parsed) = url::Url::parse(url) {
            parsed.to_string()
        } else {
            url.to_string()
        }
    }

    /// Erstelle Proxy-URL
    pub fn create_proxy_url(original_url: &str, proxy_base: &str) -> String {
        format!("{}?url={}", proxy_base, urlencoding::encode(original_url))
    }

    /// Extrahiere Original-URL aus Proxy-URL
    pub fn extract_original_url(proxy_url: &str) -> Option<String> {
        if let Ok(parsed) = url::Url::parse(proxy_url) {
            if let Some(query) = parsed.query() {
                for pair in query.split('&') {
                    if let Some((key, value)) = pair.split_once('=') {
                        if key == "url" {
                            return Some(urlencoding::decode(value).ok()?.to_string());
                        }
                    }
                }
            }
        }
        None
    }
}

/// Header-Utilities
pub struct HeaderUtils;

impl HeaderUtils {
    /// Bereinige problematische Headers
    pub fn sanitize_headers(headers: &mut HashMap<String, String>) {
        let problematic_headers = [
            "x-frame-options",
            "content-security-policy",
            "x-content-type-options",
            "strict-transport-security",
            "x-xss-protection",
        ];

        for header in &problematic_headers {
            headers.remove(&header.to_lowercase());
        }
    }

    /// Füge CORS-Headers hinzu
    pub fn add_cors_headers(headers: &mut HashMap<String, String>) {
        headers.insert("access-control-allow-origin".to_string(), "*".to_string());
        headers.insert("access-control-allow-methods".to_string(), "GET, POST, PUT, DELETE, OPTIONS, HEAD, PATCH".to_string());
        headers.insert("access-control-allow-headers".to_string(), "Content-Type, Authorization, X-Requested-With, Accept, Origin, Cache-Control, Pragma".to_string());
        headers.insert("cross-origin-resource-policy".to_string(), "cross-origin".to_string());
    }

    /// Erstelle Standard-Browser-Headers
    pub fn create_browser_headers() -> HashMap<String, String> {
        let mut headers = HashMap::new();
        
        headers.insert("Accept".to_string(), "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8".to_string());
        headers.insert("Accept-Language".to_string(), "en-US,en;q=0.9,de;q=0.8".to_string());
        headers.insert("Accept-Encoding".to_string(), "gzip, deflate, br".to_string());
        headers.insert("Connection".to_string(), "keep-alive".to_string());
        headers.insert("Upgrade-Insecure-Requests".to_string(), "1".to_string());
        headers.insert("Sec-Fetch-Dest".to_string(), "document".to_string());
        headers.insert("Sec-Fetch-Mode".to_string(), "navigate".to_string());
        headers.insert("Sec-Fetch-Site".to_string(), "none".to_string());
        headers.insert("Sec-Fetch-User".to_string(), "?1".to_string());
        
        headers
    }
}

/// Content-Utilities
pub struct ContentUtils;

impl ContentUtils {
    /// Erkenne Content-Typ
    pub fn detect_content_type(content: &str, url: &str) -> String {
        if url.ends_with(".js") {
            "application/javascript".to_string()
        } else if url.ends_with(".css") {
            "text/css".to_string()
        } else if url.ends_with(".html") || url.ends_with(".htm") {
            "text/html; charset=utf-8".to_string()
        } else if url.ends_with(".json") {
            "application/json".to_string()
        } else if url.ends_with(".xml") {
            "application/xml".to_string()
        } else if content.trim_start().starts_with("<!DOCTYPE") || content.trim_start().starts_with("<html") {
            "text/html; charset=utf-8".to_string()
        } else if content.trim_start().starts_with("{") || content.trim_start().starts_with("[") {
            "application/json".to_string()
        } else if content.trim_start().starts_with("<?xml") {
            "application/xml".to_string()
        } else {
            "text/plain".to_string()
        }
    }

    /// Komprimiere Content
    pub fn compress_content(content: &str) -> String {
        // Entferne überflüssige Leerzeichen und Zeilenumbrüche
        regex::Regex::new(r"\s+")
            .unwrap()
            .replace_all(content.trim(), " ")
            .to_string()
    }

    /// Prüfe Content-Größe
    pub fn check_content_size(content: &str, max_size: usize) -> Result<(), String> {
        let size = content.as_bytes().len();
        if size > max_size {
            Err(format!("Content size {} exceeds maximum {}", size, max_size))
        } else {
            Ok(())
        }
    }

    /// Escape HTML-Entities
    pub fn escape_html(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    }

    /// Unescape HTML-Entities
    pub fn unescape_html(text: &str) -> String {
        text.replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#x27;", "'")
            .replace("&#39;", "'")
    }
}

/// Error-Utilities
pub struct ErrorUtils;

impl ErrorUtils {
    /// Erstelle benutzerfreundliche Fehlermeldung
    pub fn create_user_friendly_error(error: &str, url: &str) -> String {
        match error {
            e if e.contains("timeout") => {
                format!("⏰ Zeitüberschreitung beim Laden von {}", url)
            }
            e if e.contains("connection") => {
                format!("🔌 Verbindungsfehler zu {}", url)
            }
            e if e.contains("dns") => {
                format!("🌐 DNS-Fehler für {}", url)
            }
            e if e.contains("certificate") || e.contains("ssl") || e.contains("tls") => {
                format!("🔒 SSL-Zertifikatsfehler für {}", url)
            }
            e if e.contains("404") => {
                format!("❌ Seite nicht gefunden: {}", url)
            }
            e if e.contains("403") => {
                format!("🚫 Zugriff verweigert: {}", url)
            }
            e if e.contains("500") => {
                format!("💥 Server-Fehler bei {}", url)
            }
            _ => {
                format!("❌ Fehler beim Laden von {}: {}", url, error)
            }
        }
    }

    /// Klassifiziere Fehler-Typ
    pub fn classify_error(error: &str) -> ErrorType {
        if error.contains("timeout") {
            ErrorType::Timeout
        } else if error.contains("connection") {
            ErrorType::Connection
        } else if error.contains("dns") {
            ErrorType::Dns
        } else if error.contains("certificate") || error.contains("ssl") {
            ErrorType::Certificate
        } else if error.contains("404") {
            ErrorType::NotFound
        } else if error.contains("403") {
            ErrorType::Forbidden
        } else if error.contains("500") {
            ErrorType::ServerError
        } else {
            ErrorType::Unknown
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    Timeout,
    Connection,
    Dns,
    Certificate,
    NotFound,
    Forbidden,
    ServerError,
    Unknown,
}

/// Performance-Utilities
pub struct PerformanceUtils;

impl PerformanceUtils {
    /// Messe Ausführungszeit
    pub fn measure_time<F, R>(f: F) -> (R, std::time::Duration)
    where
        F: FnOnce() -> R,
    {
        let start = std::time::Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }

    /// Formatiere Dauer
    pub fn format_duration(duration: std::time::Duration) -> String {
        let ms = duration.as_millis();
        if ms < 1000 {
            format!("{}ms", ms)
        } else {
            format!("{:.2}s", duration.as_secs_f64())
        }
    }
}

/// Debug-Utilities
pub struct DebugUtils;

impl DebugUtils {
    /// Logge Request-Details
    pub fn log_request(url: &str, method: &str, headers: &HashMap<String, String>) {
        println!("🌐 {} {}", method, url);
        for (key, value) in headers {
            if key.to_lowercase().contains("authorization") {
                println!("  {}: [REDACTED]", key);
            } else {
                println!("  {}: {}", key, value);
            }
        }
    }

    /// Logge Response-Details
    pub fn log_response(url: &str, status: u16, size: usize, duration: std::time::Duration) {
        println!(
            "📦 {} - {} - {} bytes - {}",
            url,
            status,
            size,
            PerformanceUtils::format_duration(duration)
        );
    }

    /// Erstelle Debug-Info
    pub fn create_debug_info(url: &str, content: &str) -> String {
        format!(
            "URL: {}\nContent-Length: {} bytes\nContent-Type: {}\nFirst 200 chars: {}",
            url,
            content.len(),
            ContentUtils::detect_content_type(content, url),
            content.chars().take(200).collect::<String>()
        )
    }
} 