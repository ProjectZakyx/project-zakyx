// 🌐 Platform Module für ZAKYX Browser
// Cross-Platform-Unterstützung für Windows, Linux, macOS und mehr

#![allow(dead_code)] // Platform system API - comprehensive API kept for extensibility

// #[cfg(target_os = "windows")]
// use windows::Win32::Foundation::HWND;

// Temporärer Fallback für HWND
#[cfg(target_os = "windows")]
type HWND = *mut std::ffi::c_void;

#[cfg(not(target_os = "windows"))]
type HWND = u64;

pub mod types;
pub mod manager;

// Platform-spezifische Module (nur kompiliert für entsprechende Targets)
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

// Re-exports für einfache Verwendung
pub use types::{
    Platform,
    WebEngine,
    GUIFramework,
    PlatformCapability,
    PlatformStack,
    PlatformFeatures,
};

pub use manager::CrossPlatformManager;

// Platform-spezifische Re-exports
#[cfg(target_os = "windows")]
pub use windows::{WindowsSupport, WindowsVersion, WindowsFeatures};

#[cfg(target_os = "linux")]
pub use linux::{LinuxSupport, LinuxDistribution, DesktopEnvironment, LinuxFeatures};

#[cfg(target_os = "macos")]
pub use macos::{MacOSSupport, MacOSVersion, MacOSFeatures};

/// Hauptklasse für Platform-Management
/// 
/// Diese Klasse bietet eine einheitliche Schnittstelle für alle Platform-spezifischen
/// Funktionen und abstrahiert die Unterschiede zwischen verschiedenen Betriebssystemen.
/// 
/// # Beispiel
/// 
/// ```rust
/// use crate::platform::PlatformExtensionManager;
/// 
/// let manager = PlatformExtensionManager::new(hwnd)?;
/// let info = manager.get_platform_info();
/// println!("Platform: {}", info.join("\n"));
/// ```
#[derive(Debug)]
pub struct PlatformExtensionManager {
    cross_platform_manager: CrossPlatformManager,
    
    #[cfg(target_os = "windows")]
    window_handle: HWND,
    
    #[cfg(not(target_os = "windows"))]
    window_handle: u64, // Placeholder für andere Plattformen
}

impl PlatformExtensionManager {
    /// Erstellt einen neuen Platform Extension Manager
    /// 
    /// # Parameter
    /// - `window_handle`: Platform-spezifisches Window Handle
    /// 
    /// # Fehler
    /// Gibt einen Fehler zurück, wenn die Platform-Initialisierung fehlschlägt
    #[cfg(target_os = "windows")]
    pub fn new(window_handle: HWND) -> anyhow::Result<Self> {
        println!("🌐 Initializing Platform Extension Manager...");
        
        let cross_platform_manager = CrossPlatformManager::new()?;
        
        Ok(Self {
            cross_platform_manager,
            window_handle,
        })
    }
    
    /// Erstellt einen neuen Platform Extension Manager (Non-Windows)
    #[cfg(not(target_os = "windows"))]
    pub fn new(window_handle: u64) -> anyhow::Result<Self> {
        println!("🌐 Initializing Platform Extension Manager...");
        
        let cross_platform_manager = CrossPlatformManager::new()?;
        
        Ok(Self {
            cross_platform_manager,
            window_handle,
        })
    }

