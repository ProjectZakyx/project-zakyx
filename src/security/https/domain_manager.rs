// 🏠 Domain Manager für HTTPS-Ausnahmen
// Verwaltet HTTP-Ausnahmen und Domain-spezifische Einstellungen

use anyhow::Result;
use std::collections::HashSet;

pub struct HttpsDomainManager {
    allowed_http_domains: Vec<String>,
    blocked_domains: HashSet<String>,
    trusted_domains: HashSet<String>,
}

impl HttpsDomainManager {
    pub fn new() -> Self {
        Self {
            allowed_http_domains: vec![
                "localhost".to_string(),
                "127.0.0.1".to_string(),
                "::1".to_string(),
            ],
            blocked_domains: HashSet::new(),
            trusted_domains: HashSet::new(),
        }
    }

    /// Füge HTTP-Ausnahme hinzu
    pub fn add_http_exception(&mut self, domain: String) -> Result<()> {
        if !self.allowed_http_domains.contains(&domain) {
            self.allowed_http_domains.push(domain.clone());
            println!("🟡 Added HTTP exception for domain: {}", domain);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Domain already in HTTP exceptions: {}", domain))
        }
    }

    /// Entferne HTTP-Ausnahme
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

    /// Prüfe ob HTTP für Domain erlaubt ist
    pub fn is_http_allowed(&self, domain: &str) -> bool {
        // Check exact matches
        if self.allowed_http_domains.contains(&domain.to_string()) {
            return true;
        }

        // Check if it's a localhost variant
        if self.is_localhost_domain(domain) {
            return true;
        }

        // Check if it's a private IP
        if self.is_private_ip(domain) {
            return true;
        }

        false
    }

    /// Prüfe ob Domain localhost ist
    pub fn is_localhost_domain(&self, domain: &str) -> bool {
        domain == "localhost" ||
        domain.starts_with("localhost:") ||
        domain == "127.0.0.1" ||
        domain.starts_with("127.0.0.1:") ||
        domain == "::1" ||
        domain.starts_with("[::1]")
    }

    /// Prüfe ob Domain eine private IP ist
    pub fn is_private_ip(&self, domain: &str) -> bool {
        // Remove port if present
        let ip = if let Some(colon_pos) = domain.find(':') {
            &domain[..colon_pos]
        } else {
            domain
        };

        // Check IPv4 private ranges
        if ip.starts_with("192.168.") ||
           ip.starts_with("10.") ||
           (ip.starts_with("172.") && self.is_private_ip_range_172(ip)) {
            return true;
        }

        // Check IPv6 private ranges
        if ip.starts_with("fc00:") || ip.starts_with("fd00:") || ip == "::1" {
            return true;
        }

        false
    }

    /// Prüfe 172.x.x.x private range (172.16.0.0 - 172.31.255.255)
    fn is_private_ip_range_172(&self, ip: &str) -> bool {
        if let Some(second_octet) = ip.split('.').nth(1) {
            if let Ok(octet) = second_octet.parse::<u8>() {
                return (16..=31).contains(&octet);
            }
        }
        false
    }

    /// Füge Domain zur Blocklist hinzu
    pub fn block_domain(&mut self, domain: String) -> Result<()> {
        if self.blocked_domains.insert(domain.clone()) {
            println!("🚫 Blocked domain: {}", domain);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Domain already blocked: {}", domain))
        }
    }

    /// Entferne Domain von Blocklist
    pub fn unblock_domain(&mut self, domain: &str) -> Result<()> {
        if self.blocked_domains.remove(domain) {
            println!("✅ Unblocked domain: {}", domain);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Domain not found in blocklist: {}", domain))
        }
    }

    /// Prüfe ob Domain blockiert ist
    pub fn is_domain_blocked(&self, domain: &str) -> bool {
        self.blocked_domains.contains(domain)
    }

    /// Füge vertrauenswürdige Domain hinzu
    pub fn add_trusted_domain(&mut self, domain: String) -> Result<()> {
        if self.trusted_domains.insert(domain.clone()) {
            println!("🛡️ Added trusted domain: {}", domain);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Domain already trusted: {}", domain))
        }
    }

    /// Entferne vertrauenswürdige Domain
    pub fn remove_trusted_domain(&mut self, domain: &str) -> Result<()> {
        if self.trusted_domains.remove(domain) {
            println!("❌ Removed trusted domain: {}", domain);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Domain not found in trusted list: {}", domain))
        }
    }

    /// Prüfe ob Domain vertrauenswürdig ist
    pub fn is_domain_trusted(&self, domain: &str) -> bool {
        self.trusted_domains.contains(domain)
    }

    /// Bulk-Import von Domains
    pub fn import_domains(&mut self, domains: Vec<String>, list_type: DomainListType) -> Result<usize> {
        let mut imported = 0;
        
        for domain in domains {
            let result = match list_type {
                DomainListType::HttpAllowed => self.add_http_exception(domain),
                DomainListType::Blocked => self.block_domain(domain),
                DomainListType::Trusted => self.add_trusted_domain(domain),
            };
            
            if result.is_ok() {
                imported += 1;
            }
        }
        
        println!("📥 Imported {} domains to {:?} list", imported, list_type);
        Ok(imported)
    }

