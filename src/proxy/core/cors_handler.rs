// 🛡️ CORS HANDLER
// Behandelt Cross-Origin Resource Sharing (CORS) und verwandte Sicherheitsaspekte
// Extrahiert aus proxy_server.rs (CORS-spezifische Logik)

use warp::http::{HeaderMap, HeaderName, HeaderValue, Response};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CorsHandler {
    config: CorsConfig,
}

#[derive(Debug, Clone)]
pub struct CorsConfig {
    pub enabled: bool,
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub allow_credentials: bool,
    pub max_age: u64,
    pub expose_headers: Vec<String>,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "OPTIONS".to_string(),
                "HEAD".to_string(),
                "PATCH".to_string(),
            ],
            allowed_headers: vec![
                "Content-Type".to_string(),
                "Authorization".to_string(),
                "X-Requested-With".to_string(),
                "Accept".to_string(),
                "Origin".to_string(),
                "Cache-Control".to_string(),
                "Pragma".to_string(),
            ],
            allow_credentials: true,
            max_age: 3600, // 1 Stunde
            expose_headers: vec![
                "Content-Length".to_string(),
                "Content-Type".to_string(),
            ],
        }
    }
}

impl CorsHandler {
    pub fn new(config: CorsConfig) -> Self {
        Self { config }
    }

    pub fn new_default() -> Self {
        Self::new(CorsConfig::default())
    }

    /// Erstelle CORS-Filter für Warp
    pub fn create_cors_filter(&self) -> warp::cors::Builder {
        warp::cors()
            .allow_any_origin()
            .allow_credentials(true)
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS", "HEAD", "PATCH"])
            .allow_headers(vec![
                "content-type",
                "authorization", 
                "x-requested-with",
                "accept",
                "origin",
                "cache-control",
                "pragma",
                "user-agent",
                "accept-encoding",
                "accept-language",
                "connection",
                "upgrade-insecure-requests",
            ])
    }

    /// Füge CORS-Header zu Response hinzu
    pub fn add_cors_headers_to_response<T>(
        &self,
        mut response: Response<T>,
    ) -> Response<T> {
        if !self.config.enabled {
            return response;
        }

        let headers = response.headers_mut();

        // Access-Control-Allow-Origin
        if self.config.allowed_origins.contains(&"*".to_string()) {
            headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
        } else if let Some(origin) = self.config.allowed_origins.first() {
            if let Ok(value) = HeaderValue::from_str(origin) {
                headers.insert("Access-Control-Allow-Origin", value);
            }
        }

        // Access-Control-Allow-Methods
        let methods_str = self.config.allowed_methods.join(", ");
        if let Ok(value) = HeaderValue::from_str(&methods_str) {
            headers.insert("Access-Control-Allow-Methods", value);
        }

        // Access-Control-Allow-Headers
        let headers_str = self.config.allowed_headers.join(", ");
        if let Ok(value) = HeaderValue::from_str(&headers_str) {
            headers.insert("Access-Control-Allow-Headers", value);
        }

        // Access-Control-Allow-Credentials
        if self.config.allow_credentials {
            headers.insert("Access-Control-Allow-Credentials", HeaderValue::from_static("true"));
        }

        // Access-Control-Max-Age
        let max_age_str = self.config.max_age.to_string();
        if let Ok(value) = HeaderValue::from_str(&max_age_str) {
            headers.insert("Access-Control-Max-Age", value);
        }

        // Access-Control-Expose-Headers
        if !self.config.expose_headers.is_empty() {
            let expose_str = self.config.expose_headers.join(", ");
            if let Ok(value) = HeaderValue::from_str(&expose_str) {
                headers.insert("Access-Control-Expose-Headers", value);
            }
        }

        // Cross-Origin-Resource-Policy für bessere Kompatibilität
        headers.insert("Cross-Origin-Resource-Policy", HeaderValue::from_static("cross-origin"));

        response
    }

    /// Prüfe ob Origin erlaubt ist
    pub fn is_origin_allowed(&self, origin: &str) -> bool {
        if !self.config.enabled {
            return true;
        }

        self.config.allowed_origins.contains(&"*".to_string()) ||
        self.config.allowed_origins.contains(&origin.to_string()) ||
        self.config.allowed_origins.iter().any(|allowed| {
            self.matches_origin_pattern(origin, allowed)
        })
    }

    /// Prüfe ob Method erlaubt ist
    pub fn is_method_allowed(&self, method: &str) -> bool {
        if !self.config.enabled {
            return true;
        }

        self.config.allowed_methods.contains(&method.to_uppercase())
    }

    /// Erstelle Preflight-Response für OPTIONS-Requests
    pub fn create_preflight_response(&self) -> Response<String> {
        let response = Response::builder()
            .status(200)
            .body("".to_string())
            .unwrap();

        self.add_cors_headers_to_response(response)
    }

    /// Verarbeite eingehende Request-Headers für CORS
    pub fn process_request_headers(&self, headers: &HeaderMap) -> CorsRequestInfo {
        let origin = headers.get("origin")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let method = headers.get("access-control-request-method")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let requested_headers = headers.get("access-control-request-headers")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.split(',').map(|h| h.trim().to_string()).collect())
            .unwrap_or_default();