    /// Gibt Platform-Informationen zurück
    pub fn get_platform_info(&self) -> Vec<String> {
        let mut info = vec![
            "🌐 PLATFORM INFORMATION".to_string(),
            "=======================".to_string(),
            "".to_string(),
        ];
        
        let system_info = self.cross_platform_manager.get_system_info();
        
        info.push(format!("Current Platform: {}", system_info.get("Platform").unwrap_or(&"Unknown".to_string())));
        info.push(format!("Web Engine: {}", system_info.get("Web Engine").unwrap_or(&"Unknown".to_string())));
        info.push(format!("GUI Framework: {}", system_info.get("GUI Framework").unwrap_or(&"Unknown".to_string())));
        info.push("".to_string());
        
        info.push("🔧 SUPPORTED PLATFORMS:".to_string());
        info.push("• ✅ Windows 10/11 (WebView2 + WinAPI)".to_string());
        info.push("• 🚧 Linux (WebKit2GTK + GTK3)".to_string());
        info.push("• 🚧 macOS (WKWebView + Cocoa)".to_string());
        info.push("".to_string());
        
        info.push("🎯 FUTURE TARGETS:".to_string());
        info.push("• 📱 Android (WebView + Native UI)".to_string());
        info.push("• 📱 iOS (WKWebView + UIKit)".to_string());
        info.push("• 🌐 Web Assembly (Browser)".to_string());
        info.push("• 📺 Smart TV Platforms".to_string());
        
        info
    }

    /// Gibt Cross-Platform-Roadmap zurück
    pub fn get_cross_platform_roadmap(&self) -> Vec<String> {
        self.cross_platform_manager.get_cross_platform_roadmap()
    }

    /// Gibt Linux-Installation-Guide zurück
    pub fn get_linux_guide(&self) -> Vec<String> {
        #[cfg(target_os = "linux")]
        {
            self.cross_platform_manager.get_installation_guide()
        }
        #[cfg(not(target_os = "linux"))]
        {
            vec![
                "🐧 LINUX INSTALLATION".to_string(),
                "=====================".to_string(),
                "".to_string(),
                "📦 UBUNTU/DEBIAN:".to_string(),
                "sudo apt update".to_string(),
                "sudo apt install webkit2gtk-4.0-dev".to_string(),
                "sudo apt install libgtk-3-dev".to_string(),
                "sudo apt install pkg-config".to_string(),
                "".to_string(),
                "📦 FEDORA/RHEL:".to_string(),
                "sudo dnf install webkit2gtk3-devel".to_string(),
                "sudo dnf install gtk3-devel".to_string(),
                "sudo dnf install pkgconf-pkg-config".to_string(),
                "".to_string(),
                "📦 ARCH LINUX:".to_string(),
                "sudo pacman -S webkit2gtk".to_string(),
                "sudo pacman -S gtk3".to_string(),
                "sudo pacman -S pkgconf".to_string(),
                "".to_string(),
                "🔧 BUILD COMMAND:".to_string(),
                "cargo build --features=linux".to_string(),
            ]
        }
    }

    /// Gibt macOS-Installation-Guide zurück
    pub fn get_macos_guide(&self) -> Vec<String> {
        #[cfg(target_os = "macos")]
        {
            self.cross_platform_manager.get_installation_guide()
        }
        #[cfg(not(target_os = "macos"))]
        {
            vec![
                "🍎 MACOS INSTALLATION".to_string(),
                "=====================".to_string(),
                "".to_string(),
                "🛠️ PREREQUISITES:".to_string(),
                "• Xcode Command Line Tools".to_string(),
                "• macOS 10.15+ (Catalina or later)".to_string(),
                "• Rust toolchain".to_string(),
                "".to_string(),
                "📦 INSTALL XCODE TOOLS:".to_string(),
                "xcode-select --install".to_string(),
                "".to_string(),
                "🔧 BUILD COMMAND:".to_string(),
                "cargo build --features=macos".to_string(),
                "".to_string(),
                "📱 APP BUNDLE:".to_string(),
                "cargo bundle --features=macos".to_string(),
                "".to_string(),
                "🏪 MAC APP STORE:".to_string(),
                "• Code signing required".to_string(),
                "• App Sandbox enabled".to_string(),
                "• Notarization for distribution".to_string(),
            ]
        }
    }

