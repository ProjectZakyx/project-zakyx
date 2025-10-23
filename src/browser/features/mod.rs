// 🔧 Browser Features Module
// Erweiterte Browser-Funktionen für ZAKYX Browser

pub mod download_manager;
pub mod password_manager;
pub mod extension_manager;
pub mod developer_tools;

// Re-exports für einfachen Zugriff
pub use download_manager::{DownloadManager, Download, DownloadStatus, DownloadStats};
pub use password_manager::{PasswordManager, SavedPassword, PasswordStrength, PasswordStats};
pub use extension_manager::{ExtensionManager, Extension, ExtensionCategory, ExtensionStats};
pub use developer_tools::{DeveloperTools, ConsoleLogEntry, NetworkLogEntry, LogLevel, LogFilters};

// Hauptklasse für Advanced Browser Features
use anyhow::Result;
use std::path::PathBuf;
use windows::Win32::Foundation::HWND;

pub struct AdvancedBrowserFeatures {
    download_manager: DownloadManager,
    password_manager: PasswordManager,
    extension_manager: ExtensionManager,
    developer_tools: DeveloperTools,
    window_handle: Option<HWND>,
}

impl AdvancedBrowserFeatures {
    pub fn new(window_handle: Option<HWND>) -> Result<Self> {
        println!("🔧 Initializing Advanced Browser Features...");
        
        Ok(Self {
            download_manager: DownloadManager::new()?,
            password_manager: PasswordManager::new()?,
            extension_manager: ExtensionManager::new()?,
            developer_tools: DeveloperTools::new(),
            window_handle,
        })
    }

    /// Verarbeite Kommando
    pub async fn handle_command(&mut self, command: &str) -> Result<Vec<String>> {
        match command.trim().to_lowercase().as_str() {
            // Download Commands
            "downloads" => Ok(self.get_downloads_display()),
            cmd if cmd.starts_with("download ") => {
                let url = cmd.strip_prefix("download ").unwrap_or("");
                let filename = format!("file_{}.html", chrono::Utc::now().timestamp());
                let _ = self.start_download(url, &filename).await?;
                Ok(vec![format!("📥 Download started: {}", url)])
            }
            cmd if cmd.starts_with("download pause ") => {
                let download_id = cmd.strip_prefix("download pause ").unwrap_or("");
                match self.download_manager.pause_download(download_id) {
                    Ok(_) => Ok(vec![format!("⏸️ Download paused: {}", download_id)]),
                    Err(e) => Ok(vec![format!("❌ Error: {}", e)]),
                }
            }
            cmd if cmd.starts_with("download resume ") => {
                let download_id = cmd.strip_prefix("download resume ").unwrap_or("");
                match self.download_manager.resume_download(download_id).await {
                    Ok(_) => Ok(vec![format!("▶️ Download resumed: {}", download_id)]),
                    Err(e) => Ok(vec![format!("❌ Error: {}", e)]),
                }
            }

            // Password Commands
            "passwords" => Ok(self.get_passwords_display()),
            cmd if cmd.starts_with("password unlock ") => {
                let master_password = cmd.strip_prefix("password unlock ").unwrap_or("");
                match self.password_manager.unlock(master_password) {
                    Ok(_) => Ok(vec!["🔓 Password manager unlocked".to_string()]),
                    Err(e) => Ok(vec![format!("❌ Error: {}", e)]),
                }
            }
            "password lock" => {
                self.password_manager.lock();
                Ok(vec!["🔒 Password manager locked".to_string()])
            }
            cmd if cmd.starts_with("password save ") => {
                let parts: Vec<&str> = cmd.strip_prefix("password save ").unwrap_or("").split_whitespace().collect();
                if parts.len() >= 3 {
                    match self.password_manager.save_password(parts[0], parts[1], parts[2]).await {
                        Ok(id) => Ok(vec![format!("🔐 Password saved: {}", id)]),
                        Err(e) => Ok(vec![format!("❌ Error: {}", e)]),
                    }
                } else {
                    Ok(vec!["❌ Usage: password save <website> <username> <password>".to_string()])
                }
            }

            // Extension Commands
            "extensions" => Ok(self.get_extensions_display()),
            cmd if cmd.starts_with("extension toggle ") => {
                let ext_id = cmd.strip_prefix("extension toggle ").unwrap_or("");
                match self.extension_manager.toggle_extension(ext_id) {
                    Ok(enabled) => Ok(vec![format!(
                        "🔌 Extension '{}': {}",
                        ext_id,
                        if enabled { "Enabled" } else { "Disabled" }
                    )]),
                    Err(e) => Ok(vec![format!("❌ Error: {}", e)]),
                }
            }
            cmd if cmd.starts_with("extension install ") => {
                let file_path = cmd.strip_prefix("extension install ").unwrap_or("");
                match self.extension_manager.install_extension(PathBuf::from(file_path)).await {
                    Ok(id) => Ok(vec![format!("📦 Extension installed: {}", id)]),
                    Err(e) => Ok(vec![format!("❌ Error: {}", e)]),
                }
            }

            // Developer Tools Commands
            "devtools" => Ok(self.get_devtools_display()),
            "devtools toggle" => {
                let enabled = self.developer_tools.toggle();
                Ok(vec![format!(
                    "🔧 Developer Tools: {}",
                    if enabled { "Enabled" } else { "Disabled" }
                )])
            }
            "console clear" => {
                self.developer_tools.clear_console_logs();
                Ok(vec!["🧹 Console logs cleared".to_string()])
            }
            "network clear" => {
                self.developer_tools.clear_network_logs();
                Ok(vec!["🧹 Network logs cleared".to_string()])
            }
            "export logs" => {
                match self.developer_tools.export_logs() {
                    Ok(json) => {
                        // In Realität würde man das in eine Datei schreiben
                        Ok(vec![format!("📤 Logs exported ({} characters)", json.len())])
                    }
                    Err(e) => Ok(vec![format!("❌ Export failed: {}", e)]),
                }
            }

            // General Commands
            "stats" => Ok(self.get_stats_display()),
            "cleanup" => {
                let removed_downloads = self.download_manager.cleanup_completed();
                Ok(vec![format!("🧹 Cleaned up {} completed downloads", removed_downloads)])
            }

            _ => Ok(vec![
                "❓ Unknown command. Available commands:".to_string(),
                "• downloads, passwords, extensions, devtools".to_string(),
                "• download <url>, password unlock <master>, extension toggle <id>".to_string(),
                "• devtools toggle, console clear, network clear".to_string(),
                "• stats, cleanup".to_string(),
            ]),
        }
    }

