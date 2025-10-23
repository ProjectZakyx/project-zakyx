// 🔒 Privacy Manager - Privacy settings and tracking protection

use anyhow::Result;
use crate::security::types::PrivacyMode;

pub struct PrivacyManager {
    current_mode: PrivacyMode,
    incognito_active: bool,
    cookies_enabled: bool,
    tracking_protection: bool,
    do_not_track: bool,
    javascript_enabled: bool,
    location_sharing: bool,
    camera_access: bool,
    microphone_access: bool,
    notification_permission: bool,
}

impl PrivacyManager {
    pub fn new() -> Self {
        println!("🔒 Initializing Privacy Manager...");
        
        Self {
            current_mode: PrivacyMode::Normal,
            incognito_active: false,
            cookies_enabled: true,
            tracking_protection: true,
            do_not_track: true,
            javascript_enabled: true,
            location_sharing: false,
            camera_access: false,
            microphone_access: false,
            notification_permission: false,
        }
    }

    pub fn set_privacy_mode(&mut self, mode: PrivacyMode) -> Result<()> {
        println!("🔒 Setting privacy mode to: {:?}", mode);
        
        match mode {
            PrivacyMode::Normal => {
                self.cookies_enabled = true;
                self.tracking_protection = true;
                self.do_not_track = true;
                self.javascript_enabled = true;
            }
            PrivacyMode::Incognito => {
                self.cookies_enabled = false;
                self.tracking_protection = true;
                self.do_not_track = true;
                self.javascript_enabled = true;
                self.incognito_active = true;
            }
            PrivacyMode::Strict => {
                self.cookies_enabled = false;
                self.tracking_protection = true;
                self.do_not_track = true;
                self.javascript_enabled = false;
                self.location_sharing = false;
                self.camera_access = false;
                self.microphone_access = false;
                self.notification_permission = false;
            }
        }
        
        self.current_mode = mode;
        println!("✅ Privacy mode updated successfully");
        Ok(())
    }

    pub fn toggle_incognito(&mut self) -> bool {
        self.incognito_active = !self.incognito_active;
        
        if self.incognito_active {
            println!("🕵️ Incognito mode activated");
            // Automatically disable cookies in incognito mode
            self.cookies_enabled = false;
            self.current_mode = PrivacyMode::Incognito;
        } else {
            println!("🕵️ Incognito mode deactivated");
            // Return to normal mode
            self.cookies_enabled = true;
            self.current_mode = PrivacyMode::Normal;
        }
        
        self.incognito_active
    }

    pub fn toggle_cookies(&mut self) -> bool {
        self.cookies_enabled = !self.cookies_enabled;
        println!(
            "🍪 Cookies: {}",
            if self.cookies_enabled { "Enabled" } else { "Disabled" }
        );
        self.cookies_enabled
    }

    pub fn toggle_tracking_protection(&mut self) -> bool {
        self.tracking_protection = !self.tracking_protection;
        println!(
            "🛡️ Tracking Protection: {}",
            if self.tracking_protection { "Enabled" } else { "Disabled" }
        );
        self.tracking_protection
    }

    pub fn toggle_do_not_track(&mut self) -> bool {
        self.do_not_track = !self.do_not_track;
        println!(
            "🚫 Do Not Track: {}",
            if self.do_not_track { "Enabled" } else { "Disabled" }
        );
        self.do_not_track
    }

    pub fn toggle_javascript(&mut self) -> bool {
        self.javascript_enabled = !self.javascript_enabled;
        println!(
            "⚡ JavaScript: {}",
            if self.javascript_enabled { "Enabled" } else { "Disabled" }
        );
        self.javascript_enabled
    }

    pub fn toggle_location_sharing(&mut self) -> bool {
        self.location_sharing = !self.location_sharing;
        println!(
            "📍 Location Sharing: {}",
            if self.location_sharing { "Enabled" } else { "Disabled" }
        );
        self.location_sharing
    }

    pub fn toggle_camera_access(&mut self) -> bool {
        self.camera_access = !self.camera_access;
        println!(
            "📷 Camera Access: {}",
            if self.camera_access { "Enabled" } else { "Disabled" }
        );
        self.camera_access
    }

    pub fn toggle_microphone_access(&mut self) -> bool {
        self.microphone_access = !self.microphone_access;
        println!(
            "🎤 Microphone Access: {}",
            if self.microphone_access { "Enabled" } else { "Disabled" }
        );
        self.microphone_access
    }

    pub fn should_block_tracking(&self, url: &str) -> bool {
        if !self.tracking_protection {
            return false;
        }

        // List of known tracking domains and patterns
        let tracking_patterns = vec![
            "facebook.com/tr",
            "google-analytics.com",
            "googletagmanager.com",
            "doubleclick.net",
            "googlesyndication.com",
            "amazon-adsystem.com",
            "scorecardresearch.com",
            "quantserve.com",
            "outbrain.com",
            "taboola.com",
            "addthis.com",
            "sharethis.com",
            "hotjar.com",
            "fullstory.com",
            "mixpanel.com",
            "segment.com",
            "amplitude.com",
            "intercom.io",
            "zendesk.com/embeddable",
            "google.com/recaptcha",
        ];

        for pattern in tracking_patterns {
            if url.contains(pattern) {
                println!("🛡️ Blocked tracking attempt: {}", url);
                return true;
            }
        }

        false
    }

