// 🌐 WebView2 Environment Detection für ZAKYX Browser
// Erkennung und Validierung der WebView2-Runtime-Umgebung

use std::process::Command;
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// WebView2-Environment-Informationen
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebView2EnvironmentInfo {
    /// WebView2-Version
    pub version: String,
    
    /// Installations-Pfad
    pub installation_path: String,
    
    /// Verfügbarkeit
    pub is_available: bool,
    
    /// Runtime-Typ
    pub runtime_type: WebView2RuntimeType,
    
    /// Channel-Information
    pub channel: WebView2Channel,
    
    /// Architektur
    pub architecture: String,
    
    /// Installation-Datum
    pub installation_date: Option<String>,
    
    /// Zusätzliche Informationen
    pub additional_info: std::collections::HashMap<String, String>,
}

/// WebView2-Runtime-Typ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WebView2RuntimeType {
    /// Evergreen (automatische Updates)
    Evergreen,
    
    /// Fixed Version (spezifische Version)
    FixedVersion,
    
    /// Edge Canary
    EdgeCanary,
    
    /// Edge Dev
    EdgeDev,
    
    /// Edge Beta
    EdgeBeta,
    
    /// Edge Stable
    EdgeStable,
    
    /// Unbekannt
    Unknown,
}

/// WebView2-Channel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WebView2Channel {
    /// Stable Channel
    Stable,
    
    /// Beta Channel
    Beta,
    
    /// Dev Channel
    Dev,
    
    /// Canary Channel
    Canary,
    
    /// Unbekannt
    Unknown,
}

impl std::fmt::Display for WebView2RuntimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebView2RuntimeType::Evergreen => write!(f, "Evergreen"),
            WebView2RuntimeType::FixedVersion => write!(f, "Fixed Version"),
            WebView2RuntimeType::EdgeCanary => write!(f, "Edge Canary"),
            WebView2RuntimeType::EdgeDev => write!(f, "Edge Dev"),
            WebView2RuntimeType::EdgeBeta => write!(f, "Edge Beta"),
            WebView2RuntimeType::EdgeStable => write!(f, "Edge Stable"),
            WebView2RuntimeType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl std::fmt::Display for WebView2Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebView2Channel::Stable => write!(f, "Stable"),
            WebView2Channel::Beta => write!(f, "Beta"),
            WebView2Channel::Dev => write!(f, "Dev"),
            WebView2Channel::Canary => write!(f, "Canary"),
            WebView2Channel::Unknown => write!(f, "Unknown"),
        }
    }
}

/// WebView2-Environment-Detector
#[derive(Debug)]
pub struct WebView2EnvironmentDetector {
    /// Cache für Environment-Informationen
    cached_info: Option<WebView2EnvironmentInfo>,
    
    /// Letzte Überprüfung
    last_check: Option<std::time::SystemTime>,
    
    /// Cache-Gültigkeitsdauer in Sekunden
    cache_duration: u64,
}

impl WebView2EnvironmentDetector {
    /// Erstellt einen neuen Environment-Detector
    pub fn new() -> Self {
        Self {
            cached_info: None,
            last_check: None,
            cache_duration: 300, // 5 Minuten
        }
    }

    /// Erstellt einen Detector mit angepasster Cache-Dauer
    pub fn with_cache_duration(cache_duration: u64) -> Self {
        Self {
            cached_info: None,
            last_check: None,
            cache_duration,
        }
    }

    /// Überprüft WebView2-Verfügbarkeit (mit Cache)
    pub fn check_availability(&mut self) -> Result<WebView2EnvironmentInfo> {
        // Cache-Validierung
        if let (Some(cached), Some(last_check)) = (&self.cached_info, self.last_check) {
            if let Ok(elapsed) = last_check.elapsed() {
                if elapsed.as_secs() < self.cache_duration {
                    println!("📋 Using cached WebView2 environment info");
                    return Ok(cached.clone());
                }
            }
        }

        println!("🔍 Detecting WebView2 environment...");
        
        // Neue Erkennung durchführen
        let info = self.detect_environment()?;
        
        // Cache aktualisieren
        self.cached_info = Some(info.clone());
        self.last_check = Some(std::time::SystemTime::now());
        
        Ok(info)
    }

    /// Erzwingt eine neue Erkennung (ignoriert Cache)
    pub fn force_detection(&mut self) -> Result<WebView2EnvironmentInfo> {
        self.cached_info = None;
        self.last_check = None;
        self.check_availability()
    }

