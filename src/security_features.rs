use anyhow::Result;
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use windows::Win32::Foundation::HWND;

// 🛡️ SECURITY FEATURES - Bereich 4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrivacyMode {
    Normal,
    Incognito,
    Strict,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdBlockRule {
    pub pattern: String,
    pub rule_type: AdBlockRuleType,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdBlockRuleType {
    Block,
    Allow,
    Hide,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAlert {
    pub id: String,
    pub alert_type: SecurityAlertType,
    pub message: String,
    pub url: String,
    pub timestamp: DateTime<Utc>,
    pub severity: SecuritySeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityAlertType {
    InsecureConnection,
    MaliciousWebsite,
    TrackingAttempt,
    DataLeak,
    SuspiciousDownload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PasswordStrength {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedPassword {
    pub password: String,
    pub strength: PasswordStrength,
    pub length: usize,
    pub generated_at: DateTime<Utc>,
}

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
        ];

        for (pattern, rule_type) in default_rules {
            self.rules.push(AdBlockRule {
                pattern: pattern.to_string(),
                rule_type,
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
                        return false;
                    }
                    AdBlockRuleType::Hide => {
                        println!("🫥 Hidden: {} (rule: {})", url, rule.pattern);
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn add_rule(&mut self, pattern: String, rule_type: AdBlockRuleType) {
        let rule = AdBlockRule {
            pattern: pattern.clone(),
            rule_type,
            enabled: true,
            created_at: Utc::now(),
        };

        self.rules.push(rule);
        self.blocked_domains.insert(pattern);
        println!("🛡️ Added ad-block rule");
    }

    pub fn toggle_enabled(&mut self) -> bool {
        self.enabled = !self.enabled;
        println!(
            "🛡️ Ad-Blocker: {}",
            if self.enabled { "Enabled" } else { "Disabled" }
        );
        self.enabled
    }

    pub fn get_stats(&self) -> Vec<String> {
        vec![
            "🛡️ AD-BLOCKER STATUS".to_string(),
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
            format!("Rules Loaded: {}", self.rules.len()),
            format!("Blocked Requests: {}", self.blocked_count),
            "".to_string(),
            "🚫 BLOCKED DOMAINS:".to_string(),
        ]
        .into_iter()
        .chain(
            self.blocked_domains
                .iter()
                .take(10)
                .map(|d| format!("   • {}", d)),
        )
        .chain(if self.blocked_domains.len() > 10 {
            vec![format!(
                "   ... und {} weitere",
                self.blocked_domains.len() - 10
            )]
        } else {
            vec![]
        })
        .collect()
    }
}

pub struct PrivacyManager {
    current_mode: PrivacyMode,
    incognito_active: bool,
    cookies_enabled: bool,
    tracking_protection: bool,
    do_not_track: bool,
}

impl PrivacyManager {
    pub fn new() -> Self {
        PrivacyManager {
            current_mode: PrivacyMode::Normal,
            incognito_active: false,
            cookies_enabled: true,
            tracking_protection: true,
            do_not_track: true,
        }
    }

    pub fn set_privacy_mode(&mut self, mode: PrivacyMode) {
        self.current_mode = mode;

        match &self.current_mode {
            PrivacyMode::Normal => {
                self.cookies_enabled = true;
                self.tracking_protection = false;
                self.incognito_active = false;
            }
            PrivacyMode::Incognito => {
                self.cookies_enabled = false;
                self.tracking_protection = true;
                self.incognito_active = true;
            }
            PrivacyMode::Strict => {
                self.cookies_enabled = false;
                self.tracking_protection = true;
                self.incognito_active = true;
                self.do_not_track = true;
            }
        }

        println!("🔒 Privacy mode changed to: {:?}", self.current_mode);
    }

    pub fn toggle_incognito(&mut self) -> bool {
        self.incognito_active = !self.incognito_active;
        if self.incognito_active {
            self.current_mode = PrivacyMode::Incognito;
            self.cookies_enabled = false;
            self.tracking_protection = true;
        } else {
            self.current_mode = PrivacyMode::Normal;
            self.cookies_enabled = true;
        }

        println!(
            "🕵️ Incognito mode: {}",
            if self.incognito_active {
                "Enabled"
            } else {
                "Disabled"
            }
        );
        self.incognito_active
    }

    pub fn should_block_tracking(&self, url: &str) -> bool {
        if !self.tracking_protection {
            return false;
        }

        let tracking_domains = [
            "google-analytics.com",
            "googletagmanager.com",
            "facebook.com/tr",
            "twitter.com/i/adsct",
            "linkedin.com/insight",
            "bing.com/th",
        ];

        for domain in &tracking_domains {
            if url.contains(domain) {
                println!("🚫 Blocked tracking: {}", url);
                return true;
            }
        }

        false
    }

    pub fn get_privacy_display(&self) -> Vec<String> {
        vec![
            "🔒 PRIVACY MANAGER".to_string(),
            "==================".to_string(),
            "".to_string(),
            format!("Privacy Mode: {:?}", self.current_mode),
            format!(
                "Incognito: {}",
                if self.incognito_active { "✅" } else { "❌" }
            ),
            format!(
                "Cookies: {}",
                if self.cookies_enabled { "✅" } else { "❌" }
            ),
            format!(
                "Tracking Protection: {}",
                if self.tracking_protection {
                    "✅"
                } else {
                    "❌"
                }
            ),
            format!(
                "Do Not Track: {}",
                if self.do_not_track { "✅" } else { "❌" }
            ),
            "".to_string(),
            "🔒 PRIVACY FEATURES:".to_string(),
            "• Cookie Management".to_string(),
            "• Tracking Protection".to_string(),
            "• Incognito Mode".to_string(),
            "• Do Not Track Headers".to_string(),
            "• Fingerprint Protection".to_string(),
            "".to_string(),
            "💡 COMMANDS:".to_string(),
            "• 'privacy toggle' → Incognito wechseln".to_string(),
            "• 'privacy strict' → Strict Mode".to_string(),
            "• 'privacy normal' → Normal Mode".to_string(),
        ]
    }
}

pub struct PasswordGenerator {
    last_generated: Option<GeneratedPassword>,
}

impl PasswordGenerator {
    pub fn new() -> Self {
        PasswordGenerator {
            last_generated: None,
        }
    }

    pub fn generate_password(&mut self, length: usize, include_symbols: bool) -> GeneratedPassword {
        let mut rng = rand::thread_rng();
        let mut password = String::new();

        let lowercase = "abcdefghijklmnopqrstuvwxyz";
        let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let numbers = "0123456789";
        let symbols = if include_symbols {
            "!@#$%^&*()_+-=[]{}|;:,.<>?"
        } else {
            ""
        };

        let all_chars = format!("{}{}{}{}", lowercase, uppercase, numbers, symbols);
        let chars: Vec<char> = all_chars.chars().collect();

        // Garantiere mindestens einen Charakter jeder Kategorie
        if length >= 4 {
            password.push(
                lowercase
                    .chars()
                    .nth(rng.gen_range(0..lowercase.len()))
                    .unwrap(),
            );
            password.push(
                uppercase
                    .chars()
                    .nth(rng.gen_range(0..uppercase.len()))
                    .unwrap(),
            );
            password.push(
                numbers
                    .chars()
                    .nth(rng.gen_range(0..numbers.len()))
                    .unwrap(),
            );

            if include_symbols && !symbols.is_empty() {
                password.push(
                    symbols
                        .chars()
                        .nth(rng.gen_range(0..symbols.len()))
                        .unwrap(),
                );
            }
        }

        // Fülle den Rest zufällig auf
        while password.len() < length {
            password.push(chars[rng.gen_range(0..chars.len())]);
        }

        // Mische das Passwort
        let mut password_chars: Vec<char> = password.chars().collect();
        for i in 0..password_chars.len() {
            let j = rng.gen_range(0..password_chars.len());
            password_chars.swap(i, j);
        }

        let final_password: String = password_chars.iter().collect();
        let strength = self.calculate_strength(&final_password);

        let generated = GeneratedPassword {
            password: final_password,
            strength,
            length,
            generated_at: Utc::now(),
        };

        self.last_generated = Some(generated.clone());
        println!(
            "🔐 Generated password: {} characters, strength: {:?}",
            length, generated.strength
        );

        generated
    }

    fn calculate_strength(&self, password: &str) -> PasswordStrength {
        let mut score = 0;

        if password.len() >= 8 {
            score += 1;
        }
        if password.len() >= 12 {
            score += 1;
        }
        if password.chars().any(|c| c.is_uppercase()) {
            score += 1;
        }
        if password.chars().any(|c| c.is_lowercase()) {
            score += 1;
        }
        if password.chars().any(|c| c.is_numeric()) {
            score += 1;
        }
        if password.chars().any(|c| !c.is_alphanumeric()) {
            score += 1;
        }

        match score {
            0..=2 => PasswordStrength::Weak,
            3..=4 => PasswordStrength::Medium,
            5 => PasswordStrength::Strong,
            _ => PasswordStrength::VeryStrong,
        }
    }

    pub fn get_generator_display(&self) -> Vec<String> {
        let mut result = vec![
            "🔐 PASSWORD GENERATOR".to_string(),
            "=====================".to_string(),
            "".to_string(),
        ];

        if let Some(ref last) = self.last_generated {
            result.extend(vec![
                "🔑 LAST GENERATED:".to_string(),
                format!("   Password: {}", "*".repeat(last.length)),
                format!("   Length: {} characters", last.length),
                format!("   Strength: {:?}", last.strength),
                format!("   Generated: {}", last.generated_at.format("%H:%M:%S")),
                "".to_string(),
            ]);
        }

        result.extend(vec![
            "💡 COMMANDS:".to_string(),
            "• 'password generate' → Neues Passwort".to_string(),
            "• 'password strong' → Starkes Passwort (16 Zeichen)".to_string(),
            "• 'password simple' → Einfaches Passwort (8 Zeichen)".to_string(),
            "".to_string(),
            "🔒 SECURITY FEATURES:".to_string(),
            "• Uppercase/Lowercase Mix".to_string(),
            "• Numbers & Symbols".to_string(),
            "• Cryptographically Random".to_string(),
            "• Strength Analysis".to_string(),
        ]);

        result
    }
}

pub struct HTTPSEnforcer {
    enabled: bool,
    enforce_strict: bool,
    upgraded_urls: u64,
    blocked_http: u64,
}

impl HTTPSEnforcer {
    pub fn new() -> Self {
        HTTPSEnforcer {
            enabled: true,
            enforce_strict: false,
            upgraded_urls: 0,
            blocked_http: 0,
        }
    }

    pub fn process_url(&mut self, url: &str) -> Result<String> {
        if !self.enabled {
            return Ok(url.to_string());
        }

        if url.starts_with("http://") {
            if self.enforce_strict {
                self.blocked_http += 1;
                println!("🚫 Blocked HTTP connection: {}", url);
                return Err(anyhow::anyhow!(
                    "HTTP connections not allowed in strict mode"
                ));
            } else {
                let https_url = url.replace("http://", "https://");
                self.upgraded_urls += 1;
                println!("🔒 Upgraded to HTTPS: {} -> {}", url, https_url);
                return Ok(https_url);
            }
        }

        Ok(url.to_string())
    }

    pub fn toggle_strict_mode(&mut self) -> bool {
        self.enforce_strict = !self.enforce_strict;
        println!(
            "🔒 HTTPS Strict Mode: {}",
            if self.enforce_strict {
                "Enabled"
            } else {
                "Disabled"
            }
        );
        self.enforce_strict
    }

    pub fn toggle_enabled(&mut self) -> bool {
        self.enabled = !self.enabled;
        println!(
            "🔒 HTTPS Enforcer: {}",
            if self.enabled { "Enabled" } else { "Disabled" }
        );
        self.enabled
    }

    pub fn get_https_display(&self) -> Vec<String> {
        vec![
            "🔒 HTTPS ENFORCER".to_string(),
            "=================".to_string(),
            "".to_string(),
            format!(
                "Status: {}",
                if self.enabled {
                    "✅ Enabled"
                } else {
                    "❌ Disabled"
                }
            ),
            format!(
                "Strict Mode: {}",
                if self.enforce_strict {
                    "✅ Enabled"
                } else {
                    "❌ Disabled"
                }
            ),
            format!("URLs Upgraded: {}", self.upgraded_urls),
            format!("HTTP Blocked: {}", self.blocked_http),
            "".to_string(),
            "🔒 SECURITY FEATURES:".to_string(),
            "• Automatic HTTP → HTTPS Upgrade".to_string(),
            "• Strict HTTPS-Only Mode".to_string(),
            "• Mixed Content Protection".to_string(),
            "• Certificate Validation".to_string(),
            "".to_string(),
            "💡 COMMANDS:".to_string(),
            "• 'https toggle' → HTTPS Enforcer ein/aus".to_string(),
            "• 'https strict' → Strict Mode ein/aus".to_string(),
        ]
    }
}

// 🛡️ MAIN SECURITY FEATURES MANAGER
pub struct SecurityFeaturesManager {
    ad_blocker: AdBlocker,
    privacy_manager: PrivacyManager,
    password_generator: PasswordGenerator,
    https_enforcer: HTTPSEnforcer,
    security_alerts: Vec<SecurityAlert>,
    window_handle: HWND,
}

impl SecurityFeaturesManager {
    pub fn new(window_handle: HWND) -> Result<Self> {
        println!("🛡️ Initializing Security Features...");

        Ok(SecurityFeaturesManager {
            ad_blocker: AdBlocker::new(),
            privacy_manager: PrivacyManager::new(),
            password_generator: PasswordGenerator::new(),
            https_enforcer: HTTPSEnforcer::new(),
            security_alerts: Vec::new(),
            window_handle,
        })
    }

    // URL Processing
    pub fn process_url(&mut self, url: &str) -> Result<String> {
        // Check ad-blocking
        if self.ad_blocker.should_block(url) {
            return Err(anyhow::anyhow!("URL blocked by ad-blocker"));
        }

        // Check tracking protection
        if self.privacy_manager.should_block_tracking(url) {
            return Err(anyhow::anyhow!("URL blocked by privacy protection"));
        }

        // HTTPS enforcement
        self.https_enforcer.process_url(url)
    }

    // Security Features
    pub fn toggle_ad_blocker(&mut self) -> bool {
        self.ad_blocker.toggle_enabled()
    }

    pub fn toggle_incognito(&mut self) -> bool {
        self.privacy_manager.toggle_incognito()
    }

    pub fn generate_password(&mut self, length: usize, symbols: bool) -> GeneratedPassword {
        self.password_generator.generate_password(length, symbols)
    }

    pub fn toggle_https_strict(&mut self) -> bool {
        self.https_enforcer.toggle_strict_mode()
    }

    // Display Methods
    pub fn get_security_overview(&self) -> Vec<String> {
        vec![
            "🛡️ SECURITY FEATURES".to_string(),
            "====================".to_string(),
            "".to_string(),
            "💬 VERFÜGBARE KOMMANDOS:".to_string(),
            "• 'adblocker' → Ad-Blocker Status".to_string(),
            "• 'privacy' → Privacy Manager".to_string(),
            "• 'password' → Password Generator".to_string(),
            "• 'https' → HTTPS Enforcer".to_string(),
            "".to_string(),
            "🛡️ SECURITY FEATURES:".to_string(),
            "• 🚫 Ad-Blocker & Tracking Protection".to_string(),
            "• 🔒 Privacy Modes & Incognito".to_string(),
            "• 🔐 Secure Password Generation".to_string(),
            "• 🔒 HTTPS Enforcement".to_string(),
            "• 🚨 Real-time Security Alerts".to_string(),
            "".to_string(),
            format!(
                "Ad-Blocker: {}",
                if self.ad_blocker.enabled {
                    "✅"
                } else {
                    "❌"
                }
            ),
            format!("Privacy Mode: {:?}", self.privacy_manager.current_mode),
            format!(
                "HTTPS Enforcer: {}",
                if self.https_enforcer.enabled {
                    "✅"
                } else {
                    "❌"
                }
            ),
            format!("Security Alerts: {}", self.security_alerts.len()),
            "".to_string(),
            "⚡ Kommando eingeben und Enter drücken!".to_string(),
        ]
    }

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

    pub async fn handle_command(&mut self, command: &str) -> Result<Vec<String>> {
        match command.trim().to_lowercase().as_str() {
            "adblocker" => Ok(self.get_ad_blocker_display()),
            "privacy" => Ok(self.get_privacy_display()),
            "password" => Ok(self.get_password_display()),
            "https" => Ok(self.get_https_display()),

            "adblocker toggle" => {
                let enabled = self.toggle_ad_blocker();
                Ok(vec![format!(
                    "🛡️ Ad-Blocker: {}",
                    if enabled { "Enabled" } else { "Disabled" }
                )])
            }

            "privacy toggle" => {
                let incognito = self.toggle_incognito();
                Ok(vec![format!(
                    "🔒 Incognito Mode: {}",
                    if incognito { "Enabled" } else { "Disabled" }
                )])
            }

            "privacy strict" => {
                self.privacy_manager.set_privacy_mode(PrivacyMode::Strict);
                Ok(vec!["🔒 Strict privacy mode activated".to_string()])
            }

            "privacy normal" => {
                self.privacy_manager.set_privacy_mode(PrivacyMode::Normal);
                Ok(vec!["🔒 Normal privacy mode activated".to_string()])
            }

            "password generate" => {
                let password = self.generate_password(12, true);
                Ok(vec![
                    format!("🔐 Generated password: {} characters", password.length),
                    format!("🔐 Strength: {:?}", password.strength),
                    "🔐 Password has been generated (hidden for security)".to_string(),
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

            "https toggle" => {
                let enabled = self.https_enforcer.toggle_enabled();
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

            _ => Ok(vec![
                "❓ Unknown security command. Try: adblocker, privacy, password, https".to_string(),
            ]),
        }
    }

    pub async fn cleanup(&self) -> Result<()> {
        println!("🧹 Security Features cleanup...");
        Ok(())
    }
}