    pub fn should_allow_cookies(&self, _url: &str) -> bool {
        // In incognito mode, cookies are always disabled
        if self.incognito_active {
            return false;
        }
        
        self.cookies_enabled
    }

    pub fn should_allow_javascript(&self, _url: &str) -> bool {
        self.javascript_enabled
    }

    pub fn should_allow_location(&self, _url: &str) -> bool {
        self.location_sharing
    }

    pub fn get_current_mode(&self) -> &PrivacyMode {
        &self.current_mode
    }

    pub fn is_incognito_active(&self) -> bool {
        self.incognito_active
    }

    pub fn get_privacy_score(&self) -> u8 {
        let mut score = 0;
        
        if self.tracking_protection { score += 20; }
        if self.do_not_track { score += 15; }
        if !self.cookies_enabled { score += 15; }
        if !self.javascript_enabled { score += 10; }
        if !self.location_sharing { score += 15; }
        if !self.camera_access { score += 10; }
        if !self.microphone_access { score += 10; }
        if !self.notification_permission { score += 5; }
        
        score
    }

    pub fn get_privacy_level(&self) -> String {
        let score = self.get_privacy_score();
        
        match score {
            0..=30 => "🔴 Niedrig".to_string(),
            31..=60 => "🟡 Mittel".to_string(),
            61..=80 => "🟢 Hoch".to_string(),
            81..=100 => "💪 Maximal".to_string(),
            _ => "❓ Unbekannt".to_string(),
        }
    }

    pub fn get_privacy_display(&self) -> Vec<String> {
        vec![
            "🔒 PRIVACY MANAGER".to_string(),
            "=================".to_string(),
            "".to_string(),
            format!("Aktueller Modus: {:?}", self.current_mode),
            format!("Beschreibung: {}", self.current_mode.description()),
            format!("Privacy Score: {}/100", self.get_privacy_score()),
            format!("Privacy Level: {}", self.get_privacy_level()),
            "".to_string(),
            "🛡️ PRIVACY FEATURES:".to_string(),
            format!(
                "• Incognito Mode: {}",
                if self.incognito_active { "✅ Aktiv" } else { "❌ Inaktiv" }
            ),
            format!(
                "• Cookies: {}",
                if self.cookies_enabled { "✅ Aktiviert" } else { "❌ Deaktiviert" }
            ),
            format!(
                "• Tracking Protection: {}",
                if self.tracking_protection { "✅ Aktiviert" } else { "❌ Deaktiviert" }
            ),
            format!(
                "• Do Not Track: {}",
                if self.do_not_track { "✅ Aktiviert" } else { "❌ Deaktiviert" }
            ),
            format!(
                "• JavaScript: {}",
                if self.javascript_enabled { "✅ Aktiviert" } else { "❌ Deaktiviert" }
            ),
            "".to_string(),
            "🔐 PERMISSION SETTINGS:".to_string(),
            format!(
                "• Location Sharing: {}",
                if self.location_sharing { "✅ Erlaubt" } else { "❌ Blockiert" }
            ),
            format!(
                "• Camera Access: {}",
                if self.camera_access { "✅ Erlaubt" } else { "❌ Blockiert" }
            ),
            format!(
                "• Microphone Access: {}",
                if self.microphone_access { "✅ Erlaubt" } else { "❌ Blockiert" }
            ),
            format!(
                "• Notifications: {}",
                if self.notification_permission { "✅ Erlaubt" } else { "❌ Blockiert" }
            ),
            "".to_string(),
            "💡 KOMMANDOS:".to_string(),
            "• 'privacy toggle' → Incognito Ein/Aus".to_string(),
            "• 'privacy strict' → Strict Mode".to_string(),
            "• 'privacy normal' → Normal Mode".to_string(),
            "• 'privacy cookies' → Cookies Ein/Aus".to_string(),
            "• 'privacy tracking' → Tracking Protection".to_string(),
        ]
    }

    pub fn get_detailed_privacy_report(&self) -> Vec<String> {
        vec![
            "🔒 DETAILLIERTER PRIVACY-REPORT".to_string(),
            "==============================".to_string(),
            "".to_string(),
            format!("📊 PRIVACY SCORE: {}/100", self.get_privacy_score()),
            format!("🏆 PRIVACY LEVEL: {}", self.get_privacy_level()),
            "".to_string(),
            "🎯 KATEGORIEN:".to_string(),
            format!("   • Tracking Protection: {}", if self.tracking_protection { "✅ +20" } else { "❌ 0" }),
            format!("   • Do Not Track: {}", if self.do_not_track { "✅ +15" } else { "❌ 0" }),
            format!("   • Cookies Disabled: {}", if !self.cookies_enabled { "✅ +15" } else { "❌ 0" }),
            format!("   • JavaScript Disabled: {}", if !self.javascript_enabled { "✅ +10" } else { "❌ 0" }),
            format!("   • Location Blocked: {}", if !self.location_sharing { "✅ +15" } else { "❌ 0" }),
            format!("   • Camera Blocked: {}", if !self.camera_access { "✅ +10" } else { "❌ 0" }),
            format!("   • Microphone Blocked: {}", if !self.microphone_access { "✅ +10" } else { "❌ 0" }),
            format!("   • Notifications Blocked: {}", if !self.notification_permission { "✅ +5" } else { "❌ 0" }),
            "".to_string(),
            "🛡️ EMPFEHLUNGEN:".to_string(),
        ]
        .into_iter()
        .chain(self.get_privacy_recommendations())
        .collect()
    }

