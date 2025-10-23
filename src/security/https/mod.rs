// 🔒 HTTPS Security Module
// Modulare HTTPS-Sicherheit für ZAKYX Browser

pub mod url_rewriter;
pub mod domain_manager;
pub mod stats_reporter;
pub mod config_manager;

// Re-exports für einfachen Zugriff
pub use url_rewriter::HttpsUrlRewriter;
pub use domain_manager::{HttpsDomainManager, DomainListType, DomainStats};
pub use stats_reporter::{HttpsStatsReporter, HttpsStats, SecurityConfig};
pub use config_manager::{HttpsConfigManager, HttpsConfig};

// Hauptklasse für HTTPS Enforcer
use anyhow::Result;

pub struct HTTPSEnforcer {
    url_rewriter: HttpsUrlRewriter,
    domain_manager: HttpsDomainManager,
    stats_reporter: HttpsStatsReporter,
    config_manager: HttpsConfigManager,
}

impl HTTPSEnforcer {
    pub fn new() -> Self {
        println!("🔒 Initializing modular HTTPS Enforcer...");
        
        let config_manager = HttpsConfigManager::new();
        let config = config_manager.get_config();
        
        Self {
            url_rewriter: HttpsUrlRewriter::new(config.enabled, config.enforce_strict),
            domain_manager: HttpsDomainManager::new(),
            stats_reporter: HttpsStatsReporter::new(),
            config_manager,
        }
    }

    /// Verarbeite URL mit HTTPS-Upgrade
    pub fn process_url(&mut self, url: &str) -> Result<String> {
        let config = self.config_manager.get_config();
        
        if !config.enabled {
            return Ok(url.to_string());
        }

        let allowed_domains = self.domain_manager.get_allowed_http_domains();
        
        match self.url_rewriter.process_url(url, allowed_domains) {
            Ok(processed_url) => {
                // Statistiken aktualisieren
                if url != processed_url {
                    self.stats_reporter.record_upgrade(url, &processed_url);
                } else if self.url_rewriter.is_https(&processed_url) {
                    self.stats_reporter.record_https_request(&processed_url);
                }
                
                Ok(processed_url)
            }
            Err(e) => {
                // HTTP wurde blockiert
                self.stats_reporter.record_block(url, &e.to_string());
                Err(e)
            }
        }
    }

    /// Füge HTTP-Ausnahme hinzu
    pub fn add_http_exception(&mut self, domain: String) -> Result<()> {
        self.domain_manager.add_http_exception(domain)
    }

    /// Entferne HTTP-Ausnahme
    pub fn remove_http_exception(&mut self, domain: &str) -> Result<()> {
        self.domain_manager.remove_http_exception(domain)
    }

    /// Wende Security-Preset an
    pub fn apply_security_preset(&mut self, preset_name: &str) -> Result<()> {
        self.config_manager.apply_preset(preset_name)?;
        
        // Aktualisiere URL-Rewriter mit neuer Konfiguration
        let config = self.config_manager.get_config();
        self.url_rewriter.set_upgrade_enabled(config.enabled);
        self.url_rewriter.set_strict_mode(config.enforce_strict);
        
        Ok(())
    }

    /// Toggle-Funktionen
    pub fn toggle_enabled(&mut self) -> bool {
        let enabled = self.config_manager.toggle_enabled();
        self.url_rewriter.set_upgrade_enabled(enabled);
        enabled
    }

    pub fn toggle_strict_mode(&mut self) -> bool {
        let strict = self.config_manager.toggle_strict_mode();
        self.url_rewriter.set_strict_mode(strict);
        strict
    }

    pub fn toggle_hsts(&mut self) -> bool {
        self.config_manager.toggle_hsts()
    }

    pub fn toggle_certificate_validation(&mut self) -> bool {
        self.config_manager.toggle_certificate_validation()
    }

    pub fn toggle_mixed_content_blocking(&mut self) -> bool {
        self.config_manager.toggle_mixed_content_blocking()
    }

    /// Hole Status-Report
    pub fn get_status_report(&self) -> Vec<String> {
        let config = self.config_manager.get_config();
        let security_config = SecurityConfig {
            enabled: config.enabled,
            enforce_strict: config.enforce_strict,
            hsts_enabled: config.hsts_enabled,
            certificate_validation: config.certificate_validation,
            mixed_content_blocking: config.mixed_content_blocking,
        };
        
        self.stats_reporter.get_basic_report(&security_config)
    }

    /// Hole detaillierten Report
    pub fn get_detailed_security_report(&self) -> Vec<String> {
        let config = self.config_manager.get_config();
        let security_config = SecurityConfig {
            enabled: config.enabled,
            enforce_strict: config.enforce_strict,
            hsts_enabled: config.hsts_enabled,
            certificate_validation: config.certificate_validation,
            mixed_content_blocking: config.mixed_content_blocking,
        };
        
        self.stats_reporter.get_detailed_report(&security_config)
    }

    /// Exportiere Konfiguration
    pub fn export_configuration(&self) -> Result<String> {
        self.config_manager.export_configuration()
    }

