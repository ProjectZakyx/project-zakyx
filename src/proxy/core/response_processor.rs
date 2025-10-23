// 📝 RESPONSE PROCESSOR
// Verarbeitet und transformiert HTTP-Responses
// Extrahiert aus proxy_server.rs (Response-Verarbeitungslogik)

// use crate::proxy::{ProxyResult, ProxyError}; // Entfernt - nicht mehr benötigt
use crate::error::ZAKYXBrowserError;
use reqwest;
use std::collections::HashMap;
use url;

#[derive(Debug, Clone)]
pub struct ProcessedResponse {
    pub content: String,
    pub content_type: String,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub size_bytes: usize,
}

#[derive(Clone, Debug)]
pub struct ResponseProcessor {
    config: ProcessorConfig,
}

#[derive(Debug, Clone)]
pub struct ProcessorConfig {
    pub max_content_size: usize,
    pub enable_content_enhancement: bool,
    pub enable_security_fixes: bool,
    pub enable_cors_fixes: bool,
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            max_content_size: 50 * 1024 * 1024,
            enable_content_enhancement: true,
            enable_security_fixes: true,
            enable_cors_fixes: true,
        }
    }
}

impl ResponseProcessor {
    pub fn new(config: ProcessorConfig) -> Self {
        Self { config }
    }

    pub fn new_default() -> Self {
        Self::new(ProcessorConfig::default())
    }

    pub async fn process_response(
        &self,
        response: reqwest::Response,
        original_url: &str,
    ) -> Result<ProcessedResponse, ZAKYXBrowserError> {
        let status_code = response.status().as_u16();
        let headers = self.extract_headers(&response);
        let content_type = self.determine_content_type(&response, original_url);
        let raw_content = response.text().await
            .map_err(|e| ZAKYXBrowserError::network_error(&format!("Failed to read response body: {}", e), Some(original_url)))?;

        if raw_content.len() > self.config.max_content_size {
            return Err(ZAKYXBrowserError::proxy_error(&format!(
                "Content size {} exceeds limit {}",
                raw_content.len(),
                self.config.max_content_size
            ), Some(original_url)));
        }

        let processed_content = if self.is_html_content(&content_type, &raw_content) {
            self.enhance_html_content(&raw_content, original_url)
        } else {
            raw_content
        };

        Ok(ProcessedResponse {
            content: processed_content.clone(),
            content_type: content_type.clone(),
            status_code,
            headers: self.process_headers(headers, &content_type),
            size_bytes: processed_content.len(),
        })
    }

