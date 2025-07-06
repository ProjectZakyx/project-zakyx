// 🛡️ Security Features Manager - Unified security system coordinator

use anyhow::Result;
use windows::Win32::Foundation::HWND;

use crate::security::{
    ad_blocker::AdBlocker,
    privacy_manager::PrivacyManager,
    password_generator::PasswordGenerator,
    https_enforcer::HTTPSEnforcer,
    types::{SecurityAlert, PrivacyMode, GeneratedPassword},
};

pub struct SecurityFeaturesManager {
    ad_blocker: AdBlocker,
    privacy_manager: PrivacyManager,
    password_generator: PasswordGenerator,
    https_enforcer: HTTPSEnforcer,
    security_alerts: Vec<SecurityAlert>,
    window_handle: HWND,
    is_initialized: bool,
}

impl SecurityFeaturesManager {
    pub fn new(window_handle: HWND) -> Result<Self> {
        println!("🛡️ Initializing Security Features Manager...");

        Ok(SecurityFeaturesManager {
            ad_blocker: AdBlocker::new(),
            privacy_manager: PrivacyManager::new(),
            password_generator: PasswordGenerator::new(),
            https_enforcer: HTTPSEnforcer::new(),
            security_alerts: Vec::new(),
            window_handle,
            is_initialized: true,
        })
    }

    // 🌐 URL PROCESSING PIPELINE
    pub fn process_url(&mut self, url: &str) -> Result<String> {
        // Step 1: Check ad-blocking
        if self.ad_blocker.should_block(url) {
            return Err(anyhow::anyhow!("URL blocked by ad-blocker: {}", url));
        }

        // Step 2: Check tracking protection
        if self.privacy_manager.should_block_tracking(url) {
            return Err(anyhow::anyhow!("URL blocked by privacy protection: {}", url));
        }

        // Step 3: HTTPS enforcement
        let processed_url = self.https_enforcer.process_url(url)?;
        
        println!("🛡️ URL processed successfully: {} → {}", url, processed_url);
        Ok(processed_url)
    }

    // 🚫 AD-BLOCKER CONTROLS
    pub fn toggle_ad_blocker(&mut self) -> bool {
        self.ad_blocker.toggle_enabled()
    }

    pub fn get_ad_blocker_stats(&self) -> Vec<String> {
        self.ad_blocker.get_stats()
    }

    pub fn reset_ad_blocker_stats(&mut self) {
        self.ad_blocker.reset_stats();
    }

    // 🔒 PRIVACY CONTROLS
    pub fn toggle_incognito(&mut self) -> bool {
        self.privacy_manager.toggle_incognito()
    }

    pub fn set_privacy_mode(&mut self, mode: PrivacyMode) -> Result<()> {
        self.privacy_manager.set_privacy_mode(mode)
    }

    pub fn get_privacy_score(&self) -> u8 {
        self.privacy_manager.get_privacy_score()
    }

    pub fn apply_privacy_preset(&mut self, preset: &str) -> Result<()> {
        self.privacy_manager.apply_privacy_preset(preset)
    }

    // 🔐 PASSWORD GENERATION
    pub fn generate_password(&mut self, length: usize, symbols: bool) -> GeneratedPassword {
        self.password_generator.generate_password(length, symbols)
    }

    pub fn generate_passphrase(&mut self, word_count: usize) -> GeneratedPassword {
        self.password_generator.generate_passphrase(word_count)
    }

    pub fn generate_pin(&mut self, length: usize) -> GeneratedPassword {
        self.password_generator.generate_pin(length)
    }

    pub fn check_password_strength(&self, password: &str) -> (crate::security::types::PasswordStrength, Vec<String>) {
        self.password_generator.check_password_strength(password)
    }

    // 🔒 HTTPS ENFORCEMENT
    pub fn toggle_https_enforcer(&mut self) -> bool {
        self.https_enforcer.toggle_enabled()
    }

    pub fn toggle_https_strict(&mut self) -> bool {
        self.https_enforcer.toggle_strict_mode()
    }

    pub fn get_https_security_score(&self) -> u8 {
        self.https_enforcer.get_security_score()
    }

    pub fn apply_https_preset(&mut self, preset: &str) -> Result<()> {
        self.https_enforcer.apply_security_preset(preset)
    }

    // 📊 OVERALL SECURITY SCORE
    pub fn get_overall_security_score(&self) -> u8 {
        let ad_score = if self.ad_blocker.is_enabled() { 20 } else { 0 };
        let privacy_score = (self.privacy_manager.get_privacy_score() as f32 * 0.3) as u8;
        let https_score = (self.https_enforcer.get_security_score() as f32 * 0.5) as u8;
        
        ad_score + privacy_score + https_score
    }

