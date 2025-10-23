// 🌐 PLATFORM EXTENSION - LEGACY FILE
// Diese Datei bleibt für Rückwärtskompatibilität
// Verwende stattdessen src/platform/ für neue Entwicklungen

use anyhow::Result;
use windows::Win32::Foundation::HWND;

// Re-export der neuen modularen Struktur
pub use crate::platform::{
    Platform,
    WebEngine, 
    GUIFramework,
    PlatformCapability,
    PlatformStack,
    PlatformFeatures,
    CrossPlatformManager,
    PlatformExtensionManager as NewPlatformExtensionManager,
};

#[cfg(target_os = "windows")]
pub use crate::platform::{WindowsSupport, WindowsVersion, WindowsFeatures};

#[cfg(target_os = "linux")]
pub use crate::platform::{LinuxSupport, LinuxDistribution, DesktopEnvironment, LinuxFeatures};

#[cfg(target_os = "macos")]
pub use crate::platform::{MacOSSupport, MacOSVersion, MacOSFeatures};

/// Legacy Platform Extension Manager (deprecated - use crate::platform::PlatformExtensionManager)
#[deprecated(note = "Use crate::platform::PlatformExtensionManager instead")]
pub struct PlatformExtensionManager {
    inner: NewPlatformExtensionManager,
}

impl PlatformExtensionManager {
    /// Erstellt einen neuen Platform Extension Manager (Legacy-Wrapper)
    pub fn new(window_handle: HWND) -> Result<Self> {
        println!("⚠️ Using legacy PlatformExtensionManager - consider migrating to crate::platform::PlatformExtensionManager");
        
        let inner = NewPlatformExtensionManager::new(window_handle)?;
        
        Ok(Self { inner })
    }

    /// Gibt Platform-Übersicht zurück (Legacy-Wrapper)
    pub fn get_platform_overview(&self) -> Vec<String> {
        self.inner.get_platform_overview()
    }
}

impl Default for PlatformExtensionManager {
    fn default() -> Self {
        Self::new(HWND(std::ptr::null_mut()))
            .unwrap_or_else(|_| Self {
                inner: NewPlatformExtensionManager::default(),
            })
    }
}