    fn extract_headers(&self, response: &reqwest::Response) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        for (name, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(name.to_string(), value_str.to_string());
            }
        }
        headers
    }

    fn determine_content_type(&self, response: &reqwest::Response, url: &str) -> String {
        if let Some(ct) = response.headers().get("content-type") {
            if let Ok(ct_str) = ct.to_str() {
                return ct_str.to_string();
            }
        }

        if url.ends_with(".js") {
            "application/javascript".to_string()
        } else if url.ends_with(".css") {
            "text/css".to_string()
        } else if url.ends_with(".html") {
            "text/html; charset=utf-8".to_string()
        } else {
            "text/plain".to_string()
        }
    }

    fn is_html_content(&self, content_type: &str, content: &str) -> bool {
        content_type.contains("text/html") ||
        content.trim_start().starts_with("<!DOCTYPE") ||
        content.trim_start().starts_with("<html")
    }

    fn enhance_html_content(&self, html: &str, _url: &str) -> String {
        let mut enhanced = html.to_string();

        if enhanced.contains("<head>") {
            let meta_injection = r#"
    <meta name="zakyx-proxy-enhanced" content="true">
    <meta http-equiv="Content-Security-Policy" content="default-src * 'unsafe-inline' 'unsafe-eval' data: blob:;">
    <meta name="referrer" content="no-referrer">
"#;
            enhanced = enhanced.replace("<head>", &format!("<head>{}", meta_injection));
        }

        enhanced
    }

    fn process_headers(&self, mut headers: HashMap<String, String>, content_type: &str) -> HashMap<String, String> {
        if self.config.enable_cors_fixes {
            headers.insert("access-control-allow-origin".to_string(), "*".to_string());
            headers.insert("access-control-allow-methods".to_string(), "GET, POST, PUT, DELETE, OPTIONS, HEAD, PATCH".to_string());
            headers.insert("access-control-allow-headers".to_string(), "Content-Type, Authorization, X-Requested-With, Accept, Origin, Cache-Control, Pragma".to_string());
            headers.insert("cross-origin-resource-policy".to_string(), "cross-origin".to_string());
        }

        headers.insert("content-type".to_string(), content_type.to_string());
        headers
    }

    fn is_javascript_content(&self, content: &str) -> bool {
        content.trim_start().starts_with("function") ||
        content.trim_start().starts_with("var ") ||
        content.trim_start().starts_with("let ") ||
        content.trim_start().starts_with("const ") ||
        content.trim_start().starts_with("class ") ||
        content.contains("console.log") ||
        content.contains("document.") ||
        content.contains("window.")
    }

    fn is_css_content(&self, content: &str) -> bool {
        content.trim_start().starts_with("@") ||
        content.contains("{") && content.contains("}") &&
        (content.contains("color:") || content.contains("background:") || content.contains("margin:") || content.contains("padding:"))
    }

    fn apply_security_fixes(&self, html: &str) -> String {
        let mut fixed = html.to_string();
        
        // Entferne potenziell gefährliche Skripte
        fixed = fixed.replace("javascript:", "");
        fixed = fixed.replace("data:text/html", "");
        fixed = fixed.replace("vbscript:", "");
        
        // Füge Sicherheitsheader hinzu
        if !fixed.contains("<meta") && fixed.contains("<head>") {
            fixed = fixed.replace("<head>", r#"<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<meta http-equiv="X-Content-Type-Options" content="nosniff">
<meta http-equiv="X-Frame-Options" content="SAMEORIGIN">
<meta http-equiv="Content-Security-Policy" content="default-src 'self' 'unsafe-inline' 'unsafe-eval' *; script-src 'self' 'unsafe-inline' 'unsafe-eval' *; style-src 'self' 'unsafe-inline' *;">"#);
        }
        
        fixed
    }

    fn extract_base_url(&self, url: &str) -> Option<String> {
        if let Ok(parsed) = url::Url::parse(url) {
            if let Some(host) = parsed.host_str() {
                let scheme = parsed.scheme();
                return Some(format!("{}://{}", scheme, host));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_config() {
        let config = ProcessorConfig::default();
        assert!(config.enable_content_enhancement);
        assert!(config.enable_security_fixes);
        assert!(config.enable_cors_fixes);
        assert_eq!(config.max_content_size, 50 * 1024 * 1024);
    }

    #[test]
    fn test_content_type_detection() {
        let processor = ResponseProcessor::new_default();
        
        assert!(processor.is_html_content("text/html", "<!DOCTYPE html>"));
        assert!(processor.is_html_content("text/html; charset=utf-8", "<html>"));
        assert!(processor.is_javascript_content("function test() {}"));
        assert!(processor.is_css_content("body { color: red; }"));
    }

    #[test]
    fn test_html_enhancement() {
        let processor = ResponseProcessor::new_default();
        let html = "<html><head></head><body></body></html>";
        let enhanced = processor.enhance_html_content(html, "https://example.com");
        
        assert!(enhanced.contains("zakyx-proxy-enhanced"));
        assert!(enhanced.contains("Content-Security-Policy"));
    }

    #[test]
    fn test_security_fixes() {
        let processor = ResponseProcessor::new_default();
        let html = "<html><head></head><body><script>javascript:alert('xss')</script></body></html>";
        let secured = processor.apply_security_fixes(html);
        
        assert!(!secured.contains("javascript:alert"));
    }

    #[test]
    fn test_base_url_extraction() {
        let processor = ResponseProcessor::new_default();
        
        assert_eq!(
            processor.extract_base_url("https://example.com/path/to/page"),
            Some("https://example.com".to_string())
        );
        
        assert_eq!(
            processor.extract_base_url("http://localhost:3000/test"),
            Some("http://localhost:3000".to_string())
        );
    }
}

// 📦 Smart Proxy Response Processor
// Verarbeitet HTTP-Responses und bereitet sie für die Rückgabe auf

#[derive(Debug, Clone)]
pub struct ProxyResponse {
    pub content: String,
    pub content_type: String,
    pub status_code: u16,
}

impl ProxyResponse {
    /// Erstelle eine neue ProxyResponse
    pub fn new(content: String, content_type: String, status_code: u16) -> Self {
        Self {
            content,
            content_type,
            status_code,
        }
    }

    /// Überprüfe ob die Response erfolgreich war
    pub fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }

    /// Überprüfe ob die Response HTML-Content enthält
    pub fn is_html(&self) -> bool {
        self.content_type.contains("text/html")
    }

    /// Überprüfe ob die Response JSON-Content enthält
    pub fn is_json(&self) -> bool {
        self.content_type.contains("application/json")
    }

    /// Überprüfe ob die Response Text-Content enthält
    pub fn is_text(&self) -> bool {
        self.content_type.starts_with("text/")
    }

    /// Hole die Größe des Contents in Bytes
    pub fn content_size(&self) -> usize {
        self.content.as_bytes().len()
    }

    /// Erstelle eine Error-Response
    pub fn error(status_code: u16, message: &str) -> Self {
        Self {
            content: format!(
                r#"<!DOCTYPE html>
                <html>
                <head>
                    <title>Proxy Error {}</title>
                    <style>
                        body {{ font-family: Arial, sans-serif; padding: 40px; text-align: center; }}
                        .error {{ background: #ffebee; border: 1px solid #f44336; padding: 20px; border-radius: 5px; }}
                        .error-code {{ font-size: 24px; color: #f44336; margin-bottom: 10px; }}
                        .error-message {{ color: #666; }}
                    </style>
                </head>
                <body>
                    <div class="error">
                        <div class="error-code">Error {}</div>
                        <div class="error-message">{}</div>
                    </div>
                </body>
                </html>"#,
                status_code, status_code, message
            ),
            content_type: "text/html".to_string(),
            status_code,
        }
    }
}

/// Legacy response processor functions
impl ResponseProcessor {
    /// Verarbeite eine HTTP-Response zu einer ProxyResponse (Legacy-Kompatibilität)
    pub async fn process_response_legacy(
        response: reqwest::Response,
        url: &str,
    ) -> Result<ProxyResponse, ZAKYXBrowserError> {
        let status_code = response.status().as_u16();
        let content_type = response.headers()
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("text/html")
            .to_string();
        
        let content = response.text().await
            .map_err(|e| ZAKYXBrowserError::network_error(&format!("Failed to read response body: {}", e), Some(url)))?;
        
        let mut proxy_response = ProxyResponse::new(content, content_type, status_code);
        
        // Führe zusätzliche Verarbeitung durch
        if proxy_response.is_html() {
            proxy_response = Self::process_html_response(proxy_response, url)?;
        }
        
        Ok(proxy_response)
    }

    /// Verarbeite HTML-Responses speziell
    fn process_html_response(
        mut response: ProxyResponse,
        url: &str,
    ) -> Result<ProxyResponse, ZAKYXBrowserError> {
        // Basis-URL für relative Links hinzufügen
        if !response.content.contains("<base href=") {
            if let Ok(parsed_url) = url::Url::parse(url) {
                let base_url = format!("{}://{}", parsed_url.scheme(), parsed_url.host_str().unwrap_or(""));
                let base_tag = format!("<base href=\"{}\">", base_url);
                
                if let Some(head_pos) = response.content.find("<head>") {
                    response.content.insert_str(head_pos + 6, &base_tag);
                }
            }
        }
        
        Ok(response)
    }

    /// Validiere Response-Größe
    pub fn validate_size(&self, response: &ProxyResponse, max_size: usize) -> Result<(), ZAKYXBrowserError> {
        if response.content_size() > max_size {
            return Err(ZAKYXBrowserError::proxy_error(&format!(
                "Response size {} exceeds maximum allowed size {}",
                response.content_size(),
                max_size
            ), None));
        }
        Ok(())
    }

    /// Erweiterte Response-Verarbeitung mit allen Features
    pub async fn process_extended_response(
        &self,
        response: reqwest::Response,
        original_url: &str,
    ) -> Result<ProcessedResponse, ZAKYXBrowserError> {
        let status_code = response.status().as_u16();
        let headers = self.extract_headers(&response);
        let content_type = self.determine_content_type(&response, original_url);
        let raw_content = response.text().await
            .map_err(|e| ZAKYXBrowserError::network_error(&format!("Failed to read response body: {}", e), Some(original_url)))?;

        if raw_content.len() > self.config.max_content_size {
            return Err(ZAKYXBrowserError::proxy_error(&format!(
                "Content size {} exceeds limit {}",
                raw_content.len(),
                self.config.max_content_size
            ), Some(original_url)));
        }

        let processed_content = if self.is_html_content(&content_type, &raw_content) {
            self.enhance_html_content(&raw_content, original_url)
        } else {
            raw_content
        };

        Ok(ProcessedResponse {
            content: processed_content.clone(),
            content_type: content_type.clone(),
            status_code,
            headers: self.process_headers(headers, &content_type),
            size_bytes: processed_content.len(),
        })
    }
} 