    pub fn get_security_level(&self) -> String {
        let score = self.get_overall_security_score();
        
        match score {
            0..=30 => "🔴 Niedrig".to_string(),
            31..=60 => "🟡 Mittel".to_string(),
            61..=80 => "🟢 Hoch".to_string(),
            81..=100 => "💪 Maximal".to_string(),
            _ => "❓ Unbekannt".to_string(),
        }
    }

    // 📱 DISPLAY METHODS
    pub fn get_security_overview(&self) -> Vec<String> {
        vec![
            "🛡️ SECURITY FEATURES OVERVIEW".to_string(),
            "=============================".to_string(),
            "".to_string(),
            format!("📊 Gesamt Security Score: {}/100", self.get_overall_security_score()),
            format!("🏆 Security Level: {}", self.get_security_level()),
            "".to_string(),
            "🎯 KOMPONENTEN-STATUS:".to_string(),
            format!(
                "• 🚫 Ad-Blocker: {} ({} Regeln, {} blockiert)",
                if self.ad_blocker.is_enabled() { "✅" } else { "❌" },
                self.ad_blocker.get_rule_count(),
                self.ad_blocker.get_blocked_count()
            ),
            format!(
                "• 🔒 Privacy Manager: {} (Score: {}/100)",
                match self.privacy_manager.get_current_mode() {
                    PrivacyMode::Normal => "🟢 Normal",
                    PrivacyMode::Incognito => "🟡 Incognito",
                    PrivacyMode::Strict => "🔴 Strict",
                },
                self.privacy_manager.get_privacy_score()
            ),
            format!(
                "• 🔐 Password Generator: {} ({} generiert)",
                "✅ Bereit",
                self.password_generator.get_generation_history().len()
            ),
            format!(
                "• 🔒 HTTPS Enforcer: {} ({} upgrades)",
                if self.https_enforcer.is_enabled() { "✅" } else { "❌" },
                self.https_enforcer.get_upgrade_count()
            ),
            format!("• 🚨 Security Alerts: {} aktiv", self.security_alerts.len()),
            "".to_string(),
            "💡 VERFÜGBARE KOMMANDOS:".to_string(),
            "• 'adblocker' → Ad-Blocker Status".to_string(),
            "• 'privacy' → Privacy Manager".to_string(),
            "• 'password' → Password Generator".to_string(),
            "• 'https' → HTTPS Enforcer".to_string(),
            "• 'security overview' → Vollständige Übersicht".to_string(),
            "• 'security report' → Detaillierter Sicherheitsbericht".to_string(),
            "".to_string(),
            "⚡ Kommando eingeben und Enter drücken!".to_string(),
        ]
    }

    pub fn get_detailed_security_report(&self) -> Vec<String> {
        let mut report = vec![
            "🛡️ DETAILLIERTER SICHERHEITSBERICHT".to_string(),
            "===================================".to_string(),
            "".to_string(),
            format!("📊 GESAMT SECURITY SCORE: {}/100", self.get_overall_security_score()),
            format!("🏆 SECURITY LEVEL: {}", self.get_security_level()),
            "".to_string(),
            "🎯 KOMPONENTEN-BEWERTUNG:".to_string(),
        ];

        // Add component scores
        let ad_score = if self.ad_blocker.is_enabled() { 20 } else { 0 };
        let privacy_score = (self.privacy_manager.get_privacy_score() as f32 * 0.3) as u8;
        let https_score = (self.https_enforcer.get_security_score() as f32 * 0.5) as u8;

        report.extend(vec![
            format!("   • 🚫 Ad-Blocker: {} Punkte", ad_score),
            format!("   • 🔒 Privacy Manager: {} Punkte", privacy_score),
            format!("   • 🔒 HTTPS Enforcer: {} Punkte", https_score),
            "".to_string(),
            "📈 AKTIVITÄTS-STATISTIKEN:".to_string(),
            format!("   • Blockierte Werbung: {}", self.ad_blocker.get_blocked_count()),
            format!("   • HTTPS Upgrades: {}", self.https_enforcer.get_upgrade_count()),
            format!("   • Generierte Passwörter: {}", self.password_generator.get_generation_history().len()),
            format!("   • Privacy Score: {}/100", self.privacy_manager.get_privacy_score()),
            "".to_string(),
            "🛡️ SICHERHEITSEMPFEHLUNGEN:".to_string(),
        ]);

        // Add recommendations
        report.extend(self.get_security_recommendations());

        report
    }

