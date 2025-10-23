// 📊 HTTPS Statistics & Reporting
// Sammelt Metriken und erstellt Berichte über HTTPS-Aktivitäten

use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct HttpsStats {
    pub upgraded_urls: u64,
    pub blocked_http: u64,
    pub total_requests: u64,
    pub start_time: DateTime<Utc>,
    pub domain_stats: HashMap<String, DomainMetrics>,
}

#[derive(Debug, Clone)]
pub struct DomainMetrics {
    pub http_requests: u64,
    pub https_requests: u64,
    pub upgrades: u64,
    pub blocks: u64,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

pub struct HttpsStatsReporter {
    stats: HttpsStats,
    enabled: bool,
}

impl HttpsStatsReporter {
    pub fn new() -> Self {
        Self {
            stats: HttpsStats {
                upgraded_urls: 0,
                blocked_http: 0,
                total_requests: 0,
                start_time: Utc::now(),
                domain_stats: HashMap::new(),
            },
            enabled: true,
        }
    }

    /// Registriere URL-Upgrade
    pub fn record_upgrade(&mut self, original_url: &str, upgraded_url: &str) {
        if !self.enabled {
            return;
        }

        self.stats.upgraded_urls += 1;
        self.stats.total_requests += 1;

        let domain = self.extract_domain(original_url);
        self.update_domain_metrics(&domain, |metrics| {
            metrics.upgrades += 1;
            metrics.http_requests += 1;
        });

        println!("📈 Recorded URL upgrade: {} → {}", original_url, upgraded_url);
    }

    /// Registriere HTTP-Block
    pub fn record_block(&mut self, url: &str, reason: &str) {
        if !self.enabled {
            return;
        }

        self.stats.blocked_http += 1;
        self.stats.total_requests += 1;

        let domain = self.extract_domain(url);
        self.update_domain_metrics(&domain, |metrics| {
            metrics.blocks += 1;
            metrics.http_requests += 1;
        });

        println!("🚫 Recorded HTTP block: {} ({})", url, reason);
    }

    /// Registriere HTTPS-Request
    pub fn record_https_request(&mut self, url: &str) {
        if !self.enabled {
            return;
        }

        self.stats.total_requests += 1;

        let domain = self.extract_domain(url);
        self.update_domain_metrics(&domain, |metrics| {
            metrics.https_requests += 1;
        });
    }

    /// Aktualisiere Domain-Metriken
    fn update_domain_metrics<F>(&mut self, domain: &str, updater: F)
    where
        F: FnOnce(&mut DomainMetrics),
    {
        let now = Utc::now();
        let metrics = self.stats.domain_stats.entry(domain.to_string())
            .or_insert_with(|| DomainMetrics {
                http_requests: 0,
                https_requests: 0,
                upgrades: 0,
                blocks: 0,
                first_seen: now,
                last_seen: now,
            });

        updater(metrics);
        metrics.last_seen = now;
    }

    /// Extrahiere Domain aus URL
    fn extract_domain(&self, url: &str) -> String {
        if let Some(start) = url.find("://") {
            let after_protocol = &url[start + 3..];
            
            let domain_end = after_protocol
                .find('/')
                .or_else(|| after_protocol.find('?'))
                .or_else(|| after_protocol.find('#'))
                .unwrap_or(after_protocol.len());
            
            let domain_with_port = &after_protocol[..domain_end];
            
            if let Some(colon_pos) = domain_with_port.find(':') {
                domain_with_port[..colon_pos].to_string()
            } else {
                domain_with_port.to_string()
            }
        } else {
            url.to_string()
        }
    }

    /// Berechne Upgrade-Rate
    pub fn calculate_upgrade_rate(&self) -> f32 {
        let total_http = self.stats.upgraded_urls + self.stats.blocked_http;
        if total_http > 0 {
            (self.stats.upgraded_urls as f32 / total_http as f32) * 100.0
        } else {
            0.0
        }
    }

    /// Berechne Security Score
    pub fn calculate_security_score(&self, config: &SecurityConfig) -> u8 {
        let mut score = 0u8;
        
        // Base features
        if config.enabled { score += 25; }
        if config.enforce_strict { score += 20; }
        if config.hsts_enabled { score += 20; }
        if config.certificate_validation { score += 20; }
        if config.mixed_content_blocking { score += 15; }
        
        score
    }

    /// Bestimme Security Level
    pub fn get_security_level(&self, score: u8) -> String {
        match score {
            90..=100 => "🛡️ MAXIMUM".to_string(),
            70..=89 => "🔒 HOCH".to_string(),
            50..=69 => "🟡 MITTEL".to_string(),
            30..=49 => "🟠 NIEDRIG".to_string(),
            _ => "🔴 UNSICHER".to_string(),
        }
    }

