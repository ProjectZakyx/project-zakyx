/// 🛡️ ETHICAL SAFEGUARDS FÜR ZAKYX BROWSER
/// Implementiert Schutzmaßnahmen für verantwortungsvolles Web-Browsing

use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::time::sleep;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EthicalConfig {
    pub max_requests_per_minute: u32,
    pub min_delay_between_requests: Duration,
    pub respect_robots_txt: bool,
    pub transparent_user_agent: bool,
    pub contact_info: Option<String>,
}

impl Default for EthicalConfig {
    fn default() -> Self {
        Self {
            max_requests_per_minute: 30, // Konservativ: 30 Anfragen pro Minute
            min_delay_between_requests: Duration::from_secs(2), // Minimum 2 Sekunden
            respect_robots_txt: true,
            transparent_user_agent: true,
            contact_info: Some("zakyx-browser@example.com".to_string()),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct RateLimiter {
    domain_limits: HashMap<String, DomainLimiter>,
    global_config: EthicalConfig,
}

#[derive(Debug)]
#[allow(dead_code)]
struct DomainLimiter {
    last_request: Instant,
    request_count: u32,
    minute_start: Instant,
}

impl RateLimiter {
    #[allow(dead_code)]
    pub fn new(config: EthicalConfig) -> Self {
        Self {
            domain_limits: HashMap::new(),
            global_config: config,
        }
    }
    
    #[allow(dead_code)]
    pub async fn wait_if_needed(&mut self, domain: &str) -> Result<(), String> {
        let now = Instant::now();
        
        // Hole oder erstelle Domain-spezifische Limits
        let limiter = self.domain_limits.entry(domain.to_string())
            .or_insert(DomainLimiter {
                last_request: now - Duration::from_secs(10), // Weit in der Vergangenheit
                request_count: 0,
                minute_start: now,
            });
        
        // Prüfe Minute-basierte Limits
        if now.duration_since(limiter.minute_start) >= Duration::from_secs(60) {
            limiter.request_count = 0;
            limiter.minute_start = now;
        }
        
        // Prüfe ob Limit erreicht
        if limiter.request_count >= self.global_config.max_requests_per_minute {
            let wait_time = Duration::from_secs(60) - now.duration_since(limiter.minute_start);
            println!("⏳ Rate limit reached for {}. Waiting {:?}", domain, wait_time);
            sleep(wait_time).await;
            limiter.request_count = 0;
            limiter.minute_start = Instant::now();
        }
        
        // Prüfe minimale Verzögerung zwischen Anfragen
        let time_since_last = now.duration_since(limiter.last_request);
        if time_since_last < self.global_config.min_delay_between_requests {
            let wait_time = self.global_config.min_delay_between_requests - time_since_last;
            println!("⏱️ Waiting {:?} before next request to {}", wait_time, domain);
            sleep(wait_time).await;
        }
        
        // Update Statistiken
        limiter.last_request = Instant::now();
        limiter.request_count += 1;
        
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn get_ethical_user_agent(&self, purpose: &str) -> String {
        if self.global_config.transparent_user_agent {
            let contact = self.global_config.contact_info
                .as_ref()
                .map(|c| format!("; {}", c))
                .unwrap_or_default();
            
            format!(
                "ZAKYXBrowser/1.0 (+https://github.com/zakyx-browser{}) Purpose: {}",
                contact, purpose
            )
        } else {
            // Fallback zu Standard User-Agent
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string()
        }
    }
    
    #[allow(dead_code)]
    pub fn log_request(&self, domain: &str, purpose: &str, success: bool) {
        let status = if success { "✅ SUCCESS" } else { "❌ FAILED" };
        println!("📊 [ETHICAL LOG] {} - {} - Purpose: {} - Domain: {}", 
                 chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
                 status, purpose, domain);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct EthicalValidator;

impl EthicalValidator {
    /// Prüft ob eine URL ethisch vertretbar ist
    #[allow(dead_code)]
    pub fn validate_request(url: &str, purpose: &str) -> Result<(), String> {
        // Prüfe auf offensichtlich problematische Patterns
        let problematic_patterns = [
            "admin", "login", "password", "private", "internal",
            "secret", "confidential", "restricted", "auth"
        ];
        
        let url_lower = url.to_lowercase();
        for pattern in &problematic_patterns {
            if url_lower.contains(pattern) {
                return Err(format!(
                    "🚫 Potentially unethical request detected: URL contains '{}'. Purpose: {}",
                    pattern, purpose
                ));
            }
        }
        
        // Prüfe Purpose auf problematische Absichten
        let problematic_purposes = [
            "scraping", "mass download", "ddos", "spam", "fraud",
            "bypass paywall", "steal data", "hack"
        ];
        
        let purpose_lower = purpose.to_lowercase();
        for pattern in &problematic_purposes {
            if purpose_lower.contains(pattern) {
                return Err(format!(
                    "🚫 Potentially unethical purpose detected: '{}' in purpose: {}",
                    pattern, purpose
                ));
            }
        }
        
        Ok(())
    }
    
    /// Generiert ethische Empfehlungen basierend auf der Domain
    #[allow(dead_code)]
    pub fn get_ethical_recommendations(domain: &str) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Banking/Finance
        if domain.contains("bank") || domain.contains("paypal") || domain.contains("finance") {
            recommendations.push("🏦 Banking site detected: Ensure you have legitimate access rights".to_string());
            recommendations.push("💳 Consider using official APIs instead of scraping".to_string());
        }
        
        // Social Media
        if domain.contains("facebook") || domain.contains("twitter") || domain.contains("instagram") {
            recommendations.push("📱 Social media site: Respect user privacy and platform ToS".to_string());
            recommendations.push("👥 Consider rate limiting to avoid impacting other users".to_string());
        }
        
        // News/Media
        if domain.contains("news") || domain.contains("media") || domain.contains("journal") {
            recommendations.push("📰 News site detected: Consider supporting journalism by subscribing".to_string());
            recommendations.push("📖 Respect copyright and fair use guidelines".to_string());
        }
        
        // E-Commerce
        if domain.contains("shop") || domain.contains("store") || domain.contains("amazon") || domain.contains("ebay") {
            recommendations.push("🛒 E-commerce site: Avoid automated purchasing or price manipulation".to_string());
            recommendations.push("💰 Respect terms of service regarding automated access".to_string());
        }
        
        // Government
        if domain.contains(".gov") || domain.contains("government") || domain.contains("official") {
            recommendations.push("🏛️ Government site: Public information access is generally acceptable".to_string());
            recommendations.push("📋 Ensure compliance with local laws and regulations".to_string());
        }
        
        // Default recommendations
        if recommendations.is_empty() {
            recommendations.push("✅ General site: Follow robots.txt and be respectful of server resources".to_string());
            recommendations.push("🤝 Consider contacting site owners for large-scale data needs".to_string());
        }
        
        recommendations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_rate_limiter() {
        let config = EthicalConfig {
            max_requests_per_minute: 2,
            min_delay_between_requests: Duration::from_millis(100),
            ..Default::default()
        };
        
        let mut limiter = RateLimiter::new(config);
        
        // Erste Anfrage sollte sofort durchgehen
        assert!(limiter.wait_if_needed("example.com").await.is_ok());
        
        // Zweite Anfrage sollte warten
        let start = Instant::now();
        assert!(limiter.wait_if_needed("example.com").await.is_ok());
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(100));
    }
    
    #[test]
    fn test_ethical_validator() {
        // Legitime Anfrage
        assert!(EthicalValidator::validate_request("https://example.com", "research").is_ok());
        
        // Problematische URL
        assert!(EthicalValidator::validate_request("https://example.com/admin", "research").is_err());
        
        // Problematischer Purpose
        assert!(EthicalValidator::validate_request("https://example.com", "scraping for profit").is_err());
    }
    
    #[test]
    fn test_ethical_recommendations() {
        let recommendations = EthicalValidator::get_ethical_recommendations("bank.com");
        assert!(!recommendations.is_empty());
        assert!(recommendations.iter().any(|r| r.contains("Banking")));
    }
} 