        CorsRequestInfo {
            origin,
            method,
            requested_headers,
        }
    }

    /// Pattern-Matching für Origins (unterstützt Wildcards)
    fn matches_origin_pattern(&self, origin: &str, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }

        if pattern.starts_with("*.") {
            let domain = &pattern[2..];
            return origin.ends_with(domain) || origin == domain;
        }

        origin == pattern
    }

    /// Entferne CSP-Header die CORS blockieren könnten
    pub fn strip_problematic_headers(&self, headers: &mut HeaderMap) {
        // Liste von Headern die Probleme verursachen können
        let problematic_headers = [
            "content-security-policy",
            "content-security-policy-report-only",
            "x-frame-options",
            "x-content-type-options",
            "strict-transport-security",
        ];

        for header_name in &problematic_headers {
            if let Ok(name) = HeaderName::from_bytes(header_name.as_bytes()) {
                headers.remove(&name);
            }
        }
    }

    /// Injiziere permissive CSP-Header
    pub fn inject_permissive_csp(&self, headers: &mut HeaderMap) {
        // Sehr permissive CSP die CORS-Probleme verhindert
        let permissive_csp = "default-src * 'unsafe-inline' 'unsafe-eval' data: blob:; script-src * 'unsafe-inline' 'unsafe-eval'; style-src * 'unsafe-inline';";
        
        if let Ok(value) = HeaderValue::from_str(permissive_csp) {
            headers.insert("Content-Security-Policy", value);
        }
    }

    /// Erstelle Basis-CORS-Headers als HashMap
    pub fn get_cors_headers_map(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();

        if !self.config.enabled {
            return headers;
        }

        // Basis CORS Headers
        headers.insert(
            "Access-Control-Allow-Origin".to_string(),
            if self.config.allowed_origins.contains(&"*".to_string()) {
                "*".to_string()
            } else {
                self.config.allowed_origins.first().unwrap_or(&"*".to_string()).clone()
            }
        );

        headers.insert(
            "Access-Control-Allow-Methods".to_string(),
            self.config.allowed_methods.join(", ")
        );

        headers.insert(
            "Access-Control-Allow-Headers".to_string(),
            self.config.allowed_headers.join(", ")
        );

        if self.config.allow_credentials {
            headers.insert(
                "Access-Control-Allow-Credentials".to_string(),
                "true".to_string()
            );
        }

        headers.insert(
            "Access-Control-Max-Age".to_string(),
            self.config.max_age.to_string()
        );

        headers.insert(
            "Cross-Origin-Resource-Policy".to_string(),
            "cross-origin".to_string()
        );

        headers
    }

    pub fn add_cors_headers(&self, response: Response<String>) -> Response<String> {
        let response = Response::builder()
            .status(response.status())
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS, HEAD, PATCH")
            .header("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With, Accept, Origin, Cache-Control, Pragma")
            .header("Access-Control-Allow-Credentials", "true")
            .header("Cross-Origin-Resource-Policy", "cross-origin")
            .body(response.body().clone())
            .unwrap();

        response
    }
}

#[derive(Debug)]
pub struct CorsRequestInfo {
    pub origin: Option<String>,
    pub method: Option<String>,
    pub requested_headers: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cors_config_default() {
        let config = CorsConfig::default();
        assert!(config.enabled);
        assert!(config.allowed_origins.contains(&"*".to_string()));
        assert!(config.allowed_methods.contains(&"GET".to_string()));
        assert!(config.allow_credentials);
    }

    #[test]
    fn test_origin_matching() {
        let handler = CorsHandler::new_default();
        
        // Test wildcard
        assert!(handler.is_origin_allowed("https://example.com"));
        assert!(handler.is_origin_allowed("http://localhost:3000"));
        
        // Test specific config
        let specific_config = CorsConfig {
            enabled: true,
            allowed_origins: vec!["https://trusted.com".to_string()],
            ..Default::default()
        };
        let specific_handler = CorsHandler::new(specific_config);
        assert!(specific_handler.is_origin_allowed("https://trusted.com"));
        assert!(!specific_handler.is_origin_allowed("https://evil.com"));
    }

    #[test]
    fn test_method_checking() {
        let handler = CorsHandler::new_default();
        
        assert!(handler.is_method_allowed("GET"));
        assert!(handler.is_method_allowed("POST"));
        assert!(handler.is_method_allowed("get")); // Case insensitive
        assert!(!handler.is_method_allowed("TRACE")); // Not in default list
    }

    #[test]
    fn test_cors_headers_map() {
        let handler = CorsHandler::new_default();
        let headers = handler.get_cors_headers_map();
        
        assert!(headers.contains_key("Access-Control-Allow-Origin"));
        assert!(headers.contains_key("Access-Control-Allow-Methods"));
        assert!(headers.contains_key("Access-Control-Allow-Headers"));
        assert_eq!(headers.get("Access-Control-Allow-Origin").unwrap(), "*");
    }

    #[test]
    fn test_origin_pattern_matching() {
        let handler = CorsHandler::new_default();
        
        // Test subdomain pattern
        assert!(handler.matches_origin_pattern("api.example.com", "*.example.com"));
        assert!(handler.matches_origin_pattern("example.com", "*.example.com"));
        assert!(!handler.matches_origin_pattern("evil.com", "*.example.com"));
        
        // Test exact match
        assert!(handler.matches_origin_pattern("https://exact.com", "https://exact.com"));
        assert!(!handler.matches_origin_pattern("https://exact.com", "https://other.com"));
    }
} 
