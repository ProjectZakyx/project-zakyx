// 🔒 HTTPS Enforcer - Force secure connections and upgrade HTTP to HTTPS

use anyhow::Result;

pub struct HTTPSEnforcer {
    enabled: bool,
    enforce_strict: bool,
    upgraded_urls: u64,
    blocked_http: u64,
    allowed_http_domains: Vec<String>,
    hsts_enabled: bool,
    certificate_validation: bool,
    mixed_content_blocking: bool,
}

impl HTTPSEnforcer {
    pub fn new() -> Self {
        println!("🔒 Initializing HTTPS Enforcer...");
        
        Self {
            enabled: true,
            enforce_strict: false,
            upgraded_urls: 0,
            blocked_http: 0,
            allowed_http_domains: vec![
                "localhost".to_string(),
                "127.0.0.1".to_string(),
                "::1".to_string(),
            ],
            hsts_enabled: true,
            certificate_validation: true,
            mixed_content_blocking: true,
        }
    }

    pub fn process_url(&mut self, url: &str) -> Result<String> {
        if !self.enabled {
            return Ok(url.to_string());
        }

        // Check if URL is already HTTPS
        if url.starts_with("https://") {
            return Ok(url.to_string());
        }

        // Check if URL is HTTP
        if url.starts_with("http://") {
            return self.handle_http_url(url);
        }

        // Handle URLs without protocol
        if !url.contains("://") {
            let assumed_https = format!("https://{}", url);
            println!("🔒 Assuming HTTPS for: {} → {}", url, assumed_https);
            return Ok(assumed_https);
        }

        // For other protocols (ftp, file, etc.), pass through
        Ok(url.to_string())
    }

    fn handle_http_url(&mut self, url: &str) -> Result<String> {
        // Extract domain from URL
        let domain = self.extract_domain(url);
        
        // Check if domain is in allowed HTTP list
        if self.is_http_allowed(&domain) {
            println!("🟡 HTTP allowed for domain: {}", domain);
            return Ok(url.to_string());
        }

        // In strict mode, block HTTP entirely
        if self.enforce_strict {
            self.blocked_http += 1;
            println!("🚫 Blocked HTTP request in strict mode: {}", url);
            return Err(anyhow::anyhow!("HTTP connections blocked in strict mode"));
        }

        // Upgrade HTTP to HTTPS
        let https_url = url.replace("http://", "https://");
        self.upgraded_urls += 1;
        
        println!("🔒 Upgraded HTTP to HTTPS: {} → {}", url, https_url);
        Ok(https_url)
    }

    fn extract_domain(&self, url: &str) -> String {
        if let Some(start) = url.find("://") {
            let after_protocol = &url[start + 3..];
            if let Some(end) = after_protocol.find('/') {
                after_protocol[..end].to_string()
            } else if let Some(end) = after_protocol.find(':') {
                after_protocol[..end].to_string()
            } else {
                after_protocol.to_string()
            }
        } else {
            url.to_string()
        }
    }

    fn is_http_allowed(&self, domain: &str) -> bool {
        // Check exact matches
        if self.allowed_http_domains.contains(&domain.to_string()) {
            return true;
        }

        // Check if it's a localhost variant
        if domain.starts_with("localhost") || 
           domain.starts_with("127.0.0.1") || 
           domain.starts_with("192.168.") ||
           domain.starts_with("10.") ||
           (domain.starts_with("172.") && self.is_private_ip_range(domain)) {
            return true;
        }

        false
    }

    fn is_private_ip_range(&self, domain: &str) -> bool {
        if let Some(second_octet) = domain.split('.').nth(1) {
            if let Ok(octet) = second_octet.parse::<u8>() {
                return (16..=31).contains(&octet);
            }
        }
        false
    }

    pub fn add_http_exception(&mut self, domain: String) -> Result<()> {
        if !self.allowed_http_domains.contains(&domain) {
            self.allowed_http_domains.push(domain.clone());
            println!("🟡 Added HTTP exception for domain: {}", domain);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Domain already in HTTP exceptions: {}", domain))
        }
    }

    pub fn remove_http_exception(&mut self, domain: &str) -> Result<()> {
        let initial_len = self.allowed_http_domains.len();
        self.allowed_http_domains.retain(|d| d != domain);
        
        if self.allowed_http_domains.len() < initial_len {
            println!("🗑️ Removed HTTP exception for domain: {}", domain);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Domain not found in HTTP exceptions: {}", domain))
        }
    }

