// 🔄 URL Rewriter für HTTPS-Upgrade
// Konvertiert HTTP-URLs zu HTTPS und verwaltet URL-Transformationen

use anyhow::Result;

pub struct HttpsUrlRewriter {
    upgrade_enabled: bool,
    strict_mode: bool,
}

impl HttpsUrlRewriter {
    pub fn new(upgrade_enabled: bool, strict_mode: bool) -> Self {
        Self {
            upgrade_enabled,
            strict_mode,
        }
    }

    /// Hauptfunktion: Verarbeite URL und führe HTTPS-Upgrade durch
    pub fn process_url(&self, url: &str, allowed_domains: &[String]) -> Result<String> {
        if !self.upgrade_enabled {
            return Ok(url.to_string());
        }

        // Check if URL is already HTTPS
        if url.starts_with("https://") {
            return Ok(url.to_string());
        }

        // Check if URL is HTTP
        if url.starts_with("http://") {
            return self.handle_http_url(url, allowed_domains);
        }

        // Handle relative URLs or other protocols
        if !url.contains("://") {
            // Assume HTTPS for protocol-relative URLs
            if url.starts_with("//") {
                return Ok(format!("https:{}", url));
            }
            // For relative paths, return as-is
            return Ok(url.to_string());
        }

        // Non-HTTP protocols (ftp, file, etc.) - pass through
        Ok(url.to_string())
    }

    /// Verarbeite HTTP-URL
    fn handle_http_url(&self, url: &str, allowed_domains: &[String]) -> Result<String> {
        let domain = self.extract_domain(url);
        
        // Check if HTTP is allowed for this domain
        if self.is_http_allowed(&domain, allowed_domains) {
            println!("🟡 HTTP allowed for domain: {}", domain);
            return Ok(url.to_string());
        }

        // In strict mode, block HTTP entirely
        if self.strict_mode {
            println!("🚫 HTTP blocked in strict mode: {}", url);
            return Err(anyhow::anyhow!(
                "HTTP connections are blocked in strict mode. Domain: {}", 
                domain
            ));
        }

        // Upgrade HTTP to HTTPS
        let https_url = url.replacen("http://", "https://", 1);
        println!("⬆️ Upgraded HTTP → HTTPS: {} → {}", url, https_url);
        Ok(https_url)
    }

    /// Extrahiere Domain aus URL
    pub fn extract_domain(&self, url: &str) -> String {
        if let Some(start) = url.find("://") {
            let after_protocol = &url[start + 3..];
            
            // Find the end of domain (before path, query, or fragment)
            let domain_end = after_protocol
                .find('/')
                .or_else(|| after_protocol.find('?'))
                .or_else(|| after_protocol.find('#'))
                .unwrap_or(after_protocol.len());
            
            let domain_with_port = &after_protocol[..domain_end];
            
            // Remove port if present
            if let Some(colon_pos) = domain_with_port.find(':') {
                domain_with_port[..colon_pos].to_string()
            } else {
                domain_with_port.to_string()
            }
        } else {
            url.to_string()
        }
    }