    /// Gibt Platform-Übersicht zurück
    pub fn get_platform_overview(&self) -> Vec<String> {
        let mut overview = vec![
            "🌐 PLATFORM EXTENSION".to_string(),
            "=====================".to_string(),
            "".to_string(),
            "💬 VERFÜGBARE KOMMANDOS:".to_string(),
            "• 'platform info' → Platform Information".to_string(),
            "• 'platform linux' → Linux Installation".to_string(),
            "• 'platform macos' → macOS Installation".to_string(),
            "• 'platform roadmap' → Development Roadmap".to_string(),
            "".to_string(),
            "🌐 CROSS-PLATFORM FEATURES:".to_string(),
        ];
        
        let platform = self.cross_platform_manager.get_current_platform();
        match platform {
            Platform::Windows => {
                overview.push("• 🪟 Windows 10/11 Support".to_string());
                overview.push("• ✅ WebView2 Integration".to_string());
                overview.push("• ✅ Native Windows Controls".to_string());
                overview.push("• ✅ Aero Glass Effects".to_string());
            },
            Platform::Linux => {
                overview.push("• 🐧 Linux Distribution Support".to_string());
                overview.push("• 🚧 WebKit2GTK Integration".to_string());
                overview.push("• 🚧 GTK Native UI".to_string());
                overview.push("• 🚧 D-Bus System Integration".to_string());
            },
            Platform::MacOS => {
                overview.push("• 🍎 macOS Support".to_string());
                overview.push("• 🚧 WKWebView Integration".to_string());
                overview.push("• 🚧 Cocoa Native UI".to_string());
                overview.push("• 🚧 Metal Hardware Acceleration".to_string());
            },
            _ => {
                overview.push("• ❓ Platform Detection".to_string());
                overview.push("• 🚧 Cross-Platform Fallback".to_string());
            }
        }
        
        overview.push("".to_string());
        overview.push("🎯 DEVELOPMENT STATUS:".to_string());
        overview.push("• ✅ Core Architecture Complete".to_string());
        overview.push("• ✅ Windows Implementation".to_string());
        overview.push("• 🚧 Linux Implementation (In Progress)".to_string());
        overview.push("• 🚧 macOS Implementation (Planned)".to_string());
        overview.push("• 🔮 Mobile Support (Future)".to_string());
        
        overview
    }

    /// Führt Platform-spezifische Optimierungen durch
    pub fn optimize_for_platform(&mut self) -> anyhow::Result<()> {
        self.cross_platform_manager.optimize_for_platform()
    }

    /// Prüft Platform-Kompatibilität
    pub fn check_compatibility(&self) -> Vec<String> {
        self.cross_platform_manager.check_compatibility()
    }

    /// Gibt detaillierte System-Informationen zurück
    pub fn get_detailed_system_info(&self) -> std::collections::HashMap<String, String> {
        self.cross_platform_manager.get_system_info()
    }

    /// Gibt Platform-Capabilities zurück
    pub fn get_capabilities(&self) -> &[PlatformCapability] {
        self.cross_platform_manager.get_capabilities()
    }

    /// Prüft ob eine bestimmte Capability unterstützt wird
    pub fn has_capability(&self, capability_name: &str) -> bool {
        self.cross_platform_manager.has_capability(capability_name)
    }

    /// Gibt den Cross-Platform-Manager zurück (für erweiterte Funktionen)
    pub fn get_cross_platform_manager(&self) -> &CrossPlatformManager {
        &self.cross_platform_manager
    }

    /// Gibt den Cross-Platform-Manager zurück (mutable, für erweiterte Funktionen)
    pub fn get_cross_platform_manager_mut(&mut self) -> &mut CrossPlatformManager {
        &mut self.cross_platform_manager
    }

    /// Bereinigt Platform-spezifische Ressourcen
    pub fn cleanup(&mut self) -> anyhow::Result<()> {
        self.cross_platform_manager.cleanup()
    }
}

impl Default for PlatformExtensionManager {
    fn default() -> Self {
        #[cfg(target_os = "windows")]
        {
            Self::new(std::ptr::null_mut())
                .unwrap_or_else(|_| Self {
                    cross_platform_manager: CrossPlatformManager::default(),
                    window_handle: std::ptr::null_mut(),
                })
        }
        #[cfg(not(target_os = "windows"))]
        {
            Self::new(0)
                .unwrap_or_else(|_| Self {
                    cross_platform_manager: CrossPlatformManager::default(),
                    window_handle: 0,
                })
        }
    }
}