    pub fn toggle_enabled(&mut self) -> bool {
        self.enabled = !self.enabled;
        println!(
            "🔒 HTTPS Enforcer: {}",
            if self.enabled { "Enabled" } else { "Disabled" }
        );
        self.enabled
    }

    pub fn toggle_strict_mode(&mut self) -> bool {
        self.enforce_strict = !self.enforce_strict;
        println!(
            "🔒 HTTPS Strict Mode: {}",
            if self.enforce_strict { "Enabled" } else { "Disabled" }
        );
        self.enforce_strict
    }

    pub fn toggle_hsts(&mut self) -> bool {
        self.hsts_enabled = !self.hsts_enabled;
        println!(
            "🔒 HSTS (HTTP Strict Transport Security): {}",
            if self.hsts_enabled { "Enabled" } else { "Disabled" }
        );
        self.hsts_enabled
    }

    pub fn toggle_certificate_validation(&mut self) -> bool {
        self.certificate_validation = !self.certificate_validation;
        println!(
            "🔒 Certificate Validation: {}",
            if self.certificate_validation { "Enabled" } else { "Disabled" }
        );
        self.certificate_validation
    }

    pub fn toggle_mixed_content_blocking(&mut self) -> bool {
        self.mixed_content_blocking = !self.mixed_content_blocking;
        println!(
            "🔒 Mixed Content Blocking: {}",
            if self.mixed_content_blocking { "Enabled" } else { "Disabled" }
        );
        self.mixed_content_blocking
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_strict_mode(&self) -> bool {
        self.enforce_strict
    }

    pub fn get_upgrade_count(&self) -> u64 {
        self.upgraded_urls
    }

    pub fn get_blocked_count(&self) -> u64 {
        self.blocked_http
    }

    pub fn reset_stats(&mut self) {
        self.upgraded_urls = 0;
        self.blocked_http = 0;
        println!("📊 HTTPS Enforcer stats reset");
    }

    pub fn get_security_score(&self) -> u8 {
        let mut score = 0;
        
        if self.enabled { score += 25; }
        if self.enforce_strict { score += 20; }
        if self.hsts_enabled { score += 20; }
        if self.certificate_validation { score += 20; }
        if self.mixed_content_blocking { score += 15; }
        
        score
    }

    pub fn get_security_level(&self) -> String {
        let score = self.get_security_score();
        
        match score {
            0..=30 => "🔴 Niedrig".to_string(),
            31..=60 => "🟡 Mittel".to_string(),
            61..=80 => "🟢 Hoch".to_string(),
            81..=100 => "💪 Maximal".to_string(),
            _ => "❓ Unbekannt".to_string(),
        }
    }

    pub fn get_https_display(&self) -> Vec<String> {
        vec![
            "🔒 HTTPS ENFORCER".to_string(),
            "================".to_string(),
            "".to_string(),
            format!("Status: {}", if self.enabled { "✅ Enabled" } else { "❌ Disabled" }),
            format!("Strict Mode: {}", if self.enforce_strict { "✅ Enabled" } else { "❌ Disabled" }),
            format!("Security Score: {}/100", self.get_security_score()),
            format!("Security Level: {}", self.get_security_level()),
            "".to_string(),
            "📊 STATISTIKEN:".to_string(),
            format!("• Upgraded URLs: {}", self.upgraded_urls),
            format!("• Blocked HTTP: {}", self.blocked_http),
            format!("• HTTP Exceptions: {}", self.allowed_http_domains.len()),
            "".to_string(),
            "🔒 SECURITY FEATURES:".to_string(),
            format!(
                "• HSTS: {}",
                if self.hsts_enabled { "✅ Enabled" } else { "❌ Disabled" }
            ),
            format!(
                "• Certificate Validation: {}",
                if self.certificate_validation { "✅ Enabled" } else { "❌ Disabled" }
            ),
            format!(
                "• Mixed Content Blocking: {}",
                if self.mixed_content_blocking { "✅ Enabled" } else { "❌ Disabled" }
            ),
            "".to_string(),
            "🟡 HTTP EXCEPTIONS:".to_string(),
        ]
        .into_iter()
        .chain(
            if self.allowed_http_domains.is_empty() {
                vec!["   • Keine HTTP-Ausnahmen konfiguriert".to_string()]
            } else {
                self.allowed_http_domains
                    .iter()
                    .take(10)
                    .map(|domain| format!("   • {}", domain))
                    .collect()
            }
        )
        .chain(
            if self.allowed_http_domains.len() > 10 {
                vec![format!("   ... und {} weitere", self.allowed_http_domains.len() - 10)]
            } else {
                vec![]
            }
        )
        .chain(vec![
            "".to_string(),
            "💡 KOMMANDOS:".to_string(),
            "• 'https toggle' → Ein/Aus".to_string(),
            "• 'https strict' → Strict Mode".to_string(),
            "• 'https hsts' → HSTS Ein/Aus".to_string(),
            "• 'https stats' → Statistiken".to_string(),
        ])
        .collect()
    }

    pub fn get_detailed_security_report(&self) -> Vec<String> {
        vec![
            "🔒 DETAILLIERTER HTTPS-SECURITY REPORT".to_string(),
            "======================================".to_string(),
            "".to_string(),
            format!("📊 SECURITY SCORE: {}/100", self.get_security_score()),
            format!("🏆 SECURITY LEVEL: {}", self.get_security_level()),
            "".to_string(),
            "🎯 FEATURE-BEWERTUNG:".to_string(),
            format!("   • HTTPS Enforcer: {}", if self.enabled { "✅ +25" } else { "❌ 0" }),
            format!("   • Strict Mode: {}", if self.enforce_strict { "✅ +20" } else { "❌ 0" }),
            format!("   • HSTS: {}", if self.hsts_enabled { "✅ +20" } else { "❌ 0" }),
            format!("   • Certificate Validation: {}", if self.certificate_validation { "✅ +20" } else { "❌ 0" }),
            format!("   • Mixed Content Blocking: {}", if self.mixed_content_blocking { "✅ +15" } else { "❌ 0" }),
            "".to_string(),
            "📈 PERFORMANCE STATISTIKEN:".to_string(),
            format!("   • Upgraded HTTP → HTTPS: {}", self.upgraded_urls),
            format!("   • Blocked HTTP Requests: {}", self.blocked_http),
            format!("   • Upgrade Rate: {:.1}%", self.calculate_upgrade_rate()),
            "".to_string(),
            "🛡️ EMPFEHLUNGEN:".to_string(),
        ]
        .into_iter()
        .chain(self.get_security_recommendations())
        .collect()
    }

    fn calculate_upgrade_rate(&self) -> f32 {
        let total_http = self.upgraded_urls + self.blocked_http;
        if total_http > 0 {
            (self.upgraded_urls as f32 / total_http as f32) * 100.0
        } else {
            0.0
        }
    }

    fn get_security_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if !self.enabled {
            recommendations.push("• HTTPS Enforcer aktivieren".to_string());
        }
        
        if !self.enforce_strict {
            recommendations.push("• Strict Mode für maximale Sicherheit aktivieren".to_string());
        }
        
        if !self.hsts_enabled {
            recommendations.push("• HSTS (HTTP Strict Transport Security) aktivieren".to_string());
        }
        
        if !self.certificate_validation {
            recommendations.push("• Certificate Validation aktivieren".to_string());
        }
        
        if !self.mixed_content_blocking {
            recommendations.push("• Mixed Content Blocking aktivieren".to_string());
        }
        
        if self.allowed_http_domains.len() > 5 {
            recommendations.push("• HTTP-Ausnahmen reduzieren".to_string());
        }
        
        if recommendations.is_empty() {
            recommendations.push("🎉 Alle HTTPS-Sicherheitseinstellungen optimal!".to_string());
        }
        
        recommendations
    }