    /// Importiere Konfiguration
    pub fn import_configuration(&mut self, json_data: &str) -> Result<()> {
        self.config_manager.import_configuration(json_data)?;
        
        // Aktualisiere URL-Rewriter
        let config = self.config_manager.get_config();
        self.url_rewriter.set_upgrade_enabled(config.enabled);
        self.url_rewriter.set_strict_mode(config.enforce_strict);
        
        Ok(())
    }

    /// Hole verfügbare Presets
    pub fn get_available_presets(&self) -> Vec<String> {
        self.config_manager.get_available_presets()
    }

    /// Hole Domain-Statistiken
    pub fn get_domain_stats(&self) -> DomainStats {
        self.domain_manager.get_domain_stats()
    }

    /// Reset Statistiken
    pub fn reset_stats(&mut self) {
        self.stats_reporter.reset_stats();
    }

    // Getter für Kompatibilität
    pub fn is_enabled(&self) -> bool {
        self.config_manager.get_config().enabled
    }

    pub fn is_strict_mode(&self) -> bool {
        self.config_manager.get_config().enforce_strict
    }

    pub fn is_hsts_enabled(&self) -> bool {
        self.config_manager.get_config().hsts_enabled
    }

    pub fn is_certificate_validation_enabled(&self) -> bool {
        self.config_manager.get_config().certificate_validation
    }

    pub fn is_mixed_content_blocking_enabled(&self) -> bool {
        self.config_manager.get_config().mixed_content_blocking
    }

    pub fn get_upgraded_count(&self) -> u64 {
        self.stats_reporter.get_upgraded_count()
    }

    pub fn get_blocked_count(&self) -> u64 {
        self.stats_reporter.get_blocked_count()
    }

    pub fn get_security_score(&self) -> u8 {
        let config = self.config_manager.get_config();
        let security_config = SecurityConfig {
            enabled: config.enabled,
            enforce_strict: config.enforce_strict,
            hsts_enabled: config.hsts_enabled,
            certificate_validation: config.certificate_validation,
            mixed_content_blocking: config.mixed_content_blocking,
        };
        
        self.stats_reporter.calculate_security_score(&security_config)
    }

    pub fn get_security_level(&self) -> String {
        let score = self.get_security_score();
        self.stats_reporter.get_security_level(score)
    }

    /// Extrahiere Domain aus URL (Legacy-Kompatibilität)
    pub fn extract_domain(&self, url: &str) -> String {
        self.url_rewriter.extract_domain(url)
    }

    /// Zugriff auf Submodule für erweiterte Funktionalität
    pub fn get_url_rewriter(&self) -> &HttpsUrlRewriter {
        &self.url_rewriter
    }

    pub fn get_url_rewriter_mut(&mut self) -> &mut HttpsUrlRewriter {
        &mut self.url_rewriter
    }

    pub fn get_domain_manager(&self) -> &HttpsDomainManager {
        &self.domain_manager
    }

    pub fn get_domain_manager_mut(&mut self) -> &mut HttpsDomainManager {
        &mut self.domain_manager
    }

    pub fn get_stats_reporter(&self) -> &HttpsStatsReporter {
        &self.stats_reporter
    }

    pub fn get_stats_reporter_mut(&mut self) -> &mut HttpsStatsReporter {
        &mut self.stats_reporter
    }

    pub fn get_config_manager(&self) -> &HttpsConfigManager {
        &self.config_manager
    }

    pub fn get_config_manager_mut(&mut self) -> &mut HttpsConfigManager {
        &mut self.config_manager
    }
}

impl Default for HTTPSEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

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
    fn test_url_processing() {
        let mut enforcer = HTTPSEnforcer::new();
        
        let result = enforcer.process_url("http://example.com").unwrap();
        assert_eq!(result, "https://example.com");
        assert_eq!(enforcer.get_upgraded_count(), 1);
    }

    #[test]
    fn test_http_exceptions() {
        let mut enforcer = HTTPSEnforcer::new();
        
        enforcer.add_http_exception("example.com".to_string()).unwrap();
        let result = enforcer.process_url("http://example.com").unwrap();
        assert_eq!(result, "http://example.com");
    }

    #[test]
    fn test_preset_application() {
        let mut enforcer = HTTPSEnforcer::new();
        
        enforcer.apply_security_preset("strict").unwrap();
        assert!(enforcer.is_strict_mode());
        
        let result = enforcer.process_url("http://example.com");
        assert!(result.is_err());
        assert_eq!(enforcer.get_blocked_count(), 1);
    }

    #[test]
    fn test_toggle_functions() {
        let mut enforcer = HTTPSEnforcer::new();
        
        let initial_enabled = enforcer.is_enabled();
        let toggled = enforcer.toggle_enabled();
        assert_eq!(toggled, !initial_enabled);
        assert_eq!(enforcer.is_enabled(), toggled);
    }

    #[test]
    fn test_configuration_export_import() {
        let mut enforcer1 = HTTPSEnforcer::new();
        enforcer1.toggle_strict_mode();
        
        let config_json = enforcer1.export_configuration().unwrap();
        
        let mut enforcer2 = HTTPSEnforcer::new();
        enforcer2.import_configuration(&config_json).unwrap();
        
        assert_eq!(enforcer1.is_strict_mode(), enforcer2.is_strict_mode());
    }
}