    /// Führt die eigentliche Environment-Erkennung durch
    fn detect_environment(&self) -> Result<WebView2EnvironmentInfo> {
        let mut info = WebView2EnvironmentInfo {
            version: "Unknown".to_string(),
            installation_path: "Not Found".to_string(),
            is_available: false,
            runtime_type: WebView2RuntimeType::Unknown,
            channel: WebView2Channel::Unknown,
            architecture: self.detect_architecture(),
            installation_date: None,
            additional_info: std::collections::HashMap::new(),
        };

        // Verschiedene Erkennungsmethoden ausprobieren
        if let Ok(registry_info) = self.detect_from_registry() {
            info = registry_info;
        } else if let Ok(path_info) = self.detect_from_standard_paths() {
            info = path_info;
        } else if let Ok(edge_info) = self.detect_from_edge_installation() {
            info = edge_info;
        }

        // Zusätzliche Validierung
        if info.is_available {
            info.is_available = self.validate_installation(&info.installation_path);
        }

        // Zusätzliche Informationen sammeln
        self.gather_additional_info(&mut info);

        Ok(info)
    }

    /// Erkennt WebView2 aus der Registry
    fn detect_from_registry(&self) -> Result<WebView2EnvironmentInfo> {
        println!("🔍 Checking Windows Registry for WebView2...");
        
        // WebView2 Runtime Registry-Schlüssel
        let registry_keys = vec![
            r"HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
            r"HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
        ];

        for key in registry_keys {
            if let Ok(version) = self.query_registry_value(key, "pv") {
                if let Ok(path) = self.query_registry_value(key, "EBWebView") {
                    return Ok(WebView2EnvironmentInfo {
                        version,
                        installation_path: path,
                        is_available: true,
                        runtime_type: WebView2RuntimeType::Evergreen,
                        channel: WebView2Channel::Stable,
                        architecture: self.detect_architecture(),
                        installation_date: self.query_registry_value(key, "InstallTime").ok(),
                        additional_info: std::collections::HashMap::new(),
                    });
                }
            }
        }

        Err(anyhow::anyhow!("WebView2 not found in registry"))
    }

