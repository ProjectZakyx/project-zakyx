// 🔐 Password Manager für ZAKYX Browser
// Sichere Passwort-Speicherung, Auto-Fill und Passwort-Generierung

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedPassword {
    pub id: String,
    pub website: String,
    pub username: String,
    pub password_hash: String, // In Realität würde man das verschlüsseln
    pub auto_fill: bool,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub use_count: u32,
    pub notes: Option<String>,
}

impl SavedPassword {
    pub fn new(website: String, username: String, password: String) -> Self {
        let id = format!("pwd_{}", Utc::now().timestamp());
        
        // Einfache "Verschlüsselung" (nur Demo!)
        // In Produktion würde man richtige Verschlüsselung verwenden
        let password_hash = format!("hash_{}_len_{}", 
            password.chars().map(|c| c as u32).sum::<u32>(), 
            password.len()
        );

        Self {
            id,
            website,
            username,
            password_hash,
            auto_fill: true,
            created_at: Utc::now(),
            last_used: None,
            use_count: 0,
            notes: None,
        }
    }

    /// Markiere als verwendet
    pub fn mark_used(&mut self) {
        self.last_used = Some(Utc::now());
        self.use_count += 1;
    }

    /// Prüfe ob Passwort für Website passt
    pub fn matches_website(&self, url: &str) -> bool {
        let url_lower = url.to_lowercase();
        let website_lower = self.website.to_lowercase();
        
        // Exact match
        if url_lower.contains(&website_lower) {
            return true;
        }
        
        // Domain extraction and comparison
        if let (Some(url_domain), Some(website_domain)) = (extract_domain(&url_lower), extract_domain(&website_lower)) {
            url_domain == website_domain
        } else {
            false
        }
    }

    /// Hole Passwort-Stärke (basierend auf Hash-Info)
    pub fn get_password_strength(&self) -> PasswordStrength {
        // Extrahiere Länge aus Hash (Demo-Implementation)
        if let Some(len_part) = self.password_hash.split("_len_").nth(1) {
            if let Ok(length) = len_part.parse::<usize>() {
                return match length {
                    0..=6 => PasswordStrength::Weak,
                    7..=10 => PasswordStrength::Medium,
                    11..=15 => PasswordStrength::Strong,
                    _ => PasswordStrength::VeryStrong,
                };
            }
        }
        PasswordStrength::Unknown
    }
}

#[derive(Debug, Clone)]
pub enum PasswordStrength {
    Weak,
    Medium,
    Strong,
    VeryStrong,
    Unknown,
}

impl PasswordStrength {
    pub fn to_string(&self) -> String {
        match self {
            PasswordStrength::Weak => "🔴 Schwach".to_string(),
            PasswordStrength::Medium => "🟡 Mittel".to_string(),
            PasswordStrength::Strong => "🟢 Stark".to_string(),
            PasswordStrength::VeryStrong => "💪 Sehr stark".to_string(),
            PasswordStrength::Unknown => "❓ Unbekannt".to_string(),
        }
    }

    pub fn score(&self) -> u8 {
        match self {
            PasswordStrength::Weak => 25,
            PasswordStrength::Medium => 50,
            PasswordStrength::Strong => 75,
            PasswordStrength::VeryStrong => 100,
            PasswordStrength::Unknown => 0,
        }
    }
}

pub struct PasswordManager {
    passwords: HashMap<String, SavedPassword>,
    data_file: PathBuf,
    master_password_hash: Option<String>,
    is_locked: bool,
    auto_save: bool,
}

impl PasswordManager {
    pub fn new() -> Result<Self> {
        let data_file = std::env::current_dir()?.join("passwords.json");
        let mut manager = Self {
            passwords: HashMap::new(),
            data_file,
            master_password_hash: None,
            is_locked: true,
            auto_save: true,
        };

        // Lade gespeicherte Passwörter
        if let Err(e) = manager.load_passwords() {
            println!("⚠️ Could not load passwords: {:?}", e);
        }

        Ok(manager)
    }

    /// Entsperre Password Manager mit Master-Passwort
    pub fn unlock(&mut self, master_password: &str) -> Result<()> {
        // In Produktion würde man das Master-Passwort richtig hashen und prüfen
        let _expected_hash = "master_hash_demo";
        let provided_hash = format!("master_hash_{}", master_password.len());
        
        if self.master_password_hash.is_none() {
            // Erstes Mal - setze Master-Passwort
            self.master_password_hash = Some(provided_hash);
            self.is_locked = false;
            println!("🔓 Master password set and unlocked");
            Ok(())
        } else if self.master_password_hash.as_ref() == Some(&provided_hash) {
            self.is_locked = false;
            println!("🔓 Password manager unlocked");
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid master password"))
        }
    }