    /// Bulk-Export von Domains
    pub fn export_domains(&self, list_type: DomainListType) -> Vec<String> {
        match list_type {
            DomainListType::HttpAllowed => self.allowed_http_domains.clone(),
            DomainListType::Blocked => self.blocked_domains.iter().cloned().collect(),
            DomainListType::Trusted => self.trusted_domains.iter().cloned().collect(),
        }
    }

    /// Bereinige Domain-Listen (entferne Duplikate, sortiere)
    pub fn cleanup_domain_lists(&mut self) -> usize {
        let initial_count = self.allowed_http_domains.len();
        
        // Remove duplicates from HTTP allowed list
        let mut unique_domains = Vec::new();
        for domain in &self.allowed_http_domains {
            if !unique_domains.contains(domain) {
                unique_domains.push(domain.clone());
            }
        }
        
        // Sort domains
        unique_domains.sort();
        self.allowed_http_domains = unique_domains;
        
        let removed = initial_count - self.allowed_http_domains.len();
        if removed > 0 {
            println!("🧹 Cleaned up {} duplicate domains", removed);
        }
        
        removed
    }

    /// Suche Domains nach Pattern
    pub fn search_domains(&self, pattern: &str, list_type: DomainListType) -> Vec<String> {
        let pattern_lower = pattern.to_lowercase();
        let domains = self.export_domains(list_type);
        
        domains.into_iter()
            .filter(|domain| domain.to_lowercase().contains(&pattern_lower))
            .collect()
    }

    /// Statistiken
    pub fn get_domain_stats(&self) -> DomainStats {
        DomainStats {
            http_allowed_count: self.allowed_http_domains.len(),
            blocked_count: self.blocked_domains.len(),
            trusted_count: self.trusted_domains.len(),
            localhost_variants: self.count_localhost_variants(),
            private_ips: self.count_private_ips(),
        }
    }

    /// Zähle localhost-Varianten
    fn count_localhost_variants(&self) -> usize {
        self.allowed_http_domains.iter()
            .filter(|domain| self.is_localhost_domain(domain))
            .count()
    }

    /// Zähle private IPs
    fn count_private_ips(&self) -> usize {
        self.allowed_http_domains.iter()
            .filter(|domain| self.is_private_ip(domain))
            .count()
    }

    // Getter
    pub fn get_allowed_http_domains(&self) -> &[String] {
        &self.allowed_http_domains
    }

    pub fn get_blocked_domains(&self) -> &HashSet<String> {
        &self.blocked_domains
    }

    pub fn get_trusted_domains(&self) -> &HashSet<String> {
        &self.trusted_domains
    }
}

#[derive(Debug, Clone)]
pub enum DomainListType {
    HttpAllowed,
    Blocked,
    Trusted,
}

#[derive(Debug)]
pub struct DomainStats {
    pub http_allowed_count: usize,
    pub blocked_count: usize,
    pub trusted_count: usize,
    pub localhost_variants: usize,
    pub private_ips: usize,
}

impl Default for HttpsDomainManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_exception_management() {
        let mut manager = HttpsDomainManager::new();
        
        manager.add_http_exception("example.com".to_string()).unwrap();
        assert!(manager.is_http_allowed("example.com"));
        
        manager.remove_http_exception("example.com").unwrap();
        assert!(!manager.is_http_allowed("example.com"));
    }

    #[test]
    fn test_localhost_detection() {
        let manager = HttpsDomainManager::new();
        
        assert!(manager.is_localhost_domain("localhost"));
        assert!(manager.is_localhost_domain("localhost:3000"));
        assert!(manager.is_localhost_domain("127.0.0.1"));
        assert!(manager.is_localhost_domain("127.0.0.1:8080"));
        assert!(!manager.is_localhost_domain("example.com"));
    }

    #[test]
    fn test_private_ip_detection() {
        let manager = HttpsDomainManager::new();
        
        assert!(manager.is_private_ip("192.168.1.1"));
        assert!(manager.is_private_ip("10.0.0.1"));
        assert!(manager.is_private_ip("172.16.0.1"));
        assert!(!manager.is_private_ip("8.8.8.8"));
        assert!(!manager.is_private_ip("example.com"));
    }

    #[test]
    fn test_domain_blocking() {
        let mut manager = HttpsDomainManager::new();
        
        manager.block_domain("malicious.com".to_string()).unwrap();
        assert!(manager.is_domain_blocked("malicious.com"));
        
        manager.unblock_domain("malicious.com").unwrap();
        assert!(!manager.is_domain_blocked("malicious.com"));
    }

    #[test]
    fn test_trusted_domains() {
        let mut manager = HttpsDomainManager::new();
        
        manager.add_trusted_domain("trusted.com".to_string()).unwrap();
        assert!(manager.is_domain_trusted("trusted.com"));
        
        manager.remove_trusted_domain("trusted.com").unwrap();
        assert!(!manager.is_domain_trusted("trusted.com"));
    }

    #[test]
    fn test_domain_search() {
        let mut manager = HttpsDomainManager::new();
        manager.add_http_exception("example.com".to_string()).unwrap();
        manager.add_http_exception("test.example.com".to_string()).unwrap();
        
        let results = manager.search_domains("example", DomainListType::HttpAllowed);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&"example.com".to_string()));
        assert!(results.contains(&"test.example.com".to_string()));
    }
}