    /// Fragt einen Registry-Wert ab
    fn query_registry_value(&self, key: &str, value: &str) -> Result<String> {
        let output = Command::new("reg")
            .args(&["query", key, "/v", value])
            .output()?;

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.contains(value) {
                    if let Some(result) = line.split_whitespace().last() {
                        return Ok(result.to_string());
                    }
                }
            }
        }

        Err(anyhow::anyhow!("Registry value not found"))
    }

    /// Erkennt WebView2 aus Standard-Installationspfaden
    fn detect_from_standard_paths(&self) -> Result<WebView2EnvironmentInfo> {
        println!("🔍 Checking standard installation paths...");
        
        let standard_paths = vec![
            r"C:\Program Files (x86)\Microsoft\EdgeWebView\Application",
            r"C:\Program Files\Microsoft\EdgeWebView\Application",
            r"C:\Program Files (x86)\Microsoft\Edge\Application",
            r"C:\Program Files\Microsoft\Edge\Application",
        ];

        for base_path in standard_paths {
            if let Ok(info) = self.scan_installation_directory(base_path) {
                return Ok(info);
            }
        }

        Err(anyhow::anyhow!("WebView2 not found in standard paths"))
    }

    /// Erkennt WebView2 über Edge-Installation
    fn detect_from_edge_installation(&self) -> Result<WebView2EnvironmentInfo> {
        println!("🔍 Checking Microsoft Edge installation...");
        
        // Edge kann WebView2 bereitstellen
        let edge_paths = vec![
            r"C:\Program Files (x86)\Microsoft\Edge\Application",
            r"C:\Program Files\Microsoft\Edge\Application",
        ];

        for path in edge_paths {
            if Path::new(path).exists() {
                if let Ok(version) = self.get_version_from_path(path) {
                    return Ok(WebView2EnvironmentInfo {
                        version,
                        installation_path: path.to_string(),
                        is_available: true,
                        runtime_type: WebView2RuntimeType::EdgeStable,
                        channel: WebView2Channel::Stable,
                        architecture: self.detect_architecture(),
                        installation_date: None,
                        additional_info: std::collections::HashMap::new(),
                    });
                }
            }
        }

        Err(anyhow::anyhow!("Edge installation not found"))
    }

    /// Scannt ein Installationsverzeichnis
    fn scan_installation_directory(&self, base_path: &str) -> Result<WebView2EnvironmentInfo> {
        let path = Path::new(base_path);
        if !path.exists() {
            return Err(anyhow::anyhow!("Path does not exist: {}", base_path));
        }

        // Suche nach Versionsdirektorien
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    let dir_name = entry.file_name().to_string_lossy().to_string();
                    
                    // Prüfe ob es eine Versionsnummer ist
                    if self.is_version_string(&dir_name) {
                        let full_path = entry.path().to_string_lossy().to_string();
                        
                        return Ok(WebView2EnvironmentInfo {
                            version: dir_name,
                            installation_path: full_path,
                            is_available: true,
                            runtime_type: self.determine_runtime_type(base_path),
                            channel: self.determine_channel(base_path),
                            architecture: self.detect_architecture(),
                            installation_date: None,
                            additional_info: std::collections::HashMap::new(),
                        });
                    }
                }
            }
        }

        Err(anyhow::anyhow!("No version directories found"))
    }

    /// Ermittelt die Version aus einem Pfad
    fn get_version_from_path(&self, path: &str) -> Result<String> {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    let dir_name = entry.file_name().to_string_lossy().to_string();
                    if self.is_version_string(&dir_name) {
                        return Ok(dir_name);
                    }
                }
            }
        }
        
        Err(anyhow::anyhow!("Version not found"))
    }

    /// Prüft ob ein String eine Versionsnummer ist
    fn is_version_string(&self, s: &str) -> bool {
        // Regex für Versionsnummer (z.B. "120.0.2210.121")
        let parts: Vec<&str> = s.split('.').collect();
        parts.len() >= 3 && parts.iter().all(|part| part.chars().all(|c| c.is_ascii_digit()))
    }

    /// Bestimmt den Runtime-Typ basierend auf dem Pfad
    fn determine_runtime_type(&self, path: &str) -> WebView2RuntimeType {
        if path.contains("EdgeWebView") {
            WebView2RuntimeType::Evergreen
        } else if path.contains("Edge") {
            if path.contains("Canary") {
                WebView2RuntimeType::EdgeCanary
            } else if path.contains("Dev") {
                WebView2RuntimeType::EdgeDev
            } else if path.contains("Beta") {
                WebView2RuntimeType::EdgeBeta
            } else {
                WebView2RuntimeType::EdgeStable
            }
        } else {
            WebView2RuntimeType::Unknown
        }
    }

    /// Bestimmt den Channel basierend auf dem Pfad
    fn determine_channel(&self, path: &str) -> WebView2Channel {
        if path.contains("Canary") {
            WebView2Channel::Canary
        } else if path.contains("Dev") {
            WebView2Channel::Dev
        } else if path.contains("Beta") {
            WebView2Channel::Beta
        } else {
            WebView2Channel::Stable
        }
    }

    /// Erkennt die System-Architektur
    fn detect_architecture(&self) -> String {
        if cfg!(target_arch = "x86_64") {
            "x64".to_string()
        } else if cfg!(target_arch = "x86") {
            "x86".to_string()
        } else if cfg!(target_arch = "aarch64") {
            "arm64".to_string()
        } else {
            "unknown".to_string()
        }
    }

    /// Validiert eine WebView2-Installation
    fn validate_installation(&self, path: &str) -> bool {
        let path = Path::new(path);
        
        // Prüfe ob der Pfad existiert
        if !path.exists() {
            return false;
        }

        // Suche nach wichtigen WebView2-Dateien
        let required_files = vec![
            "msedgewebview2.exe",
            "EBWebView.dll",
            "msedge.dll",
        ];

        for file in required_files {
            let file_path = path.join(file);
            if file_path.exists() {
                return true; // Mindestens eine wichtige Datei gefunden
            }
        }

        false
    }

    /// Sammelt zusätzliche Informationen
    fn gather_additional_info(&self, info: &mut WebView2EnvironmentInfo) {
        // System-Informationen
        info.additional_info.insert("os_version".to_string(), self.get_os_version());
        info.additional_info.insert("cpu_architecture".to_string(), info.architecture.clone());
        
        // WebView2-spezifische Informationen
        if info.is_available {
            info.additional_info.insert("runtime_type".to_string(), info.runtime_type.to_string());
            info.additional_info.insert("channel".to_string(), info.channel.to_string());
            
            // Dateigröße der Installation
            if let Ok(size) = self.get_installation_size(&info.installation_path) {
                info.additional_info.insert("installation_size_mb".to_string(), size.to_string());
            }
        }
    }

    /// Ermittelt die OS-Version
    fn get_os_version(&self) -> String {
        if cfg!(target_os = "windows") {
            "Windows".to_string()
        } else {
            "Unknown".to_string()
        }
    }

    /// Ermittelt die Installationsgröße in MB
    fn get_installation_size(&self, path: &str) -> Result<u64> {
        let path = Path::new(path);
        if !path.exists() {
            return Err(anyhow::anyhow!("Path does not exist"));
        }

        let mut total_size = 0u64;
        
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    total_size += metadata.len();
                }
            }
        }

        Ok(total_size / (1024 * 1024)) // Konvertiere zu MB
    }

    /// Gibt eine detaillierte Diagnose zurück
    pub fn get_diagnostic_report(&mut self) -> Result<String> {
        let info = self.check_availability()?;
        
        let mut report = String::new();
        report.push_str("🌐 WEBVIEW2 ENVIRONMENT REPORT\n");
        report.push_str("===============================\n\n");
        
        report.push_str(&format!("📋 BASIC INFORMATION:\n"));
        report.push_str(&format!("• Available: {}\n", info.is_available));
        report.push_str(&format!("• Version: {}\n", info.version));
        report.push_str(&format!("• Runtime Type: {}\n", info.runtime_type));
        report.push_str(&format!("• Channel: {}\n", info.channel));
        report.push_str(&format!("• Architecture: {}\n", info.architecture));
        report.push_str(&format!("• Installation Path: {}\n", info.installation_path));
        
        if let Some(date) = &info.installation_date {
            report.push_str(&format!("• Installation Date: {}\n", date));
        }
        
        report.push_str("\n📊 ADDITIONAL INFORMATION:\n");
        for (key, value) in &info.additional_info {
            report.push_str(&format!("• {}: {}\n", key, value));
        }
        
        if !info.is_available {
            report.push_str("\n❌ TROUBLESHOOTING:\n");
            report.push_str("• Download WebView2 Runtime from: https://developer.microsoft.com/microsoft-edge/webview2/\n");
            report.push_str("• Or install Microsoft Edge (includes WebView2)\n");
            report.push_str("• Check Windows Update for latest versions\n");
        }
        
        Ok(report)
    }
}