    /// Download-Funktionen
    pub async fn start_download(&mut self, url: &str, filename: &str) -> Result<String> {
        self.developer_tools.log_network_request("GET", url);
        let download_id = self.download_manager.start_download(url, filename).await?;
        self.developer_tools.log_network_response(url, 200, 1500);
        Ok(download_id)
    }

    /// Passwort-Funktionen
    pub async fn save_password(&mut self, website: &str, username: &str, password: &str) -> Result<String> {
        self.password_manager.save_password(website, username, password).await
    }

    pub fn get_password_for_site(&self, website: &str) -> Option<&SavedPassword> {
        self.password_manager.get_password_for_site(website)
    }

    /// Extension-Funktionen
    pub fn toggle_extension(&mut self, extension_id: &str) -> Result<bool> {
        self.extension_manager.toggle_extension(extension_id)
    }

    /// Developer Tools-Funktionen
    pub fn toggle_devtools(&mut self) -> bool {
        self.developer_tools.toggle()
    }

    pub fn log_console(&mut self, level: LogLevel, message: &str, source: &str) {
        self.developer_tools.log_console(level, message, source);
    }

    pub fn log_network_request(&mut self, method: &str, url: &str) -> String {
        self.developer_tools.log_network_request(method, url)
    }

    /// Display-Funktionen
    pub fn get_downloads_display(&self) -> Vec<String> {
        self.download_manager.get_downloads_display()
    }

    pub fn get_passwords_display(&self) -> Vec<String> {
        self.password_manager.get_passwords_display()
    }

    pub fn get_extensions_display(&self) -> Vec<String> {
        self.extension_manager.get_extensions_display()
    }

    pub fn get_devtools_display(&self) -> Vec<String> {
        self.developer_tools.get_devtools_display()
    }

    pub fn get_advanced_features_display(&self) -> Vec<String> {
        vec![
            "🔧 ADVANCED BROWSER FEATURES".to_string(),
            "=============================".to_string(),
            "".to_string(),
            "💬 VERFÜGBARE KOMMANDOS:".to_string(),
            "• 'downloads' → Download Manager".to_string(),
            "• 'passwords' → Password Manager".to_string(),
            "• 'extensions' → Extension Manager".to_string(),
            "• 'devtools' → Developer Tools".to_string(),
            "• 'stats' → Statistiken anzeigen".to_string(),
            "".to_string(),
            "🎯 FEATURES:".to_string(),
            "• 📥 Download Management mit Progress-Tracking".to_string(),
            "• 🔐 Sicherer Password Manager".to_string(),
            "• 🔌 Extension System".to_string(),
            "• 🔧 Developer Tools mit Console & Network-Logs".to_string(),
            "• 📊 Performance-Monitoring".to_string(),
            "".to_string(),
            "⚡ Kommando eingeben und Enter drücken!".to_string(),
        ]
    }

