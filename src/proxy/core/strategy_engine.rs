// 🎯 STRATEGY ENGINE
// Verwaltet verschiedene Verbindungsstrategien für Proxy-Requests
// Extrahiert aus proxy_server.rs (Connection Strategy Logik)

use crate::proxy::core::response_processor::ProxyResponse;
use crate::error::OraBrowserError;
use reqwest;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ConnectionStrategy {
    pub name: String,
    pub description: String,
    pub user_agent: String,
    pub timeout_seconds: u64,
    pub connect_timeout_seconds: u64,
    pub max_redirects: usize,
    pub accept_invalid_certs: bool,
    pub use_http2: bool,
    pub headers: Vec<(String, String)>,
}

#[derive(Clone, Debug)]
pub struct StrategyEngine {
    strategies: Vec<ConnectionStrategy>,
    current_strategy_index: usize,
}

impl StrategyEngine {
    pub fn new() -> Self {
        Self {
            strategies: Self::create_default_strategies(),
            current_strategy_index: 0,
        }
    }

    pub fn with_strategies(strategies: Vec<ConnectionStrategy>) -> Self {
        Self {
            strategies,
            current_strategy_index: 0,
        }
    }

    /// Erstelle Standard-Verbindungsstrategien
    fn create_default_strategies() -> Vec<ConnectionStrategy> {
        vec![
            // Standard-Browser-Strategy
            ConnectionStrategy {
                name: "standard_browser".to_string(),
                description: "Standard browser with modern User-Agent".to_string(),
                user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                timeout_seconds: 30,
                connect_timeout_seconds: 10,
                max_redirects: 5,
                accept_invalid_certs: false,
                use_http2: true,
                headers: vec![
                    ("Accept".to_string(), "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8".to_string()),
                    ("Accept-Language".to_string(), "de-DE,de;q=0.9,en;q=0.8".to_string()),
                    ("Accept-Encoding".to_string(), "gzip, deflate".to_string()),
                    ("DNT".to_string(), "1".to_string()),
                ],
            },

            // Mobile-Browser-Strategy
            ConnectionStrategy {
                name: "mobile_browser".to_string(),
                description: "Mobile browser simulation".to_string(),
                user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1".to_string(),
                timeout_seconds: 25,
                connect_timeout_seconds: 8,
                max_redirects: 3,
                accept_invalid_certs: false,
                use_http2: true,
                headers: vec![
                    ("Accept".to_string(), "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".to_string()),
                    ("Accept-Language".to_string(), "de-DE,de;q=0.9".to_string()),
                ],
            },

            // Lightweight-Strategy für problematische Sites
            ConnectionStrategy {
                name: "lightweight".to_string(),
                description: "Lightweight strategy for problematic sites".to_string(),
                user_agent: "curl/7.68.0".to_string(),
                timeout_seconds: 20,
                connect_timeout_seconds: 5,
                max_redirects: 3,
                accept_invalid_certs: true,
                use_http2: false,
                headers: vec![
                    ("Accept".to_string(), "*/*".to_string()),
                ],
            },

            // Stealth-Strategy
            ConnectionStrategy {
                name: "stealth".to_string(),
                description: "Stealth mode with minimal headers".to_string(),
                user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36".to_string(),
                timeout_seconds: 15,
                connect_timeout_seconds: 5,
                max_redirects: 2,
                accept_invalid_certs: true,
                use_http2: false,
                headers: vec![],
            },

            // Google-optimierte Strategy
            ConnectionStrategy {
                name: "google_optimized".to_string(),
                description: "Optimized for Google services".to_string(),
                user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                timeout_seconds: 25,
                connect_timeout_seconds: 8,
                max_redirects: 5,
                accept_invalid_certs: false,
                use_http2: true,
                headers: vec![
                    ("Accept".to_string(), "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8".to_string()),
                    ("Accept-Language".to_string(), "de-DE,de;q=0.9,en;q=0.8".to_string()),
                    ("Cache-Control".to_string(), "no-cache".to_string()),
                    ("Upgrade-Insecure-Requests".to_string(), "1".to_string()),
                ],
            },
        ]
    }