    /// Erstelle Basis-Report
    pub fn get_basic_report(&self, config: &SecurityConfig) -> Vec<String> {
        let uptime = Utc::now().signed_duration_since(self.stats.start_time);
        let security_score = self.calculate_security_score(config);
        
        vec![
            "🔒 HTTPS ENFORCER STATUS".to_string(),
            "========================".to_string(),
            "".to_string(),
            format!("📊 Security Score: {}/100", security_score),
            format!("🏆 Security Level: {}", self.get_security_level(security_score)),
            format!("⏱️ Uptime: {} Stunden", uptime.num_hours()),
            "".to_string(),
            "📈 STATISTIKEN:".to_string(),
            format!("   • Total Requests: {}", self.stats.total_requests),
            format!("   • HTTP → HTTPS Upgrades: {}", self.stats.upgraded_urls),
            format!("   • HTTP Blocks: {}", self.stats.blocked_http),
            format!("   • Upgrade Rate: {:.1}%", self.calculate_upgrade_rate()),
            "".to_string(),
            "⚙️ KONFIGURATION:".to_string(),
            format!("   • HTTPS Enforcer: {}", if config.enabled { "✅" } else { "❌" }),
            format!("   • Strict Mode: {}", if config.enforce_strict { "✅" } else { "❌" }),
            format!("   • HSTS: {}", if config.hsts_enabled { "✅" } else { "❌" }),
            format!("   • Certificate Validation: {}", if config.certificate_validation { "✅" } else { "❌" }),
            format!("   • Mixed Content Blocking: {}", if config.mixed_content_blocking { "✅" } else { "❌" }),
        ]
    }

    /// Erstelle detaillierten Report
    pub fn get_detailed_report(&self, config: &SecurityConfig) -> Vec<String> {
        let mut report = self.get_basic_report(config);
        
        report.extend(vec![
            "".to_string(),
            "🌐 TOP DOMAINS:".to_string(),
        ]);

        // Sortiere Domains nach Aktivität
        let mut domain_activity: Vec<_> = self.stats.domain_stats.iter().collect();
        domain_activity.sort_by(|a, b| {
            let total_a = a.1.http_requests + a.1.https_requests;
            let total_b = b.1.http_requests + b.1.https_requests;
            total_b.cmp(&total_a)
        });

        for (domain, metrics) in domain_activity.iter().take(10) {
            let total_requests = metrics.http_requests + metrics.https_requests;
            let https_percentage = if total_requests > 0 {
                (metrics.https_requests as f32 / total_requests as f32) * 100.0
            } else {
                0.0
            };

            report.push(format!(
                "   • {}: {} requests ({:.1}% HTTPS, {} upgrades, {} blocks)",
                domain, total_requests, https_percentage, metrics.upgrades, metrics.blocks
            ));
        }

        report.extend(vec![
            "".to_string(),
            "🛡️ EMPFEHLUNGEN:".to_string(),
        ]);
        report.extend(self.get_security_recommendations(config));

        report
    }

    /// Erstelle Security-Empfehlungen
    pub fn get_security_recommendations(&self, config: &SecurityConfig) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if !config.enabled {
            recommendations.push("• HTTPS Enforcer aktivieren".to_string());
        }
        
        if !config.enforce_strict {
            recommendations.push("• Strict Mode für maximale Sicherheit aktivieren".to_string());
        }
        
        if !config.hsts_enabled {
            recommendations.push("• HSTS (HTTP Strict Transport Security) aktivieren".to_string());
        }
        
        if !config.certificate_validation {
            recommendations.push("• Certificate Validation aktivieren".to_string());
        }
        
        if !config.mixed_content_blocking {
            recommendations.push("• Mixed Content Blocking aktivieren".to_string());
        }
        
        // Performance-basierte Empfehlungen
        if self.calculate_upgrade_rate() < 80.0 && self.stats.blocked_http > 10 {
            recommendations.push("• Mehr Domains zu HTTP-Ausnahmen hinzufügen für bessere UX".to_string());
        }
        
        if self.stats.domain_stats.len() > 100 {
            recommendations.push("• Domain-Statistiken regelmäßig bereinigen".to_string());
        }
        
        if recommendations.is_empty() {
            recommendations.push("🎉 Alle HTTPS-Sicherheitseinstellungen optimal!".to_string());
        }
        
        recommendations
    }