    pub fn get_stats_display(&self) -> Vec<String> {
        let download_stats = self.download_manager.get_stats();
        let password_stats = self.password_manager.get_stats();
        let extension_stats = self.extension_manager.get_stats();

        vec![
            "📊 BROWSER FEATURES STATISTIKEN".to_string(),
            "===============================".to_string(),
            "".to_string(),
            "📥 DOWNLOADS:".to_string(),
            format!("   • Gesamt: {} Downloads", download_stats.pending + download_stats.in_progress + download_stats.completed + download_stats.failed),
            format!("   • Aktiv: {} / Abgeschlossen: {}", download_stats.in_progress, download_stats.completed),
            format!("   • Fehlgeschlagen: {} / Pausiert: {}", download_stats.failed, download_stats.paused),
            format!("   • Heruntergeladen: {}", format_bytes(download_stats.total_downloaded)),
            "".to_string(),
            "🔐 PASSWÖRTER:".to_string(),
            format!("   • Gesamt: {} Passwörter", password_stats.total_passwords),
            format!("   • Auto-Fill aktiviert: {}", password_stats.auto_fill_enabled),
            format!("   • Schwache Passwörter: {}", password_stats.weak_passwords),
            format!("   • Starke Passwörter: {}", password_stats.strong_passwords + password_stats.very_strong_passwords),
            "".to_string(),
            "🔌 EXTENSIONS:".to_string(),
            format!("   • Gesamt: {} Extensions", extension_stats.total_extensions),
            format!("   • Aktiviert: {}", extension_stats.enabled_extensions),
            format!("   • Gesamtgröße: {}", format_bytes(extension_stats.total_size)),
            "".to_string(),
            "🔧 DEVELOPER TOOLS:".to_string(),
            format!("   • Status: {}", if self.developer_tools.is_enabled() { "✅ Aktiv" } else { "❌ Inaktiv" }),
            format!("   • Console-Logs: {}", self.developer_tools.get_console_log_count()),
            format!("   • Network-Logs: {}", self.developer_tools.get_network_log_count()),
        ]
    }

    /// Cleanup
    pub async fn cleanup(&self) -> Result<()> {
        println!("🧹 Advanced Browser Features cleanup...");
        Ok(())
    }

    // Getter für Submodule
    pub fn get_download_manager(&self) -> &DownloadManager {
        &self.download_manager
    }

    pub fn get_download_manager_mut(&mut self) -> &mut DownloadManager {
        &mut self.download_manager
    }

    pub fn get_password_manager(&self) -> &PasswordManager {
        &self.password_manager
    }

    pub fn get_password_manager_mut(&mut self) -> &mut PasswordManager {
        &mut self.password_manager
    }

    pub fn get_extension_manager(&self) -> &ExtensionManager {
        &self.extension_manager
    }

    pub fn get_extension_manager_mut(&mut self) -> &mut ExtensionManager {
        &mut self.extension_manager
    }

    pub fn get_developer_tools(&self) -> &DeveloperTools {
        &self.developer_tools
    }

    pub fn get_developer_tools_mut(&mut self) -> &mut DeveloperTools {
        &mut self.developer_tools
    }
}

/// Formatiere Bytes in menschenlesbares Format
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

impl Default for AdvancedBrowserFeatures {
    fn default() -> Self {
        Self::new(None).unwrap_or_else(|_| Self {
            download_manager: DownloadManager::default(),
            password_manager: PasswordManager::default(),
            extension_manager: ExtensionManager::default(),
            developer_tools: DeveloperTools::default(),
            window_handle: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_advanced_browser_features() {
        let mut features = AdvancedBrowserFeatures::new(None).unwrap();
        
        // Test command handling
        let result = features.handle_command("downloads").await.unwrap();
        assert!(!result.is_empty());
        
        let result = features.handle_command("stats").await.unwrap();
        assert!(result.iter().any(|line| line.contains("STATISTIKEN")));
    }

    #[tokio::test]
    async fn test_download_integration() {
        let mut features = AdvancedBrowserFeatures::new(None).unwrap();
        
        let download_id = features.start_download("https://example.com/file.txt", "test.txt").await.unwrap();
        assert!(!download_id.is_empty());
        
        let downloads = features.get_downloads_display();
        assert!(downloads.iter().any(|line| line.contains("test.txt")));
    }

    #[tokio::test]
    async fn test_password_integration() {
        let mut features = AdvancedBrowserFeatures::new(None).unwrap();
        
        // Unlock password manager first
        let _ = features.password_manager.unlock("test_master");
        
        let password_id = features.save_password("example.com", "user", "password123").await.unwrap();
        assert!(!password_id.is_empty());
        
        let found_password = features.get_password_for_site("example.com");
        assert!(found_password.is_some());
        assert_eq!(found_password.unwrap().username, "user");
    }

    #[test]
    fn test_developer_tools_integration() {
        let mut features = AdvancedBrowserFeatures::new(None).unwrap();
        
        let enabled = features.toggle_devtools();
        assert!(enabled);
        
        features.log_console(LogLevel::Info, "Test message", "TestSource");
        let devtools_display = features.get_devtools_display();
        assert!(devtools_display.iter().any(|line| line.contains("Test message")));
    }

    #[test]
    fn test_extension_integration() {
        let mut features = AdvancedBrowserFeatures::new(None).unwrap();
        
        let extensions = features.get_extensions_display();
        // Should have default extensions
        assert!(extensions.iter().any(|line| line.contains("Ad Blocker")));
        
        let stats = features.extension_manager.get_stats();
        assert!(stats.total_extensions > 0);
    }
}