    pub fn export_configuration(&self) -> Result<String> {
        let config = serde_json::json!({
            "enabled": self.enabled,
            "enforce_strict": self.enforce_strict,
            "hsts_enabled": self.hsts_enabled,
            "certificate_validation": self.certificate_validation,
            "mixed_content_blocking": self.mixed_content_blocking,
            "allowed_http_domains": self.allowed_http_domains,
            "stats": {
                "upgraded_urls": self.upgraded_urls,
                "blocked_http": self.blocked_http
            }
        });
        
        let json_data = serde_json::to_string_pretty(&config)?;
        println!("📤 Exported HTTPS Enforcer configuration");
        Ok(json_data)
    }

    pub fn import_configuration(&mut self, json_data: &str) -> Result<()> {
        let config: serde_json::Value = serde_json::from_str(json_data)?;
        
        if let Some(enabled) = config["enabled"].as_bool() {
            self.enabled = enabled;
        }
        
        if let Some(strict) = config["enforce_strict"].as_bool() {
            self.enforce_strict = strict;
        }
        
        if let Some(hsts) = config["hsts_enabled"].as_bool() {
            self.hsts_enabled = hsts;
        }
        
        if let Some(cert_val) = config["certificate_validation"].as_bool() {
            self.certificate_validation = cert_val;
        }
        
        if let Some(mixed_content) = config["mixed_content_blocking"].as_bool() {
            self.mixed_content_blocking = mixed_content;
        }
        
        if let Some(domains) = config["allowed_http_domains"].as_array() {
            self.allowed_http_domains = domains
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
        }
        
        println!("📥 Imported HTTPS Enforcer configuration");
        Ok(())
    }

