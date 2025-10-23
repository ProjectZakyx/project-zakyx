// 🌐 Cross-Platform Manager für ZAKYX Browser
// Zentrale Verwaltung aller Platform-spezifischen Funktionen

use anyhow::Result;
use crate::platform::types::{Platform, PlatformStack, PlatformFeatures, PlatformCapability};
use std::collections::HashMap;

// Dummy-Typen für nicht-aktive Plattformen
#[cfg(not(target_os = "windows"))]
struct WindowsSupport;

#[cfg(not(target_os = "linux"))]
struct LinuxSupport;

#[cfg(not(target_os = "macos"))]
struct MacOSSupport;

#[cfg(target_os = "windows")]
use crate::platform::windows::WindowsSupport;

#[cfg(target_os = "linux")]
use crate::platform::linux::LinuxSupport;

#[cfg(target_os = "macos")]
use crate::platform::macos::MacOSSupport;

/// Cross-Platform-Manager für einheitliche Platform-Abstraktion
#[derive(Debug)]
pub struct CrossPlatformManager {
    current_platform: Platform,
    platform_stack: PlatformStack,
    platform_features: PlatformFeatures,
    capabilities: Vec<PlatformCapability>,
    
    #[cfg(target_os = "windows")]
    windows_support: Option<WindowsSupport>,
    
    #[cfg(target_os = "linux")]
    linux_support: Option<LinuxSupport>,
    
    #[cfg(target_os = "macos")]
    macos_support: Option<MacOSSupport>,
}

impl CrossPlatformManager {
    /// Erstellt einen neuen Cross-Platform-Manager
    pub fn new() -> Result<Self> {
        println!("🌐 Initializing Cross-Platform Manager...");
        
        let current_platform = Platform::current();
        
        // Platform-spezifische Unterstützung initialisieren
        #[cfg(target_os = "windows")]
        let windows_support = Some(WindowsSupport::new()?);
        #[cfg(not(target_os = "windows"))]
        let windows_support: Option<WindowsSupport> = None;
        
        #[cfg(target_os = "linux")]
        let linux_support = Some(LinuxSupport::new()?);
        #[cfg(not(target_os = "linux"))]
        let _linux_support: Option<LinuxSupport> = None;
        
        #[cfg(target_os = "macos")]
        let macos_support = Some(MacOSSupport::new()?);
        #[cfg(not(target_os = "macos"))]
        let _macos_support: Option<MacOSSupport> = None;
        
        // Platform-Stack und Features erstellen
        let (platform_stack, platform_features, capabilities) = Self::create_platform_configuration(
            &current_platform,
            #[cfg(target_os = "windows")]
            &windows_support,
            #[cfg(target_os = "linux")]
            &linux_support,
            #[cfg(target_os = "macos")]
            &macos_support,
        )?;
        
        Ok(Self {
            current_platform,
            platform_stack,
            platform_features,
            capabilities,
            
            #[cfg(target_os = "windows")]
            windows_support,
            
            #[cfg(target_os = "linux")]
            linux_support,
            
            #[cfg(target_os = "macos")]
            macos_support,
        })
    }