    fn get_security_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if !self.ad_blocker.is_enabled() {
            recommendations.push("• Ad-Blocker aktivieren für besseren Schutz".to_string());
        }
        
        if self.privacy_manager.get_privacy_score() < 50 {
            recommendations.push("• Privacy-Einstellungen verschärfen".to_string());
        }
        
        if !self.https_enforcer.is_enabled() {
            recommendations.push("• HTTPS Enforcer aktivieren".to_string());
        }
        
        if !self.https_enforcer.is_strict_mode() {
            recommendations.push("• HTTPS Strict Mode für maximale Sicherheit".to_string());
        }
        
        if self.password_generator.get_generation_history().is_empty() {
            recommendations.push("• Sichere Passwörter generieren lassen".to_string());
        }
        
        if recommendations.is_empty() {
            recommendations.push("🎉 Alle Sicherheitseinstellungen sind optimal konfiguriert!".to_string());
        }
        
        recommendations
    }

    // 📄 INDIVIDUAL COMPONENT DISPLAYS
    pub fn get_ad_blocker_display(&self) -> Vec<String> {
        self.ad_blocker.get_stats()
    }

    pub fn get_privacy_display(&self) -> Vec<String> {
        self.privacy_manager.get_privacy_display()
    }

    pub fn get_password_display(&self) -> Vec<String> {
        self.password_generator.get_generator_display()
    }

    pub fn get_https_display(&self) -> Vec<String> {
        self.https_enforcer.get_https_display()
    }

    // 💻 COMMAND HANDLING
    pub async fn handle_command(&mut self, command: &str) -> Result<Vec<String>> {
        match command.trim().to_lowercase().as_str() {
            // Main views
            "adblocker" => Ok(self.get_ad_blocker_display()),
            "privacy" => Ok(self.get_privacy_display()),
            "password" => Ok(self.get_password_display()),
            "https" => Ok(self.get_https_display()),
            "security" | "overview" => Ok(self.get_security_overview()),
            "security report" | "report" => Ok(self.get_detailed_security_report()),

            // Ad-blocker commands
            "adblocker toggle" => {
                let enabled = self.toggle_ad_blocker();
                Ok(vec![format!(
                    "🛡️ Ad-Blocker: {}",
                    if enabled { "Enabled" } else { "Disabled" }
                )])
            }
            "adblocker reset" => {
                self.reset_ad_blocker_stats();
                Ok(vec!["📊 Ad-Blocker Statistiken zurückgesetzt".to_string()])
            }

            // Privacy commands
            "privacy toggle" => {
                let incognito = self.toggle_incognito();
                Ok(vec![format!(
                    "🔒 Incognito Mode: {}",
                    if incognito { "Enabled" } else { "Disabled" }
                )])
            }
            "privacy strict" => {
                self.set_privacy_mode(PrivacyMode::Strict)?;
                Ok(vec!["🔒 Strict privacy mode activated".to_string()])
            }
            "privacy normal" => {
                self.set_privacy_mode(PrivacyMode::Normal)?;
                Ok(vec!["🔒 Normal privacy mode activated".to_string()])
            }

            // Password commands
            "password generate" => {
                let password = self.generate_password(12, true);
                Ok(vec![
                    format!("🔐 Password generated: {} characters", password.length),
                    format!("🔐 Strength: {} {}", password.strength.emoji(), password.strength.description()),
                    "🔐 Password stored securely (not displayed for security)".to_string(),
                ])
            }
            "password strong" => {
                let password = self.generate_password(16, true);
                Ok(vec![format!(
                    "🔐 Strong password generated: {} characters, {:?}",
                    password.length, password.strength
                )])
            }
            "password simple" => {
                let password = self.generate_password(8, false);
                Ok(vec![format!(
                    "🔐 Simple password generated: {} characters, {:?}",
                    password.length, password.strength
                )])
            }
            "password phrase" => {
                let passphrase = self.generate_passphrase(4);
                Ok(vec![format!(
                    "🔐 Passphrase generated: {} characters, {:?}",
                    passphrase.length, passphrase.strength
                )])
            }
            "password pin" => {
                let pin = self.generate_pin(4);
                Ok(vec![format!(
                    "🔐 PIN generated: {} digits",
                    pin.length
                )])
            }

            // HTTPS commands
            "https toggle" => {
                let enabled = self.toggle_https_enforcer();
                Ok(vec![format!(
                    "🔒 HTTPS Enforcer: {}",
                    if enabled { "Enabled" } else { "Disabled" }
                )])
            }
            "https strict" => {
                let strict = self.toggle_https_strict();
                Ok(vec![format!(
                    "🔒 HTTPS Strict Mode: {}",
                    if strict { "Enabled" } else { "Disabled" }
                )])
            }

            // Security presets
            "security minimal" => {
                self.apply_privacy_preset("minimal")?;
                self.apply_https_preset("minimal")?;
                Ok(vec!["🔵 Minimal security preset applied".to_string()])
            }
            "security balanced" => {
                self.apply_privacy_preset("balanced")?;
                self.apply_https_preset("balanced")?;
                Ok(vec!["🟡 Balanced security preset applied".to_string()])
            }
            "security strict" => {
                self.apply_privacy_preset("strict")?;
                self.apply_https_preset("strict")?;
                Ok(vec!["🔴 Strict security preset applied".to_string()])
            }
            "security paranoid" => {
                self.apply_privacy_preset("paranoid")?;
                self.apply_https_preset("paranoid")?;
                Ok(vec!["🖤 Paranoid security preset applied".to_string()])
            }

            _ => Ok(vec![
                "❓ Unknown security command. Available commands:".to_string(),
                "• adblocker, privacy, password, https".to_string(),
                "• security overview, security report".to_string(),
                "• security [minimal|balanced|strict|paranoid]".to_string(),
            ]),
        }
    }

    // 🧹 CLEANUP
    pub async fn cleanup(&self) -> Result<()> {
        println!("🧹 Security Features cleanup...");
        // Individual components handle their own cleanup
        Ok(())
    }

    // 📊 STATUS CHECKS
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    pub fn get_window_handle(&self) -> HWND {
        self.window_handle
    }

    // 🔄 CONFIGURATION MANAGEMENT
    pub fn export_all_configurations(&self) -> Result<String> {
        let config = serde_json::json!({
            "ad_blocker": {
                "enabled": self.ad_blocker.is_enabled(),
                "blocked_count": self.ad_blocker.get_blocked_count(),
                "rule_count": self.ad_blocker.get_rule_count()
            },
            "privacy_manager": {
                "mode": self.privacy_manager.get_current_mode(),
                "incognito_active": self.privacy_manager.is_incognito_active(),
                "privacy_score": self.privacy_manager.get_privacy_score()
            },
            "https_enforcer": {
                "enabled": self.https_enforcer.is_enabled(),
                "strict_mode": self.https_enforcer.is_strict_mode(),
                "upgrade_count": self.https_enforcer.get_upgrade_count(),
                "security_score": self.https_enforcer.get_security_score()
            },
            "overall": {
                "security_score": self.get_overall_security_score(),
                "security_level": self.get_security_level()
            }
        });
        
        let json_data = serde_json::to_string_pretty(&config)?;
        println!("📤 Exported complete security configuration");
        Ok(json_data)
    }
}