    /// Versuche Verbindung mit aktueller Strategy
    pub async fn try_request_with_current_strategy(
        &self,
        url: &str,
        method: &str,
        body: Option<&str>,
    ) -> Result<reqwest::Response, OraBrowserError> {
        if self.current_strategy_index >= self.strategies.len() {
            return Err(OraBrowserError::proxy_error("No more strategies available", Some(url)));
        }

        let strategy = &self.strategies[self.current_strategy_index];
        self.execute_strategy(url, method, body, strategy).await
    }

    /// Versuche alle Strategien nacheinander
    pub async fn try_all_strategies(
        &mut self,
        url: &str,
        method: &str,
        body: Option<&str>,
    ) -> Result<reqwest::Response, OraBrowserError> {
        let total_strategies = self.strategies.len();
        
        for i in 0..total_strategies {
            self.current_strategy_index = i;
            let strategy = &self.strategies[i];
            
            println!("🔄 Trying strategy '{}': {}", strategy.name, strategy.description);
            
            match self.execute_strategy(url, method, body, strategy).await {
                Ok(response) => {
                    println!("✅ Strategy '{}' succeeded for: {}", strategy.name, url);
                    return Ok(response);
                }
                Err(e) => {
                    println!("❌ Strategy '{}' failed for {}: {}", strategy.name, url, e);
                    
                    // Kurze Pause zwischen Strategien
                    if i < total_strategies - 1 {
                        tokio::time::sleep(Duration::from_millis(300)).await;
                    }
                }
            }
        }

        // Wenn alle Strategien fehlschlagen, gib einen Fehler zurück
        Err(OraBrowserError::proxy_error("All connection strategies failed", Some(url)))
    }

    /// Führe eine spezifische Strategy aus
    async fn execute_strategy(
        &self,
        url: &str,
        method: &str,
        body: Option<&str>,
        strategy: &ConnectionStrategy,
    ) -> Result<reqwest::Response, OraBrowserError> {
        let mut client_builder = reqwest::Client::builder()
            .timeout(Duration::from_secs(strategy.timeout_seconds))
            .connect_timeout(Duration::from_secs(strategy.connect_timeout_seconds))
            .redirect(reqwest::redirect::Policy::limited(strategy.max_redirects))
            .user_agent(&strategy.user_agent)
            .danger_accept_invalid_certs(strategy.accept_invalid_certs);

        // HTTP/2 Konfiguration
        if strategy.use_http2 {
            client_builder = client_builder.http2_prior_knowledge();
        }

        let client = client_builder.build()
            .map_err(|e| OraBrowserError::network_error(&format!("Failed to build HTTP client: {}", e), Some(url)))?;
        
        let mut request = match method.to_uppercase().as_str() {
            "GET" => client.get(url),
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            "HEAD" => client.head(url),
            "PATCH" => client.patch(url),
            _ => client.get(url),
        };

        // Setze Standard-Header
        for (key, value) in &strategy.headers {
            request = request.header(key, value);
        }

        // Setze Body für POST/PUT/PATCH
        if let Some(body_content) = body {
            if matches!(method.to_uppercase().as_str(), "POST" | "PUT" | "PATCH") {
                request = request.body(body_content.to_string());
            }
        }

        let response = request.send().await
            .map_err(|e| OraBrowserError::network_error(&format!("HTTP request failed: {}", e), Some(url)))?;
        Ok(response)
    }

    /// Nächste Strategy wählen
    pub fn next_strategy(&mut self) -> bool {
        if self.current_strategy_index < self.strategies.len() - 1 {
            self.current_strategy_index += 1;
            true
        } else {
            false
        }
    }

    /// Strategien zurücksetzen
    pub fn reset(&mut self) {
        self.current_strategy_index = 0;
    }

    /// Aktuelle Strategy abrufen
    pub fn current_strategy(&self) -> Option<&ConnectionStrategy> {
        self.strategies.get(self.current_strategy_index)
    }

    /// Strategy nach Name suchen
    pub fn get_strategy_by_name(&self, name: &str) -> Option<&ConnectionStrategy> {
        self.strategies.iter().find(|s| s.name == name)
    }