    fn get_privacy_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if self.cookies_enabled {
            recommendations.push("• Cookies deaktivieren für mehr Privacy".to_string());
        }
        
        if !self.tracking_protection {
            recommendations.push("• Tracking Protection aktivieren".to_string());
        }
        
        if !self.do_not_track {
            recommendations.push("• Do Not Track Header senden".to_string());
        }
        
        if self.location_sharing {
            recommendations.push("• Location Sharing deaktivieren".to_string());
        }
        
        if self.camera_access {
            recommendations.push("• Camera Access einschränken".to_string());
        }
        
        if self.microphone_access {
            recommendations.push("• Microphone Access einschränken".to_string());
        }
        
        if recommendations.is_empty() {
            recommendations.push("🎉 Alle Privacy-Einstellungen optimal!".to_string());
        }
        
        recommendations
    }

    pub fn reset_to_defaults(&mut self) {
        println!("🔄 Resetting privacy settings to defaults...");
        *self = Self::new();
    }

    pub fn apply_privacy_preset(&mut self, preset: &str) -> Result<()> {
        match preset.to_lowercase().as_str() {
            "minimal" => {
                self.set_privacy_mode(PrivacyMode::Normal)?;
                self.tracking_protection = false;
                self.do_not_track = false;
                println!("🔵 Applied minimal privacy preset");
            }
            "balanced" => {
                self.set_privacy_mode(PrivacyMode::Normal)?;
                self.tracking_protection = true;
                self.do_not_track = true;
                println!("🟡 Applied balanced privacy preset");
            }
            "strict" => {
                self.set_privacy_mode(PrivacyMode::Strict)?;
                println!("🔴 Applied strict privacy preset");
            }
            "paranoid" => {
                self.set_privacy_mode(PrivacyMode::Strict)?;
                self.javascript_enabled = false;
                self.cookies_enabled = false;
                self.location_sharing = false;
                self.camera_access = false;
                self.microphone_access = false;
                self.notification_permission = false;
                println!("🖤 Applied paranoid privacy preset");
            }
            _ => {
                return Err(anyhow::anyhow!("Unknown privacy preset: {}", preset));
            }
        }
        
        Ok(())
    }
}

impl Default for PrivacyManager {
    fn default() -> Self {
        Self::new()
    }
}

// 🧪 TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_manager_creation() {
        let manager = PrivacyManager::new();
        assert!(matches!(manager.current_mode, PrivacyMode::Normal));
        assert!(!manager.incognito_active);
        assert!(manager.cookies_enabled);
        assert!(manager.tracking_protection);
    }

    #[test]
    fn test_incognito_toggle() {
        let mut manager = PrivacyManager::new();
        assert!(!manager.is_incognito_active());
        
        manager.toggle_incognito();
        assert!(manager.is_incognito_active());
        assert!(!manager.cookies_enabled);
        
        manager.toggle_incognito();
        assert!(!manager.is_incognito_active());
        assert!(manager.cookies_enabled);
    }

    #[test]
    fn test_privacy_modes() {
        let mut manager = PrivacyManager::new();
        
        manager.set_privacy_mode(PrivacyMode::Strict).unwrap();
        assert!(!manager.cookies_enabled);
        assert!(!manager.javascript_enabled);
        
        manager.set_privacy_mode(PrivacyMode::Normal).unwrap();
        assert!(manager.cookies_enabled);
        assert!(manager.javascript_enabled);
    }

    #[test]
    fn test_tracking_protection() {
        let manager = PrivacyManager::new();
        assert!(manager.should_block_tracking("https://google-analytics.com/track"));
        assert!(!manager.should_block_tracking("https://example.com"));
    }

    #[test]
    fn test_privacy_score() {
        let manager = PrivacyManager::new();
        let score = manager.get_privacy_score();
        assert!(score > 0);
        assert!(score <= 100);
    }

    #[test]
    fn test_privacy_presets() {
        let mut manager = PrivacyManager::new();
        
        manager.apply_privacy_preset("strict").unwrap();
        assert!(matches!(manager.current_mode, PrivacyMode::Strict));
        
        manager.apply_privacy_preset("minimal").unwrap();
        assert!(!manager.tracking_protection);
        assert!(!manager.do_not_track);
    }
} 