// 🧪 TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use windows::Win32::Foundation::HWND;

    #[test]
    fn test_security_manager_creation() {
        let manager = SecurityFeaturesManager::new(HWND(0)).unwrap();
        assert!(manager.is_initialized());
    }

    #[test]
    fn test_url_processing() {
        let mut manager = SecurityFeaturesManager::new(HWND(0)).unwrap();
        
        // Test HTTPS URL (should pass through)
        let result = manager.process_url("https://example.com").unwrap();
        assert_eq!(result, "https://example.com");
        
        // Test HTTP URL (should be upgraded)
        let result = manager.process_url("http://example.com").unwrap();
        assert_eq!(result, "https://example.com");
    }

    #[test]
    fn test_security_score() {
        let manager = SecurityFeaturesManager::new(HWND(0)).unwrap();
        let score = manager.get_overall_security_score();
        assert!(score > 0);
        assert!(score <= 100);
    }

    #[test]
    fn test_password_generation() {
        let mut manager = SecurityFeaturesManager::new(HWND(0)).unwrap();
        let password = manager.generate_password(12, true);
        assert_eq!(password.length, 12);
    }

    #[tokio::test]
    async fn test_command_handling() {
        let mut manager = SecurityFeaturesManager::new(HWND(0)).unwrap();
        
        let result = manager.handle_command("security").await.unwrap();
        assert!(!result.is_empty());
        
        let result = manager.handle_command("adblocker toggle").await.unwrap();
        assert!(!result.is_empty());
    }
}