/// Utility-Funktionen für Platform-Detection und -Management
pub mod utils {
    use super::*;
    
    /// Erkennt die aktuelle Plattform
    pub fn detect_platform() -> Platform {
        Platform::current()
    }
    
    /// Prüft ob die aktuelle Plattform unterstützt wird
    pub fn is_platform_supported() -> bool {
        matches!(Platform::current(), Platform::Windows | Platform::Linux | Platform::MacOS)
    }
    
    /// Gibt eine kurze Platform-Beschreibung zurück
    pub fn get_platform_summary() -> String {
        let platform = Platform::current();
        let web_engine = WebEngine::recommended_for_platform(&platform);
        let gui_framework = GUIFramework::recommended_for_platform(&platform);
        
        format!("{} + {} + {}", 
            platform.display_name(),
            web_engine.display_name(),
            gui_framework.display_name()
        )
    }
    
    /// Erstellt einen minimalen Platform-Manager für Quick-Tests
    pub fn create_minimal_manager() -> anyhow::Result<PlatformExtensionManager> {
        #[cfg(target_os = "windows")]
        {
            PlatformExtensionManager::new(std::ptr::null_mut())
        }
        #[cfg(not(target_os = "windows"))]
        {
            PlatformExtensionManager::new(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let platform = utils::detect_platform();
        assert_ne!(platform, Platform::Unknown);
        
        let is_supported = utils::is_platform_supported();
        assert!(is_supported); // Should be true on Windows/Linux/macOS
    }

    #[test]
    fn test_platform_summary() {
        let summary = utils::get_platform_summary();
        assert!(!summary.is_empty());
        assert!(summary.contains("+"));
    }

    #[test]
    fn test_minimal_manager_creation() {
        let manager = utils::create_minimal_manager();
        assert!(manager.is_ok());
        
        let mgr = manager.unwrap();
        let info = mgr.get_platform_info();
        assert!(!info.is_empty());
    }

    #[test]
    fn test_platform_extension_manager() {
        let manager = utils::create_minimal_manager().unwrap();
        
        // Test basic functionality
        let info = manager.get_platform_info();
        assert!(info.iter().any(|line| line.contains("PLATFORM INFORMATION")));
        
        let overview = manager.get_platform_overview();
        assert!(overview.iter().any(|line| line.contains("PLATFORM EXTENSION")));
        
        let roadmap = manager.get_cross_platform_roadmap();
        assert!(roadmap.iter().any(|line| line.contains("ROADMAP")));
        
        let linux_guide = manager.get_linux_guide();
        assert!(linux_guide.iter().any(|line| line.contains("LINUX INSTALLATION")));
        
        let macos_guide = manager.get_macos_guide();
        assert!(macos_guide.iter().any(|line| line.contains("MACOS INSTALLATION")));
    }

    #[test]
    fn test_capabilities() {
        let manager = utils::create_minimal_manager().unwrap();
        let capabilities = manager.get_capabilities();
        
        // Should have some capabilities on supported platforms
        if utils::is_platform_supported() {
            assert!(!capabilities.is_empty());
        }
        
        // Test capability queries
        for cap in capabilities {
            let has_cap = manager.has_capability(&cap.name);
            assert_eq!(has_cap, cap.supported);
        }
    }

    #[test]
    fn test_compatibility_check() {
        let manager = utils::create_minimal_manager().unwrap();
        let compatibility = manager.check_compatibility();
        
        assert!(!compatibility.is_empty());
        // Should have at least one message
    }

    #[test]
    fn test_system_info() {
        let manager = utils::create_minimal_manager().unwrap();
        let info = manager.get_detailed_system_info();
        
        assert!(info.contains_key("Platform"));
        assert!(info.contains_key("Web Engine"));
        assert!(info.contains_key("GUI Framework"));
    }
}