    /// Erstellt die Platform-Konfiguration basierend auf der aktuellen Plattform
    fn create_platform_configuration(
        platform: &Platform,
        #[cfg(target_os = "windows")]
        windows_support: &Option<WindowsSupport>,
        #[cfg(target_os = "linux")]
        linux_support: &Option<LinuxSupport>,
        #[cfg(target_os = "macos")]
        macos_support: &Option<MacOSSupport>,
    ) -> Result<(PlatformStack, PlatformFeatures, Vec<PlatformCapability>)> {
        match platform {
            #[cfg(target_os = "windows")]
            Platform::Windows => {
                if let Some(support) = windows_support {
                    let stack = support.create_platform_stack();
                    let features = support.get_platform_features();
                    let capabilities = support.get_capabilities();
                    Ok((stack, features, capabilities))
                } else {
                    // Fallback für Windows ohne Support
                    let stack = PlatformStack::for_platform(Platform::Windows);
                    let features = PlatformFeatures::for_platform(platform);
                    Ok((stack, features, Vec::new()))
                }
            },
            
            #[cfg(target_os = "linux")]
            Platform::Linux => {
                if let Some(support) = linux_support {
                    let stack = support.create_platform_stack();
                    let features = support.get_platform_features();
                    let capabilities = support.get_capabilities();
                    Ok((stack, features, capabilities))
                } else {
                    // Fallback für Linux ohne Support
                    let stack = PlatformStack::for_platform(Platform::Linux);
                    let features = PlatformFeatures::for_platform(platform);
                    Ok((stack, features, Vec::new()))
                }
            },
            
            #[cfg(target_os = "macos")]
            Platform::MacOS => {
                if let Some(support) = macos_support {
                    let stack = support.create_platform_stack();
                    let features = support.get_platform_features();
                    let capabilities = support.get_capabilities();
                    Ok((stack, features, capabilities))
                } else {
                    // Fallback für macOS ohne Support
                    let stack = PlatformStack::for_platform(Platform::MacOS);
                    let features = PlatformFeatures::for_platform(platform);
                    Ok((stack, features, Vec::new()))
                }
            },
            
            // Fallback für andere Plattformen
            _ => {
                let stack = PlatformStack::for_platform(platform.clone());
                let features = PlatformFeatures::for_platform(platform);
                Ok((stack, features, Vec::new()))
            }
        }
    }

    /// Gibt die aktuelle Plattform zurück
    pub fn get_current_platform(&self) -> &Platform {
        &self.current_platform
    }

    /// Gibt den Platform-Stack zurück
    pub fn get_platform_stack(&self) -> &PlatformStack {
        &self.platform_stack
    }

    /// Gibt die Platform-Features zurück
    pub fn get_platform_features(&self) -> &PlatformFeatures {
        &self.platform_features
    }

    /// Gibt die Platform-Capabilities zurück
    pub fn get_capabilities(&self) -> &[PlatformCapability] {
        &self.capabilities
    }

    /// Prüft ob eine bestimmte Capability unterstützt wird
    pub fn has_capability(&self, capability_name: &str) -> bool {
        self.capabilities.iter()
            .any(|cap| cap.name == capability_name && cap.supported)
    }

    /// Gibt eine Capability zurück (falls vorhanden)
    pub fn get_capability(&self, capability_name: &str) -> Option<&PlatformCapability> {
        self.capabilities.iter()
            .find(|cap| cap.name == capability_name)
    }

    /// Gibt alle unterstützten Capabilities zurück
    pub fn get_supported_capabilities(&self) -> Vec<&PlatformCapability> {
        self.capabilities.iter()
            .filter(|cap| cap.supported)
            .collect()
    }

    /// Gibt alle kritischen fehlenden Capabilities zurück
    pub fn get_missing_critical_capabilities(&self) -> Vec<&PlatformCapability> {
        self.capabilities.iter()
            .filter(|cap| cap.is_critical())
            .collect()
    }

    /// Gibt detaillierte System-Informationen zurück
    pub fn get_system_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        
        // Basis-Informationen
        info.insert("Platform".to_string(), self.current_platform.display_name().to_string());
        info.insert("Web Engine".to_string(), self.platform_stack.web_engine.display_name().to_string());
        info.insert("GUI Framework".to_string(), self.platform_stack.gui_framework.display_name().to_string());
        info.insert("Platform Stack".to_string(), self.platform_stack.summary());
        
        // Platform-spezifische Informationen hinzufügen
        #[cfg(target_os = "windows")]
        if let Some(support) = &self.windows_support {
            info.extend(support.get_system_info());
        }
        
        #[cfg(target_os = "linux")]
        if let Some(support) = &self.linux_support {
            info.extend(support.get_system_info());
        }
        