impl Default for WebView2EnvironmentDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_detector_creation() {
        let detector = WebView2EnvironmentDetector::new();
        assert!(detector.cached_info.is_none());
        assert!(detector.last_check.is_none());
        assert_eq!(detector.cache_duration, 300);
    }

    #[test]
    fn test_custom_cache_duration() {
        let detector = WebView2EnvironmentDetector::with_cache_duration(600);
        assert_eq!(detector.cache_duration, 600);
    }

    #[test]
    fn test_version_string_validation() {
        let detector = WebView2EnvironmentDetector::new();
        
        assert!(detector.is_version_string("120.0.2210.121"));
        assert!(detector.is_version_string("1.0.0"));
        assert!(detector.is_version_string("99.99.99.99"));
        
        assert!(!detector.is_version_string("not_a_version"));
        assert!(!detector.is_version_string("1.0"));
        assert!(!detector.is_version_string("1.0.0.a"));
    }

    #[test]
    fn test_runtime_type_determination() {
        let detector = WebView2EnvironmentDetector::new();
        
        assert_eq!(
            detector.determine_runtime_type(r"C:\Program Files\Microsoft\EdgeWebView\Application"),
            WebView2RuntimeType::Evergreen
        );
        
        assert_eq!(
            detector.determine_runtime_type(r"C:\Program Files\Microsoft\Edge\Application"),
            WebView2RuntimeType::EdgeStable
        );
        
        assert_eq!(
            detector.determine_runtime_type(r"C:\Program Files\Microsoft\Edge Dev\Application"),
            WebView2RuntimeType::EdgeDev
        );
    }

    #[test]
    fn test_channel_determination() {
        let detector = WebView2EnvironmentDetector::new();
        
        assert_eq!(
            detector.determine_channel(r"C:\Program Files\Microsoft\Edge Canary\Application"),
            WebView2Channel::Canary
        );
        
        assert_eq!(
            detector.determine_channel(r"C:\Program Files\Microsoft\Edge Dev\Application"),
            WebView2Channel::Dev
        );
        
        assert_eq!(
            detector.determine_channel(r"C:\Program Files\Microsoft\Edge Beta\Application"),
            WebView2Channel::Beta
        );
        
        assert_eq!(
            detector.determine_channel(r"C:\Program Files\Microsoft\Edge\Application"),
            WebView2Channel::Stable
        );
    }

    #[test]
    fn test_architecture_detection() {
        let detector = WebView2EnvironmentDetector::new();
        let arch = detector.detect_architecture();
        
        // Sollte eine gültige Architektur zurückgeben
        assert!(matches!(arch.as_str(), "x64" | "x86" | "arm64" | "unknown"));
    }

    #[test]
    fn test_environment_info_display() {
        let runtime_type = WebView2RuntimeType::Evergreen;
        let channel = WebView2Channel::Stable;
        
        assert_eq!(runtime_type.to_string(), "Evergreen");
        assert_eq!(channel.to_string(), "Stable");
    }

    #[test]
    fn test_environment_info_equality() {
        let info1 = WebView2EnvironmentInfo {
            version: "120.0.0.0".to_string(),
            installation_path: "test_path".to_string(),
            is_available: true,
            runtime_type: WebView2RuntimeType::Evergreen,
            channel: WebView2Channel::Stable,
            architecture: "x64".to_string(),
            installation_date: None,
            additional_info: std::collections::HashMap::new(),
        };

        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }
}