    /// Exportiere Statistiken als JSON
    pub fn export_stats_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.stats)
    }

    /// Reset Statistiken
    pub fn reset_stats(&mut self) {
        self.stats = HttpsStats {
            upgraded_urls: 0,
            blocked_http: 0,
            total_requests: 0,
            start_time: Utc::now(),
            domain_stats: HashMap::new(),
        };
        println!("🔄 HTTPS statistics reset");
    }

    /// Bereinige alte Domain-Statistiken
    pub fn cleanup_old_domain_stats(&mut self, max_domains: usize) -> usize {
        if self.stats.domain_stats.len() <= max_domains {
            return 0;
        }

        // Sortiere nach letzter Aktivität
        let mut domains_by_activity: Vec<_> = self.stats.domain_stats.iter()
            .map(|(domain, metrics)| (domain.clone(), metrics.last_seen))
            .collect();
        domains_by_activity.sort_by(|a, b| b.1.cmp(&a.1));

        // Behalte nur die aktivsten Domains
        let domains_to_keep: std::collections::HashSet<_> = domains_by_activity
            .into_iter()
            .take(max_domains)
            .map(|(domain, _)| domain)
            .collect();

        let initial_count = self.stats.domain_stats.len();
        self.stats.domain_stats.retain(|domain, _| domains_to_keep.contains(domain));
        
        let removed = initial_count - self.stats.domain_stats.len();
        if removed > 0 {
            println!("🧹 Cleaned up {} old domain statistics", removed);
        }
        
        removed
    }

    // Getter & Setter
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_stats(&self) -> &HttpsStats {
        &self.stats
    }

    pub fn get_upgraded_count(&self) -> u64 {
        self.stats.upgraded_urls
    }

    pub fn get_blocked_count(&self) -> u64 {
        self.stats.blocked_http
    }

    pub fn get_total_requests(&self) -> u64 {
        self.stats.total_requests
    }
}

// Helper struct für Konfiguration
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub enabled: bool,
    pub enforce_strict: bool,
    pub hsts_enabled: bool,
    pub certificate_validation: bool,
    pub mixed_content_blocking: bool,
}

impl Default for HttpsStatsReporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> SecurityConfig {
        SecurityConfig {
            enabled: true,
            enforce_strict: false,
            hsts_enabled: true,
            certificate_validation: true,
            mixed_content_blocking: true,
        }
    }

    #[test]
    fn test_upgrade_recording() {
        let mut reporter = HttpsStatsReporter::new();
        
        reporter.record_upgrade("http://example.com", "https://example.com");
        assert_eq!(reporter.get_upgraded_count(), 1);
        assert_eq!(reporter.get_total_requests(), 1);
    }

    #[test]
    fn test_block_recording() {
        let mut reporter = HttpsStatsReporter::new();
        
        reporter.record_block("http://example.com", "strict mode");
        assert_eq!(reporter.get_blocked_count(), 1);
        assert_eq!(reporter.get_total_requests(), 1);
    }

    #[test]
    fn test_upgrade_rate_calculation() {
        let mut reporter = HttpsStatsReporter::new();
        
        reporter.record_upgrade("http://example1.com", "https://example1.com");
        reporter.record_upgrade("http://example2.com", "https://example2.com");
        reporter.record_block("http://example3.com", "strict mode");
        
        let rate = reporter.calculate_upgrade_rate();
        assert_eq!(rate, 66.666664); // 2 upgrades out of 3 total HTTP requests
    }

    #[test]
    fn test_security_score() {
        let reporter = HttpsStatsReporter::new();
        let config = create_test_config();
        
        let score = reporter.calculate_security_score(&config);
        assert_eq!(score, 80); // All features except strict mode
    }

    #[test]
    fn test_domain_metrics() {
        let mut reporter = HttpsStatsReporter::new();
        
        reporter.record_upgrade("http://example.com/page1", "https://example.com/page1");
        reporter.record_https_request("https://example.com/page2");
        
        let stats = reporter.get_stats();
        let domain_metrics = stats.domain_stats.get("example.com").unwrap();
        
        assert_eq!(domain_metrics.http_requests, 1);
        assert_eq!(domain_metrics.https_requests, 1);
        assert_eq!(domain_metrics.upgrades, 1);
    }

    #[test]
    fn test_stats_reset() {
        let mut reporter = HttpsStatsReporter::new();
        
        reporter.record_upgrade("http://example.com", "https://example.com");
        assert_eq!(reporter.get_upgraded_count(), 1);
        
        reporter.reset_stats();
        assert_eq!(reporter.get_upgraded_count(), 0);
        assert_eq!(reporter.get_total_requests(), 0);
    }
}