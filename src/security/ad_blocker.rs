// 🚫 Ad Blocker - Advanced ad blocking and tracking protection

use anyhow::Result;
use chrono::Utc;
use std::collections::HashSet;

use crate::security::types::{AdBlockRule, AdBlockRuleType};

pub struct AdBlocker {
    rules: Vec<AdBlockRule>,
    blocked_domains: HashSet<String>,
    enabled: bool,
    blocked_count: u64,
}

impl AdBlocker {
    pub fn new() -> Self {
        let mut ad_blocker = AdBlocker {
            rules: Vec::new(),
            blocked_domains: HashSet::new(),
            enabled: true,
            blocked_count: 0,
        };

        ad_blocker.load_default_rules();
        ad_blocker
    }

    fn load_default_rules(&mut self) {
        println!("🚫 Loading default ad-block rules...");
        
        // Standard Ad-Block Regeln
        let default_rules = vec![
            ("doubleclick.net", AdBlockRuleType::Block),
            ("googleadservices.com", AdBlockRuleType::Block),
            ("googlesyndication.com", AdBlockRuleType::Block),
            ("amazon-adsystem.com", AdBlockRuleType::Block),
            ("facebook.com/tr", AdBlockRuleType::Block),
            ("google-analytics.com", AdBlockRuleType::Block),
            ("googletagmanager.com", AdBlockRuleType::Block),
            ("ads.yahoo.com", AdBlockRuleType::Block),
            ("adsystem.amazon.de", AdBlockRuleType::Block),
            ("outbrain.com", AdBlockRuleType::Block),
            // Additional tracking domains
            ("scorecardresearch.com", AdBlockRuleType::Block),
            ("quantserve.com", AdBlockRuleType::Block),
            ("addthis.com", AdBlockRuleType::Block),
            ("sharethis.com", AdBlockRuleType::Block),
            ("taboola.com", AdBlockRuleType::Block),
            // Social media tracking
            ("facebook.net", AdBlockRuleType::Block),
            ("twitter.com/i/jot", AdBlockRuleType::Block),
            ("linkedin.com/li/track", AdBlockRuleType::Block),
            // Analytics
            ("hotjar.com", AdBlockRuleType::Block),
            ("fullstory.com", AdBlockRuleType::Block),
        ];

        for (pattern, rule_type) in default_rules {
            self.rules.push(AdBlockRule {
                pattern: pattern.to_string(),
                rule_type: rule_type.clone(),
                enabled: true,
                created_at: Utc::now(),
            });

            self.blocked_domains.insert(pattern.to_string());
        }

        println!("🛡️ Loaded {} ad-block rules", self.rules.len());
    }