    /// Prüfe ob HTTP für Domain erlaubt ist
    fn is_http_allowed(&self, domain: &str, allowed_domains: &[String]) -> bool {
        // Check exact matches
        if allowed_domains.contains(&domain.to_string()) {
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

    /// Prüfe private IP-Range (172.16.0.0 - 172.31.255.255)
    fn is_private_ip_range(&self, domain: &str) -> bool {
        if let Some(second_octet) = domain.split('.').nth(1) {
            if let Ok(octet) = second_octet.parse::<u8>() {
                return (16..=31).contains(&octet);
            }
        }
        false
    }

    /// Upgrade URL von HTTP zu HTTPS
    pub fn upgrade_to_https(&self, url: &str) -> String {
        if url.starts_with("http://") {
            url.replacen("http://", "https://", 1)
        } else {
            url.to_string()
        }
    }

    /// Prüfe ob URL bereits HTTPS ist
    pub fn is_https(&self, url: &str) -> bool {
        url.starts_with("https://")
    }

    /// Prüfe ob URL HTTP ist
    pub fn is_http(&self, url: &str) -> bool {
        url.starts_with("http://")
    }

    /// Normalisiere URL (entferne trailing slash, etc.)
    pub fn normalize_url(&self, url: &str) -> String {
        let mut normalized = url.to_string();
        
        // Remove trailing slash from domain-only URLs
        if normalized.matches('/').count() == 2 && normalized.ends_with('/') {
            normalized.pop();
        }
        
        // Convert to lowercase domain
        if let Some(protocol_end) = normalized.find("://") {
            let protocol = &normalized[..protocol_end + 3];
            let rest = &normalized[protocol_end + 3..];
            
            if let Some(domain_end) = rest.find('/') {
                let domain = &rest[..domain_end];
                let path = &rest[domain_end..];
                normalized = format!("{}{}{}", protocol, domain.to_lowercase(), path);
            } else {
                normalized = format!("{}{}", protocol, rest.to_lowercase());
            }
        }
        
        normalized
    }

    /// Validiere URL-Format
    pub fn is_valid_url(&self, url: &str) -> bool {
        // Basic URL validation
        if url.is_empty() {
            return false;
        }
        
        // Must contain protocol
        if !url.contains("://") {
            return false;
        }
        
        // Must have domain after protocol
        if let Some(start) = url.find("://") {
            let after_protocol = &url[start + 3..];
            if after_protocol.is_empty() {
                return false;
            }
            
            // Domain must not start with special characters
            let first_char = after_protocol.chars().next().unwrap_or(' ');
            if first_char == '.' || first_char == '-' {
                return false;
            }
        }
        
        true
    }

    // Getter & Setter
    pub fn set_upgrade_enabled(&mut self, enabled: bool) {
        self.upgrade_enabled = enabled;
    }

    pub fn set_strict_mode(&mut self, strict: bool) {
        self.strict_mode = strict;
    }

    pub fn is_upgrade_enabled(&self) -> bool {
        self.upgrade_enabled
    }

    pub fn is_strict_mode(&self) -> bool {
        self.strict_mode
    }
}

impl Default for HttpsUrlRewriter {
    fn default() -> Self {
        Self::new(true, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_https_upgrade() {
        let rewriter = HttpsUrlRewriter::new(true, false);
        let allowed_domains = vec!["localhost".to_string()];
        
        let result = rewriter.process_url("http://example.com", &allowed_domains).unwrap();
        assert_eq!(result, "https://example.com");
    }

    #[test]
    fn test_localhost_exception() {
        let rewriter = HttpsUrlRewriter::new(true, false);
        let allowed_domains = vec!["localhost".to_string()];
        
        let result = rewriter.process_url("http://localhost:3000", &allowed_domains).unwrap();
        assert_eq!(result, "http://localhost:3000");
    }

    #[test]
    fn test_strict_mode() {
        let rewriter = HttpsUrlRewriter::new(true, true);
        let allowed_domains = vec![];
        
        let result = rewriter.process_url("http://example.com", &allowed_domains);
        assert!(result.is_err());
    }

    #[test]
    fn test_domain_extraction() {
        let rewriter = HttpsUrlRewriter::default();
        
        assert_eq!(rewriter.extract_domain("http://example.com/path"), "example.com");
        assert_eq!(rewriter.extract_domain("https://sub.example.com:8080/path"), "sub.example.com");
        assert_eq!(rewriter.extract_domain("http://localhost:3000"), "localhost");
    }

    #[test]
    fn test_url_validation() {
        let rewriter = HttpsUrlRewriter::default();
        
        assert!(rewriter.is_valid_url("https://example.com"));
        assert!(rewriter.is_valid_url("http://localhost:3000"));
        assert!(!rewriter.is_valid_url(""));
        assert!(!rewriter.is_valid_url("invalid"));
        assert!(!rewriter.is_valid_url("http://"));
    }

    #[test]
    fn test_url_normalization() {
        let rewriter = HttpsUrlRewriter::default();
        
        assert_eq!(rewriter.normalize_url("HTTPS://EXAMPLE.COM/"), "https://example.com");
        assert_eq!(rewriter.normalize_url("HTTP://SUB.EXAMPLE.COM/path"), "http://sub.example.com/path");
    }
}