        #[cfg(target_os = "macos")]
        if let Some(support) = &self.macos_support {
            info.extend(support.get_system_info());
        }
        
        // Capability-Statistiken
        let total_caps = self.capabilities.len();
        let supported_caps = self.get_supported_capabilities().len();
        let critical_missing = self.get_missing_critical_capabilities().len();
        
        info.insert("Total Capabilities".to_string(), total_caps.to_string());
        info.insert("Supported Capabilities".to_string(), supported_caps.to_string());
        info.insert("Critical Missing".to_string(), critical_missing.to_string());
        info.insert("Support Percentage".to_string(), 
            format!("{:.1}%", (supported_caps as f64 / total_caps as f64) * 100.0));
        
        info
    }

    /// Gibt Platform-spezifische Installation-Anweisungen zurück
    pub fn get_installation_guide(&self) -> Vec<String> {
        #[cfg(target_os = "windows")]
        if let Some(support) = &self.windows_support {
            return support.get_installation_guide();
        }
        
        #[cfg(target_os = "linux")]
        if let Some(support) = &self.linux_support {
            return support.get_installation_guide();
        }
        
        #[cfg(target_os = "macos")]
        if let Some(support) = &self.macos_support {
            return support.get_installation_guide();
        }
        
        // Fallback für unbekannte Plattformen
        vec![
            format!("🌐 {} INSTALLATION", self.current_platform.display_name().to_uppercase()),
            "=".repeat(30),
            "".to_string(),
            "❌ Platform not supported yet".to_string(),
            format!("• Detected Platform: {}", self.current_platform.display_name()),
            "• Please check documentation for manual installation".to_string(),
        ]
    }

    /// Gibt Platform-spezifische Entwickler-Informationen zurück
    pub fn get_developer_info(&self) -> Vec<String> {
        #[cfg(target_os = "windows")]
        if let Some(support) = &self.windows_support {
            return support.get_developer_info();
        }
        
        #[cfg(target_os = "linux")]
        if let Some(support) = &self.linux_support {
            return support.get_developer_info();
        }
        
        #[cfg(target_os = "macos")]
        if let Some(support) = &self.macos_support {
            return support.get_developer_info();
        }
        
        // Fallback für unbekannte Plattformen
        vec![
            format!("🛠️ {} DEVELOPMENT", self.current_platform.display_name().to_uppercase()),
            "=".repeat(30),
            "".to_string(),
            "❌ Development information not available".to_string(),
            format!("• Platform: {}", self.current_platform.display_name()),
            "• Please refer to general Rust documentation".to_string(),
        ]
    }

    /// Führt Platform-spezifische Optimierungen durch
    pub fn optimize_for_platform(&mut self) -> Result<()> {
        println!("⚡ Applying platform-specific optimizations...");
        
        #[cfg(target_os = "windows")]
        if let Some(support) = &mut self.windows_support {
            support.optimize_for_windows()?;
        }
        
        #[cfg(target_os = "linux")]
        if let Some(support) = &mut self.linux_support {
            support.optimize_for_linux()?;
        }
        
        #[cfg(target_os = "macos")]
        if let Some(support) = &mut self.macos_support {
            support.optimize_for_macos()?;
        }
        
        println!("✅ Platform optimizations completed");
        Ok(())
    }

    /// Prüft Platform-Kompatibilität und gibt Probleme zurück
    pub fn check_compatibility(&self) -> Vec<String> {
        #[cfg(target_os = "windows")]
        if let Some(support) = &self.windows_support {
            return support.check_compatibility();
        }
        
        #[cfg(target_os = "linux")]
        if let Some(support) = &self.linux_support {
            return support.check_compatibility();
        }
        
        #[cfg(target_os = "macos")]
        if let Some(support) = &self.macos_support {
            return support.check_compatibility();
        }
        
        // Fallback für unbekannte Plattformen
        vec![
            format!("⚠️ Platform {} not fully supported", self.current_platform.display_name()),
            "• Some features may not work correctly".to_string(),
            "• Please check for platform-specific builds".to_string(),
        ]
    }

    /// Gibt eine Platform-Übersicht zurück
    pub fn get_platform_overview(&self) -> Vec<String> {
        let mut overview = vec![
            "🌐 PLATFORM OVERVIEW".to_string(),
            "===================".to_string(),
            "".to_string(),
            format!("📋 CURRENT CONFIGURATION:"),
            format!("• Platform: {}", self.current_platform.display_name()),
            format!("• Web Engine: {}", self.platform_stack.web_engine.display_name()),
            format!("• GUI Framework: {}", self.platform_stack.gui_framework.display_name()),
            format!("• Stack: {}", self.platform_stack.summary()),
            "".to_string(),
        ];

        // Features
        let enabled_features = self.platform_features.enabled_features();
        overview.push("🚀 ENABLED FEATURES:".to_string());
        for feature in enabled_features {
            overview.push(format!("• ✅ {}", feature));
        }
        
        // Capabilities
        overview.push("".to_string());
        overview.push("🔧 CAPABILITIES:".to_string());
        let supported = self.get_supported_capabilities();
        let missing_critical = self.get_missing_critical_capabilities();
        
        overview.push(format!("• Total: {}", self.capabilities.len()));
        overview.push(format!("• Supported: {}", supported.len()));
        overview.push(format!("• Critical Missing: {}", missing_critical.len()));
        
        if !missing_critical.is_empty() {
            overview.push("".to_string());
            overview.push("❌ CRITICAL MISSING:".to_string());
            for cap in missing_critical {
                overview.push(format!("• {} - {}", cap.name, cap.description));
            }
        }

        // Kompatibilitätsstatus
        overview.push("".to_string());
        overview.push("✅ COMPATIBILITY STATUS:".to_string());
        let compatibility_issues = self.check_compatibility();
        overview.extend(compatibility_issues.into_iter().map(|issue| format!("• {}", issue)));

        overview
    }

    /// Gibt Cross-Platform-Roadmap zurück
    pub fn get_cross_platform_roadmap(&self) -> Vec<String> {
        vec![
            "🗺️ CROSS-PLATFORM ROADMAP".to_string(),
            "==========================".to_string(),
            "".to_string(),
            "✅ PHASE 1 - WINDOWS (COMPLETE):".to_string(),
            "• WebView2 Integration".to_string(),
            "• Native Windows Controls".to_string(),
            "• Performance Optimization".to_string(),
            "• Security Features".to_string(),
            "".to_string(),
            "🚧 PHASE 2 - LINUX (IN PROGRESS):".to_string(),
            "• WebKit2GTK Integration".to_string(),
            "• GTK3/GTK4 UI Components".to_string(),
            "• D-Bus System Integration".to_string(),
            "• Flatpak/Snap Packaging".to_string(),
            "".to_string(),
            "🚧 PHASE 3 - MACOS (PLANNED):".to_string(),
            "• WKWebView Integration".to_string(),
            "• Cocoa Native UI".to_string(),
            "• Mac App Store Distribution".to_string(),
            "• Universal Binary (Intel/ARM)".to_string(),
            "".to_string(),
            "🔮 PHASE 4 - MOBILE (FUTURE):".to_string(),
            "• Android WebView + Kotlin".to_string(),
            "• iOS WKWebView + Swift".to_string(),
            "• Cross-platform Rust core".to_string(),
            "• Unified API layer".to_string(),
            "".to_string(),
            "🌐 PHASE 5 - WEB (FUTURE):".to_string(),
            "• WebAssembly compilation".to_string(),
            "• Progressive Web App".to_string(),
            "• Browser extension".to_string(),
            "• Cloud synchronization".to_string(),
        ]
    }

    /// Bereinigt Platform-spezifische Ressourcen
    pub fn cleanup(&mut self) -> Result<()> {
        println!("🧹 Cleaning up platform resources...");
        
        #[cfg(target_os = "windows")]
        if let Some(support) = &mut self.windows_support {
            support.cleanup()?;
        }
        
        // Linux und macOS haben derzeit keine explizite Bereinigung
        
        println!("✅ Platform cleanup completed");
        Ok(())
    }
}