    /// Neue Strategy hinzufügen
    pub fn add_strategy(&mut self, strategy: ConnectionStrategy) {
        self.strategies.push(strategy);
    }

    /// Alle Strategy-Namen abrufen
    pub fn get_strategy_names(&self) -> Vec<&str> {
        self.strategies.iter().map(|s| s.name.as_str()).collect()
    }

    /// Optimale Strategy für URL auswählen
    pub fn select_optimal_strategy_for_url(&mut self, url: &str) -> Option<&ConnectionStrategy> {
        if url.contains("google.com") {
            self.current_strategy_index = self.strategies.iter()
                .position(|s| s.name == "google_optimized")
                .unwrap_or(0);
        } else if url.contains("mobile") || url.contains("m.") {
            self.current_strategy_index = self.strategies.iter()
                .position(|s| s.name == "mobile_browser")
                .unwrap_or(0);
        } else {
            self.current_strategy_index = 0; // Standard-Strategy
        }

        self.current_strategy()
    }
}

impl Default for StrategyEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// SmartProxy - High-Level Interface für Proxy-Requests
pub struct SmartProxy {
    strategy_engine: StrategyEngine,
}

impl SmartProxy {
    pub fn new() -> Self {
        Self {
            strategy_engine: StrategyEngine::new(),
        }
    }

    pub async fn fetch_and_strip_headers(url: &str) -> Result<ProxyResponse, OraBrowserError> {
        let mut engine = StrategyEngine::new();
        
        match engine.try_all_strategies(url, "GET", None).await {
            Ok(response) => {
                Self::process_response(response, url).await
            }
            Err(e) => {
                println!("❌ All strategies failed for {}: {}", url, e);
                Err(e)
            }
        }
    }

    async fn process_response(response: reqwest::Response, _url: &str) -> Result<ProxyResponse, OraBrowserError> {
        let status_code = response.status().as_u16();
        let content_type = response.headers()
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("text/html")
            .to_string();
        
        let content = response.text().await
            .map_err(|e| OraBrowserError::network_error(&format!("Failed to read response body: {}", e), None))?;
        
        Ok(ProxyResponse::new(content, content_type, status_code))
    }

    pub fn extract_domain(url: &str) -> String {
        if let Ok(parsed) = url::Url::parse(url) {
            parsed.host_str().unwrap_or("unknown").to_string()
        } else {
            "unknown".to_string()
        }
    }
}

impl Default for SmartProxy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_creation() {
        let engine = StrategyEngine::new();
        assert!(engine.strategies.len() > 0);
    }

    #[test]
    fn test_strategy_navigation() {
        let mut engine = StrategyEngine::new();
        let initial_index = engine.current_strategy_index;
        
        let has_next = engine.next_strategy();
        assert!(has_next);
        assert!(engine.current_strategy_index > initial_index);
        
        engine.reset();
        assert_eq!(engine.current_strategy_index, 0);
    }

    #[test]
    fn test_strategy_selection() {
        let mut engine = StrategyEngine::new();
        
        let strategy = engine.select_optimal_strategy_for_url("https://google.com/search");
        assert!(strategy.is_some());
        
        let strategy = engine.select_optimal_strategy_for_url("https://m.facebook.com");
        assert!(strategy.is_some());
    }

    #[test]
    fn test_strategy_by_name() {
        let engine = StrategyEngine::new();
        
        let strategy = engine.get_strategy_by_name("standard_browser");
        assert!(strategy.is_some());
        assert_eq!(strategy.unwrap().name, "standard_browser");
        
        let strategy = engine.get_strategy_by_name("nonexistent");
        assert!(strategy.is_none());
    }

    #[test]
    fn test_smart_proxy_creation() {
        let proxy = SmartProxy::new();
        assert!(proxy.strategy_engine.strategies.len() > 0);
    }

    #[test]
    fn test_domain_extraction() {
        let domain = SmartProxy::extract_domain("https://example.com/path");
        assert_eq!(domain, "example.com");
        
        let domain = SmartProxy::extract_domain("invalid-url");
        assert_eq!(domain, "unknown");
    }
} 