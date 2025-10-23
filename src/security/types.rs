// 🛡️ Security Types - Common types and enums for all security modules

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// 🔒 PRIVACY MODES
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrivacyMode {
    Normal,
    Incognito,
    Strict,
}

// 🚫 AD BLOCK RULES
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

// 🚨 SECURITY ALERTS
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

// 🔐 PASSWORD GENERATION
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

// 🎯 UTILITY FUNCTIONS
impl PrivacyMode {
    pub fn description(&self) -> &'static str {
        match self {
            PrivacyMode::Normal => "Standard privacy settings",
            PrivacyMode::Incognito => "Private browsing mode",
            PrivacyMode::Strict => "Maximum privacy protection",
        }
    }
}

impl SecuritySeverity {
    pub fn emoji(&self) -> &'static str {
        match self {
            SecuritySeverity::Low => "🟢",
            SecuritySeverity::Medium => "🟡",
            SecuritySeverity::High => "🟠",
            SecuritySeverity::Critical => "🔴",
        }
    }
}

impl PasswordStrength {
    pub fn emoji(&self) -> &'static str {
        match self {
            PasswordStrength::Weak => "🔴",
            PasswordStrength::Medium => "🟡",
            PasswordStrength::Strong => "🟢",
            PasswordStrength::VeryStrong => "💪",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            PasswordStrength::Weak => "Weak - easy to guess",
            PasswordStrength::Medium => "Medium - acceptable",
            PasswordStrength::Strong => "Strong - secure",
            PasswordStrength::VeryStrong => "Very Strong - excellent",
        }
    }
}

impl AdBlockRuleType {
    pub fn emoji(&self) -> &'static str {
        match self {
            AdBlockRuleType::Block => "🚫",
            AdBlockRuleType::Allow => "✅",
            AdBlockRuleType::Hide => "🫥",
        }
    }
} 