impl Default for CrossPlatformManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            let current_platform = Platform::current();
            let platform_stack = PlatformStack::for_platform(current_platform.clone());
            let platform_features = PlatformFeatures::for_platform(&current_platform);
            
            Self {
                current_platform,
                platform_stack,
                platform_features,
                capabilities: Vec::new(),
                
                #[cfg(target_os = "windows")]
                windows_support: None,
                
                #[cfg(target_os = "linux")]
                linux_support: None,
                
                #[cfg(target_os = "macos")]
                macos_support: None,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_platform_manager_creation() {
        let manager = CrossPlatformManager::new();
        assert!(manager.is_ok());
        
        let mgr = manager.unwrap();
        assert_ne!(mgr.current_platform, Platform::Unknown);
    }

    #[test]
    fn test_platform_detection() {
        let manager = CrossPlatformManager::new().unwrap();
        let platform = manager.get_current_platform();
        
        // Should detect current platform correctly
        assert_eq!(*platform, Platform::current());
    }

    #[test]
    fn test_system_info() {
        let manager = CrossPlatformManager::new().unwrap();
        let info = manager.get_system_info();
        
        assert!(info.contains_key("Platform"));
        assert!(info.contains_key("Web Engine"));
        assert!(info.contains_key("GUI Framework"));
        assert!(info.contains_key("Total Capabilities"));
    }

    #[test]
    fn test_capabilities() {
        let manager = CrossPlatformManager::new().unwrap();
        let capabilities = manager.get_capabilities();
        
        // Should have some capabilities on supported platforms
        if matches!(manager.current_platform, Platform::Windows | Platform::Linux | Platform::MacOS) {
            assert!(!capabilities.is_empty());
        }
    }

    #[test]
    fn test_platform_features() {
        let manager = CrossPlatformManager::new().unwrap();
        let features = manager.get_platform_features();
        
        // Should have reasonable defaults
        assert!(features.enabled_count() > 0);
    }

    #[test]
    fn test_installation_guide() {
        let manager = CrossPlatformManager::new().unwrap();
        let guide = manager.get_installation_guide();
        
        assert!(!guide.is_empty());
        assert!(guide.iter().any(|line| line.contains("INSTALLATION")));
    }

    #[test]
    fn test_compatibility_check() {
        let manager = CrossPlatformManager::new().unwrap();
        let compatibility = manager.check_compatibility();
        
        assert!(!compatibility.is_empty());
        // Should have at least one message (success or warning)
    }

    #[test]
    fn test_platform_overview() {
        let manager = CrossPlatformManager::new().unwrap();
        let overview = manager.get_platform_overview();
        
        assert!(!overview.is_empty());
        assert!(overview.iter().any(|line| line.contains("PLATFORM OVERVIEW")));
    }

    #[test]
    fn test_roadmap() {
        let manager = CrossPlatformManager::new().unwrap();
        let roadmap = manager.get_cross_platform_roadmap();
        
        assert!(!roadmap.is_empty());
        assert!(roadmap.iter().any(|line| line.contains("ROADMAP")));
    }

    #[test]
    fn test_capability_queries() {
        let manager = CrossPlatformManager::new().unwrap();
        
        // Test capability existence check
        let has_any = manager.get_capabilities().iter().any(|cap| cap.supported);
        
        if has_any {
            let supported = manager.get_supported_capabilities();
            assert!(!supported.is_empty());
        }
        
        // Test missing critical capabilities
        let missing = manager.get_missing_critical_capabilities();
        // This could be empty or non-empty depending on platform
    }
}