    /// Sperre Password Manager
    pub fn lock(&mut self) {
        self.is_locked = true;
        println!("🔒 Password manager locked");
    }

    /// Speichere neues Passwort
    pub async fn save_password(&mut self, website: &str, username: &str, password: &str) -> Result<String> {
        if self.is_locked {
            return Err(anyhow::anyhow!("Password manager is locked"));
        }

        // Prüfe ob bereits ein Passwort für diese Website/Username-Kombination existiert
        if let Some(_existing) = self.find_password_by_credentials(website, username) {
            return Err(anyhow::anyhow!("Password already exists for {} / {}", website, username));
        }

        let saved_password = SavedPassword::new(
            website.to_string(),
            username.to_string(),
            password.to_string()
        );

        let password_id = saved_password.id.clone();
        self.passwords.insert(password_id.clone(), saved_password);

        if self.auto_save {
            self.save_passwords().await?;
        }

        println!("🔐 Password saved for: {} / {}", website, username);
        Ok(password_id)
    }

    /// Aktualisiere existierendes Passwort
    pub async fn update_password(&mut self, password_id: &str, new_password: &str) -> Result<()> {
        if self.is_locked {
            return Err(anyhow::anyhow!("Password manager is locked"));
        }

        if let Some(saved_password) = self.passwords.get_mut(password_id) {
            // Update password hash
            saved_password.password_hash = format!("hash_{}_len_{}", 
                new_password.chars().map(|c| c as u32).sum::<u32>(), 
                new_password.len()
            );
            
            let website = saved_password.website.clone();
            
            if self.auto_save {
                self.save_passwords().await?;
            }

            println!("🔄 Password updated for: {}", website);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Password not found: {}", password_id))
        }
    }

    /// Lösche Passwort
    pub async fn delete_password(&mut self, password_id: &str) -> Result<()> {
        if self.is_locked {
            return Err(anyhow::anyhow!("Password manager is locked"));
        }

        if let Some(password) = self.passwords.remove(password_id) {
            if self.auto_save {
                self.save_passwords().await?;
            }

            println!("🗑️ Password deleted for: {}", password.website);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Password not found: {}", password_id))
        }
    }

    /// Finde Passwort für Website
    pub fn get_password_for_site(&self, website: &str) -> Option<&SavedPassword> {
        if self.is_locked {
            return None;
        }

        self.passwords.values()
            .filter(|pwd| pwd.auto_fill)
            .find(|pwd| pwd.matches_website(website))
    }

    /// Finde alle Passwörter für Website
    pub fn get_passwords_for_site(&self, website: &str) -> Vec<&SavedPassword> {
        if self.is_locked {
            return Vec::new();
        }

        self.passwords.values()
            .filter(|pwd| pwd.matches_website(website))
            .collect()
    }

    /// Finde Passwort nach Credentials
    fn find_password_by_credentials(&self, website: &str, username: &str) -> Option<&SavedPassword> {
        self.passwords.values()
            .find(|pwd| pwd.website.eq_ignore_ascii_case(website) && 
                       pwd.username.eq_ignore_ascii_case(username))
    }

    /// Markiere Passwort als verwendet
    pub fn mark_password_used(&mut self, password_id: &str) {
        if let Some(password) = self.passwords.get_mut(password_id) {
            password.mark_used();
        }
    }