    pub fn apply_security_preset(&mut self, preset: &str) -> Result<()> {
        match preset.to_lowercase().as_str() {
            "minimal" => {
                self.enabled = true;
                self.enforce_strict = false;
                self.hsts_enabled = false;
                self.certificate_validation = true;
                self.mixed_content_blocking = false;
                println!("🔵 Applied minimal HTTPS security preset");
            }
            "balanced" => {
                self.enabled = true;
                self.enforce_strict = false;
                self.hsts_enabled = true;
                self.certificate_validation = true;
                self.mixed_content_blocking = true;
                println!("🟡 Applied balanced HTTPS security preset");
            }
            "strict" => {
                self.enabled = true;
                self.enforce_strict = true;
                self.hsts_enabled = true;
                self.certificate_validation = true;
                self.mixed_content_blocking = true;
                println!("🔴 Applied strict HTTPS security preset");
            }
            "paranoid" => {
                self.enabled = true;
                self.enforce_strict = true;
                self.hsts_enabled = true;
                self.certificate_validation = true;
                self.mixed_content_blocking = true;
                self.allowed_http_domains.clear(); // Remove all exceptions
                println!("🖤 Applied paranoid HTTPS security preset");
            }
            _ => {
                return Err(anyhow::anyhow!("Unknown HTTPS security preset: {}", preset));
            }
        }
        
        Ok(())
    }
}

impl Default for HTTPSEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

// 🧪 TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_https_enforcer_creation() {
        let enforcer = HTTPSEnforcer::new();
        assert!(enforcer.is_enabled());
        assert!(!enforcer.is_strict_mode());
    }

    #[test]
    fn test_https_url_processing() {
        let mut enforcer = HTTPSEnforcer::new();
        
        // HTTPS URLs should pass through unchanged
        let result = enforcer.process_url("https://example.com").unwrap();
        assert_eq!(result, "https://example.com");
        
        // HTTP URLs should be upgraded
        let result = enforcer.process_url("http://example.com").unwrap();
        assert_eq!(result, "https://example.com");
        assert_eq!(enforcer.get_upgrade_count(), 1);
    }

    #[test]
    fn test_http_exceptions() {
        let mut enforcer = HTTPSEnforcer::new();
        
        // Localhost should be allowed
        let result = enforcer.process_url("http://localhost:3000").unwrap();
        assert_eq!(result, "http://localhost:3000");
        
        // Regular domains should be upgraded
        let result = enforcer.process_url("http://example.com").unwrap();
        assert_eq!(result, "https://example.com");
    }

    #[test]
    fn test_strict_mode() {
        let mut enforcer = HTTPSEnforcer::new();
        enforcer.toggle_strict_mode();
        
        // HTTP should be blocked in strict mode
        let result = enforcer.process_url("http://example.com");
        assert!(result.is_err());
        assert_eq!(enforcer.get_blocked_count(), 1);
    }

    #[test]
    fn test_domain_extraction() {
        let enforcer = HTTPSEnforcer::new();
        
        assert_eq!(enforcer.extract_domain("http://example.com/path"), "example.com");
        assert_eq!(enforcer.extract_domain("https://sub.example.com:8080/path"), "sub.example.com");
        assert_eq!(enforcer.extract_domain("http://localhost:3000"), "localhost");
    }

    #[test]
    fn test_http_exception_management() {
        let mut enforcer = HTTPSEnforcer::new();
        
        enforcer.add_http_exception("example.com".to_string()).unwrap();
        assert!(enforcer.allowed_http_domains.contains(&"example.com".to_string()));
        
        enforcer.remove_http_exception("example.com").unwrap();
        assert!(!enforcer.allowed_http_domains.contains(&"example.com".to_string()));
    }

    #[test]
    fn test_security_score() {
        let enforcer = HTTPSEnforcer::new();
        let score = enforcer.get_security_score();
        assert!(score > 0);
        assert!(score <= 100);
    }

    #[test]
    fn test_security_presets() {
        let mut enforcer = HTTPSEnforcer::new();
        
        enforcer.apply_security_preset("strict").unwrap();
        assert!(enforcer.enforce_strict);
        assert!(enforcer.hsts_enabled);
        
        enforcer.apply_security_preset("minimal").unwrap();
        assert!(!enforcer.enforce_strict);
        assert!(!enforcer.hsts_enabled);
    }
}