    pub fn should_block(&mut self, url: &str) -> bool {
        if !self.enabled {
            return false;
        }

        for rule in &self.rules {
            if rule.enabled && url.contains(&rule.pattern) {
                match rule.rule_type {
                    AdBlockRuleType::Block => {
                        self.blocked_count += 1;
                        println!("🚫 Blocked: {} (rule: {})", url, rule.pattern);
                        return true;
                    }
                    AdBlockRuleType::Allow => {
                        println!("✅ Allowed: {} (rule: {})", url, rule.pattern);
                        return false;
                    }
                    AdBlockRuleType::Hide => {
                        self.blocked_count += 1;
                        println!("🫥 Hidden: {} (rule: {})", url, rule.pattern);
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn add_rule(&mut self, pattern: String, rule_type: AdBlockRuleType) -> Result<()> {
        // Check if rule already exists
        if self.rules.iter().any(|r| r.pattern == pattern) {
            return Err(anyhow::anyhow!("Rule already exists: {}", pattern));
        }

        let rule = AdBlockRule {
            pattern: pattern.clone(),
            rule_type,
            enabled: true,
            created_at: Utc::now(),
        };

        self.rules.push(rule);
        self.blocked_domains.insert(pattern);
        println!("🛡️ Added ad-block rule: {}", pattern);
        Ok(())
    }

    pub fn remove_rule(&mut self, pattern: &str) -> Result<()> {
        let initial_len = self.rules.len();
        self.rules.retain(|r| r.pattern != pattern);
        self.blocked_domains.remove(pattern);
        
        if self.rules.len() < initial_len {
            println!("🗑️ Removed ad-block rule: {}", pattern);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Rule not found: {}", pattern))
        }
    }

    pub fn toggle_rule(&mut self, pattern: &str) -> Result<bool> {
        if let Some(rule) = self.rules.iter_mut().find(|r| r.pattern == pattern) {
            rule.enabled = !rule.enabled;
            println!("🔄 Toggled rule {}: {}", pattern, rule.enabled);
            Ok(rule.enabled)
        } else {
            Err(anyhow::anyhow!("Rule not found: {}", pattern))
        }
    }

    pub fn toggle_enabled(&mut self) -> bool {
        self.enabled = !self.enabled;
        println!(
            "🛡️ Ad-Blocker: {}",
            if self.enabled { "Enabled" } else { "Disabled" }
        );
        self.enabled
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_blocked_count(&self) -> u64 {
        self.blocked_count
    }

    pub fn get_rule_count(&self) -> usize {
        self.rules.len()
    }

    pub fn get_enabled_rule_count(&self) -> usize {
        self.rules.iter().filter(|r| r.enabled).count()
    }

    pub fn reset_stats(&mut self) {
        self.blocked_count = 0;
        println!("📊 Ad-blocker stats reset");
    }

    pub fn get_stats(&self) -> Vec<String> {
        let mut stats = vec![
            "🚫 AD-BLOCKER STATUS".to_string(),
            "===================".to_string(),
            "".to_string(),
            format!(
                "Status: {}",
                if self.enabled {
                    "✅ Enabled"
                } else {
                    "❌ Disabled"
                }
            ),
            format!("Total Rules: {}", self.rules.len()),
            format!("Active Rules: {}", self.get_enabled_rule_count()),
            format!("Blocked Requests: {}", self.blocked_count),
            "".to_string(),
            "🚫 BLOCKED DOMAINS:".to_string(),
        ];

        // Add top blocked domains
        let mut domains: Vec<&String> = self.blocked_domains.iter().collect();
        domains.sort();
        
        for domain in domains.iter().take(10) {
            stats.push(format!("   • {}", domain));
        }
        
        if self.blocked_domains.len() > 10 {
            stats.push(format!(
                "   ... und {} weitere",
                self.blocked_domains.len() - 10
            ));
        }

        stats.push("".to_string());
        stats.push("💡 KOMMANDOS:".to_string());
        stats.push("• 'adblocker toggle' → Ein/Aus".to_string());
        stats.push("• 'adblocker stats' → Statistiken".to_string());
        stats.push("• 'adblocker reset' → Stats zurücksetzen".to_string());

        stats
    }

    pub fn get_detailed_stats(&self) -> Vec<String> {
        let mut stats = vec![
            "🚫 DETAILLIERTE AD-BLOCKER STATISTIKEN".to_string(),
            "====================================".to_string(),
            "".to_string(),
            format!("📊 Gesamtstatistiken:"),
            format!("   • Status: {}", if self.enabled { "✅ Aktiv" } else { "❌ Deaktiviert" }),
            format!("   • Regeln geladen: {}", self.rules.len()),
            format!("   • Aktive Regeln: {}", self.get_enabled_rule_count()),
            format!("   • Blockierte Anfragen: {}", self.blocked_count),
            "".to_string(),
            "🎯 REGEL-KATEGORIEN:".to_string(),
        ];

        // Categorize rules
        let block_rules = self.rules.iter().filter(|r| matches!(r.rule_type, AdBlockRuleType::Block)).count();
        let allow_rules = self.rules.iter().filter(|r| matches!(r.rule_type, AdBlockRuleType::Allow)).count();
        let hide_rules = self.rules.iter().filter(|r| matches!(r.rule_type, AdBlockRuleType::Hide)).count();

        stats.push(format!("   • 🚫 Block-Regeln: {}", block_rules));
        stats.push(format!("   • ✅ Allow-Regeln: {}", allow_rules));
        stats.push(format!("   • 🫥 Hide-Regeln: {}", hide_rules));

        stats.push("".to_string());
        stats.push("🔧 REGEL-MANAGEMENT:".to_string());
        stats.push("• Neue Regel hinzufügen".to_string());
        stats.push("• Bestehende Regel bearbeiten".to_string());
        stats.push("• Regeln im-/exportieren".to_string());

        stats
    }

    pub fn export_rules(&self) -> Result<String> {
        let json_data = serde_json::to_string_pretty(&self.rules)?;
        println!("📤 Exported {} ad-block rules", self.rules.len());
        Ok(json_data)
    }

    pub fn import_rules(&mut self, json_data: &str) -> Result<usize> {
        let imported_rules: Vec<AdBlockRule> = serde_json::from_str(json_data)?;
        let mut imported_count = 0;

        for rule in imported_rules {
            if !self.rules.iter().any(|r| r.pattern == rule.pattern) {
                self.blocked_domains.insert(rule.pattern.clone());
                self.rules.push(rule);
                imported_count += 1;
            }
        }

        println!("📥 Imported {} new ad-block rules", imported_count);
        Ok(imported_count)
    }
}

impl Default for AdBlocker {
    fn default() -> Self {
        Self::new()
    }
}

// 🧪 TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ad_blocker_creation() {
        let blocker = AdBlocker::new();
        assert!(blocker.is_enabled());
        assert!(blocker.get_rule_count() > 0);
    }

    #[test]
    fn test_should_block_ads() {
        let mut blocker = AdBlocker::new();
        assert!(blocker.should_block("https://doubleclick.net/ads"));
        assert!(!blocker.should_block("https://example.com"));
    }

    #[test]
    fn test_add_remove_rule() {
        let mut blocker = AdBlocker::new();
        let initial_count = blocker.get_rule_count();
        
        blocker.add_rule("test.com".to_string(), AdBlockRuleType::Block).unwrap();
        assert_eq!(blocker.get_rule_count(), initial_count + 1);
        
        blocker.remove_rule("test.com").unwrap();
        assert_eq!(blocker.get_rule_count(), initial_count);
    }

    #[test]
    fn test_toggle_enabled() {
        let mut blocker = AdBlocker::new();
        assert!(blocker.is_enabled());
        
        blocker.toggle_enabled();
        assert!(!blocker.is_enabled());
        
        blocker.toggle_enabled();
        assert!(blocker.is_enabled());
    }
} 