    /// Suche Passwörter
    pub fn search_passwords(&self, query: &str) -> Vec<&SavedPassword> {
        if self.is_locked {
            return Vec::new();
        }

        let query_lower = query.to_lowercase();
        self.passwords.values()
            .filter(|pwd| {
                pwd.website.to_lowercase().contains(&query_lower) ||
                pwd.username.to_lowercase().contains(&query_lower) ||
                pwd.notes.as_ref().map_or(false, |notes| notes.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Hole Passwort-Statistiken
    pub fn get_stats(&self) -> PasswordStats {
        if self.is_locked {
            return PasswordStats::default();
        }

        let mut stats = PasswordStats::default();
        stats.total_passwords = self.passwords.len();

        for password in self.passwords.values() {
            match password.get_password_strength() {
                PasswordStrength::Weak => stats.weak_passwords += 1,
                PasswordStrength::Medium => stats.medium_passwords += 1,
                PasswordStrength::Strong => stats.strong_passwords += 1,
                PasswordStrength::VeryStrong => stats.very_strong_passwords += 1,
                PasswordStrength::Unknown => {}
            }

            if password.auto_fill {
                stats.auto_fill_enabled += 1;
            }

            if password.last_used.is_some() {
                stats.used_passwords += 1;
            }
        }

        stats
    }

    /// Generiere sicheres Passwort
    pub fn generate_password(&self, length: usize, include_symbols: bool) -> String {
        use rand::Rng;
        
        let mut chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string();
        if include_symbols {
            chars.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
        }

        let chars: Vec<char> = chars.chars().collect();
        let mut rng = rand::thread_rng();
        
        (0..length)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect()
    }

    /// Exportiere Passwörter (verschlüsselt)
    pub async fn export_passwords(&self) -> Result<String> {
        if self.is_locked {
            return Err(anyhow::anyhow!("Password manager is locked"));
        }

        let export_data = serde_json::json!({
            "passwords": self.passwords.values().collect::<Vec<_>>(),
            "export_timestamp": Utc::now().to_rfc3339(),
            "version": "1.0"
        });

        serde_json::to_string_pretty(&export_data)
            .map_err(|e| anyhow::anyhow!("Export failed: {}", e))
    }

    /// Importiere Passwörter
    pub async fn import_passwords(&mut self, json_data: &str) -> Result<usize> {
        if self.is_locked {
            return Err(anyhow::anyhow!("Password manager is locked"));
        }

        let import_data: serde_json::Value = serde_json::from_str(json_data)?;
        
        if let Some(passwords_array) = import_data.get("passwords") {
            let imported_passwords: Vec<SavedPassword> = serde_json::from_value(passwords_array.clone())?;
            let mut imported_count = 0;

            for password in imported_passwords {
                if !self.passwords.contains_key(&password.id) {
                    self.passwords.insert(password.id.clone(), password);
                    imported_count += 1;
                }
            }

            if self.auto_save && imported_count > 0 {
                self.save_passwords().await?;
            }

            println!("📥 Imported {} passwords", imported_count);
            Ok(imported_count)
        } else {
            Err(anyhow::anyhow!("Invalid import format"))
        }
    }

    /// Lade Passwörter von Datei
    fn load_passwords(&mut self) -> Result<()> {
        if !self.data_file.exists() {
            return Ok(()); // Keine Datei vorhanden
        }

        let content = std::fs::read_to_string(&self.data_file)?;
        if content.trim().is_empty() {
            return Ok(());
        }

        let data: serde_json::Value = serde_json::from_str(&content)?;
        
        if let Some(passwords_obj) = data.get("passwords") {
            self.passwords = serde_json::from_value(passwords_obj.clone())?;
        }

        if let Some(master_hash) = data.get("master_password_hash") {
            self.master_password_hash = serde_json::from_value(master_hash.clone())?;
        }

        println!("📥 Loaded {} passwords", self.passwords.len());
        Ok(())
    }

    /// Speichere Passwörter in Datei
    async fn save_passwords(&self) -> Result<()> {
        let data = serde_json::json!({
            "passwords": self.passwords,
            "master_password_hash": self.master_password_hash,
            "saved_at": Utc::now().to_rfc3339()
        });

        let content = serde_json::to_string_pretty(&data)?;
        fs::write(&self.data_file, content).await?;
        
        println!("💾 Passwords saved to file");
        Ok(())
    }

    /// Hole Display für Passwörter
    pub fn get_passwords_display(&self) -> Vec<String> {
        let mut result = vec![
            "🔐 PASSWORD MANAGER".to_string(),
            "===================".to_string(),
            "".to_string(),
        ];

        if self.is_locked {
            result.extend(vec![
                "🔒 Password Manager ist gesperrt".to_string(),
                "💡 Verwende 'unlock <master-password>' zum Entsperren".to_string(),
            ]);
            return result;
        }

        let stats = self.get_stats();
        result.extend(vec![
            format!("📊 Statistiken:"),
            format!("   • Gesamt: {} Passwörter", stats.total_passwords),
            format!("   • Auto-Fill: {} aktiviert", stats.auto_fill_enabled),
            format!("   • Verwendet: {} Passwörter", stats.used_passwords),
            "".to_string(),
            format!("🔒 Passwort-Stärke:"),
            format!("   • 🔴 Schwach: {}", stats.weak_passwords),
            format!("   • 🟡 Mittel: {}", stats.medium_passwords),
            format!("   • 🟢 Stark: {}", stats.strong_passwords),
            format!("   • 💪 Sehr stark: {}", stats.very_strong_passwords),
            "".to_string(),
        ]);

        if self.passwords.is_empty() {
            result.push("📭 Keine Passwörter gespeichert".to_string());
        } else {
            result.push("📋 GESPEICHERTE PASSWÖRTER:".to_string());
            for password in self.passwords.values() {
                result.push(format!("🌐 {}", password.website));
                result.push(format!("   Benutzer: {}", password.username));
                result.push(format!("   Stärke: {}", password.get_password_strength().to_string()));
                result.push(format!("   Auto-Fill: {}", if password.auto_fill { "✅" } else { "❌" }));
                result.push(format!("   Verwendet: {} mal", password.use_count));
                
                if let Some(last_used) = password.last_used {
                    result.push(format!("   Zuletzt: {}", last_used.format("%d.%m.%Y %H:%M")));
                }
                
                if let Some(notes) = &password.notes {
                    result.push(format!("   Notizen: {}", notes));
                }
                
                result.push("".to_string());
            }
        }

        result
    }

    // Getter & Setter
    pub fn is_locked(&self) -> bool {
        self.is_locked
    }

    pub fn set_auto_save(&mut self, enabled: bool) {
        self.auto_save = enabled;
    }

    pub fn get_password_count(&self) -> usize {
        if self.is_locked { 0 } else { self.passwords.len() }
    }

    pub fn get_password(&self, password_id: &str) -> Option<&SavedPassword> {
        if self.is_locked { None } else { self.passwords.get(password_id) }
    }
}

#[derive(Debug, Default)]
pub struct PasswordStats {
    pub total_passwords: usize,
    pub weak_passwords: usize,
    pub medium_passwords: usize,
    pub strong_passwords: usize,
    pub very_strong_passwords: usize,
    pub auto_fill_enabled: usize,
    pub used_passwords: usize,
}

/// Extrahiere Domain aus URL
fn extract_domain(url: &str) -> Option<String> {
    if let Some(start) = url.find("://") {
        let after_protocol = &url[start + 3..];
        let domain_end = after_protocol
            .find('/')
            .or_else(|| after_protocol.find('?'))
            .or_else(|| after_protocol.find('#'))
            .unwrap_or(after_protocol.len());
        
        let domain_with_port = &after_protocol[..domain_end];
        
        if let Some(colon_pos) = domain_with_port.find(':') {
            Some(domain_with_port[..colon_pos].to_string())
        } else {
            Some(domain_with_port.to_string())
        }
    } else {
        None
    }
}

impl Default for PasswordManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            passwords: HashMap::new(),
            data_file: PathBuf::from("passwords.json"),
            master_password_hash: None,
            is_locked: true,
            auto_save: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_creation() {
        let password = SavedPassword::new(
            "example.com".to_string(),
            "user@example.com".to_string(),
            "password123".to_string()
        );
        
        assert_eq!(password.website, "example.com");
        assert_eq!(password.username, "user@example.com");
        assert!(password.auto_fill);
        assert_eq!(password.use_count, 0);
    }

    #[test]
    fn test_website_matching() {
        let password = SavedPassword::new(
            "example.com".to_string(),
            "user".to_string(),
            "pass".to_string()
        );
        
        assert!(password.matches_website("https://example.com/login"));
        assert!(password.matches_website("http://www.example.com"));
        assert!(!password.matches_website("https://different.com"));
    }

    #[test]
    fn test_password_strength() {
        let weak_password = SavedPassword::new("site.com".to_string(), "user".to_string(), "123".to_string());
        let strong_password = SavedPassword::new("site.com".to_string(), "user".to_string(), "verylongpassword123".to_string());
        
        assert!(matches!(weak_password.get_password_strength(), PasswordStrength::Weak));
        assert!(matches!(strong_password.get_password_strength(), PasswordStrength::VeryStrong));
    }

    #[tokio::test]
    async fn test_password_manager() {
        let mut manager = PasswordManager::new().unwrap();
        
        // Unlock first
        manager.unlock("test_master").unwrap();
        
        // Save password
        let id = manager.save_password("example.com", "user", "password123").await.unwrap();
        assert_eq!(manager.get_password_count(), 1);
        
        // Find password
        let found = manager.get_password_for_site("https://example.com/login");
        assert!(found.is_some());
        assert_eq!(found.unwrap().username, "user");
        
        // Delete password
        manager.delete_password(&id).await.unwrap();
        assert_eq!(manager.get_password_count(), 0);
    }

    #[test]
    fn test_password_generation() {
        let manager = PasswordManager::new().unwrap();
        
        let password = manager.generate_password(12, true);
        assert_eq!(password.len(), 12);
        
        let password_no_symbols = manager.generate_password(8, false);
        assert_eq!(password_no_symbols.len(), 8);
        assert!(!password_no_symbols.chars().any(|c| "!@#$%^&*()".contains(c)));
    }

    #[test]
    fn test_domain_extraction() {
        assert_eq!(extract_domain("https://example.com/path"), Some("example.com".to_string()));
        assert_eq!(extract_domain("http://sub.example.com:8080/path"), Some("sub.example.com".to_string()));
        assert_eq!(extract_domain("invalid_url"), None);
    }
}