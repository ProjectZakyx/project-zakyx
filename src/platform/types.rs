// 🌐 Platform Types für ZAKYX Browser
// Gemeinsame Datenstrukturen für Cross-Platform-Support

use serde::{Deserialize, Serialize};

/// Unterstützte Plattformen
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Platform {
    Windows,
    Linux,
    MacOS,
    Android,
    IOS,
    WebAssembly,
    Unknown,
}

impl Platform {
    /// Erkennt die aktuelle Plattform zur Compile-Zeit
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        return Platform::Windows;

        #[cfg(target_os = "linux")]
        return Platform::Linux;

        #[cfg(target_os = "macos")]
        return Platform::MacOS;

        #[cfg(target_os = "android")]
        return Platform::Android;

        #[cfg(target_os = "ios")]
        return Platform::IOS;

        #[cfg(target_arch = "wasm32")]
        return Platform::WebAssembly;

        #[cfg(not(any(
            target_os = "windows",
            target_os = "linux", 
            target_os = "macos",
            target_os = "android",
            target_os = "ios",
            target_arch = "wasm32"
        )))]
        return Platform::Unknown;
    }

    /// Gibt den Display-Namen der Plattform zurück
    pub fn display_name(&self) -> &'static str {
        match self {
            Platform::Windows => "Windows",
            Platform::Linux => "Linux",
            Platform::MacOS => "macOS",
            Platform::Android => "Android",
            Platform::IOS => "iOS",
            Platform::WebAssembly => "WebAssembly",
            Platform::Unknown => "Unknown",
        }
    }

    /// Prüft ob die Plattform Desktop ist
    pub fn is_desktop(&self) -> bool {
        matches!(self, Platform::Windows | Platform::Linux | Platform::MacOS)
    }

    /// Prüft ob die Plattform Mobile ist
    pub fn is_mobile(&self) -> bool {
        matches!(self, Platform::Android | Platform::IOS)
    }

    /// Prüft ob die Plattform Web-basiert ist
    pub fn is_web(&self) -> bool {
        matches!(self, Platform::WebAssembly)
    }
}

/// Unterstützte Web-Engines
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WebEngine {
    /// Microsoft WebView2 (Windows)
    WebView2,
    /// WebKit2GTK (Linux)
    WebKit2GTK,
    /// WKWebView (macOS/iOS)
    WKWebView,
    /// Android WebView
    AndroidWebView,
    /// Browser Engine (WebAssembly)
    BrowserEngine,
    /// Fallback/Unknown
    Unknown,
}

impl WebEngine {
    /// Gibt die empfohlene Web-Engine für eine Plattform zurück
    pub fn recommended_for_platform(platform: &Platform) -> Self {
        match platform {
            Platform::Windows => WebEngine::WebView2,
            Platform::Linux => WebEngine::WebKit2GTK,
            Platform::MacOS | Platform::IOS => WebEngine::WKWebView,
            Platform::Android => WebEngine::AndroidWebView,
            Platform::WebAssembly => WebEngine::BrowserEngine,
            Platform::Unknown => WebEngine::Unknown,
        }
    }

    /// Gibt den Display-Namen der Web-Engine zurück
    pub fn display_name(&self) -> &'static str {
        match self {
            WebEngine::WebView2 => "Microsoft WebView2",
            WebEngine::WebKit2GTK => "WebKit2GTK",
            WebEngine::WKWebView => "WKWebView",
            WebEngine::AndroidWebView => "Android WebView",
            WebEngine::BrowserEngine => "Browser Engine",
            WebEngine::Unknown => "Unknown",
        }
    }

    /// Gibt die Version der Web-Engine zurück (falls verfügbar)
    pub fn version(&self) -> Option<&'static str> {
        match self {
            WebEngine::WebView2 => Some("1.0.2592.51"),
            WebEngine::WebKit2GTK => Some("2.40.0"),
            WebEngine::WKWebView => Some("18.0"),
            WebEngine::AndroidWebView => Some("120.0"),
            WebEngine::BrowserEngine => None,
            WebEngine::Unknown => None,
        }
    }
}

/// Unterstützte GUI-Frameworks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GUIFramework {
    /// Windows API (Win32)
    WinAPI,
    /// GTK (Linux)
    GTK,
    /// Cocoa (macOS)
    Cocoa,
    /// UIKit (iOS)
    UIKit,
    /// Android Native UI
    AndroidUI,
    /// Cross-Platform (Tauri, Egui, etc.)
    CrossPlatform,
    /// Web-based UI
    WebUI,
}

impl GUIFramework {
    /// Gibt das empfohlene GUI-Framework für eine Plattform zurück
    pub fn recommended_for_platform(platform: &Platform) -> Self {
        match platform {
            Platform::Windows => GUIFramework::WinAPI,
            Platform::Linux => GUIFramework::GTK,
            Platform::MacOS => GUIFramework::Cocoa,
            Platform::IOS => GUIFramework::UIKit,
            Platform::Android => GUIFramework::AndroidUI,
            Platform::WebAssembly => GUIFramework::WebUI,
            Platform::Unknown => GUIFramework::CrossPlatform,
        }
    }

    /// Gibt den Display-Namen des GUI-Frameworks zurück
    pub fn display_name(&self) -> &'static str {
        match self {
            GUIFramework::WinAPI => "Windows API",
            GUIFramework::GTK => "GTK",
            GUIFramework::Cocoa => "Cocoa",
            GUIFramework::UIKit => "UIKit",
            GUIFramework::AndroidUI => "Android UI",
            GUIFramework::CrossPlatform => "Cross-Platform",
            GUIFramework::WebUI => "Web UI",
        }
    }
}

/// Platform-Capability-Beschreibung
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformCapability {
    pub name: String,
    pub supported: bool,
    pub version: Option<String>,
    pub description: String,
    pub required: bool,
    pub optional_features: Vec<String>,
}

impl PlatformCapability {
    /// Erstellt eine neue Platform-Capability
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            supported: false,
            version: None,
            description,
            required: false,
            optional_features: Vec::new(),
        }
    }

    /// Setzt die Capability als unterstützt
    pub fn with_support(mut self, supported: bool) -> Self {
        self.supported = supported;
        self
    }

    /// Setzt die Version der Capability
    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    /// Markiert die Capability als erforderlich
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Fügt optionale Features hinzu
    pub fn with_optional_features(mut self, features: Vec<String>) -> Self {
        self.optional_features = features;
        self
    }

    /// Prüft ob die Capability verfügbar ist
    pub fn is_available(&self) -> bool {
        self.supported
    }

    /// Prüft ob die Capability kritisch ist
    pub fn is_critical(&self) -> bool {
        self.required && !self.supported
    }
}

/// Platform-Stack-Konfiguration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformStack {
    pub platform: Platform,
    pub web_engine: WebEngine,
    pub gui_framework: GUIFramework,
    pub capabilities: Vec<PlatformCapability>,
}

impl PlatformStack {
    /// Erstellt einen optimalen Platform-Stack für die aktuelle Plattform
    pub fn optimal() -> Self {
        let platform = Platform::current();
        let web_engine = WebEngine::recommended_for_platform(&platform);
        let gui_framework = GUIFramework::recommended_for_platform(&platform);

        Self {
            platform,
            web_engine,
            gui_framework,
            capabilities: Vec::new(),
        }
    }

    /// Erstellt einen Platform-Stack für eine spezifische Plattform
    pub fn for_platform(platform: Platform) -> Self {
        let web_engine = WebEngine::recommended_for_platform(&platform);
        let gui_framework = GUIFramework::recommended_for_platform(&platform);

        Self {
            platform,
            web_engine,
            gui_framework,
            capabilities: Vec::new(),
        }
    }

    /// Fügt Capabilities hinzu
    pub fn with_capabilities(mut self, capabilities: Vec<PlatformCapability>) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Prüft ob der Stack vollständig unterstützt wird
    pub fn is_fully_supported(&self) -> bool {
        self.capabilities.iter().all(|cap| !cap.is_critical())
    }

    /// Gibt kritische fehlende Capabilities zurück
    pub fn missing_critical_capabilities(&self) -> Vec<&PlatformCapability> {
        self.capabilities.iter().filter(|cap| cap.is_critical()).collect()
    }

    /// Gibt eine Zusammenfassung des Stacks zurück
    pub fn summary(&self) -> String {
        format!(
            "{} + {} + {}",
            self.platform.display_name(),
            self.web_engine.display_name(),
            self.gui_framework.display_name()
        )
    }
}

/// Platform-Feature-Flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformFeatures {
    pub native_notifications: bool,
    pub system_tray: bool,
    pub global_shortcuts: bool,
    pub file_associations: bool,
    pub auto_updater: bool,
    pub window_controls: bool,
    pub transparency: bool,
    pub hardware_acceleration: bool,
    pub multi_window: bool,
    pub clipboard_access: bool,
}

impl Default for PlatformFeatures {
    fn default() -> Self {
        Self {
            native_notifications: true,
            system_tray: true,
            global_shortcuts: false,
            file_associations: false,
            auto_updater: false,
            window_controls: true,
            transparency: false,
            hardware_acceleration: true,
            multi_window: true,
            clipboard_access: true,
        }
    }
}

impl PlatformFeatures {
    /// Erstellt Platform-Features für eine spezifische Plattform
    pub fn for_platform(platform: &Platform) -> Self {
        match platform {
            Platform::Windows => Self {
                native_notifications: true,
                system_tray: true,
                global_shortcuts: true,
                file_associations: true,
                auto_updater: true,
                window_controls: true,
                transparency: true,
                hardware_acceleration: true,
                multi_window: true,
                clipboard_access: true,
            },
            Platform::Linux => Self {
                native_notifications: true,
                system_tray: true,
                global_shortcuts: false,
                file_associations: true,
                auto_updater: false,
                window_controls: true,
                transparency: false,
                hardware_acceleration: true,
                multi_window: true,
                clipboard_access: true,
            },
            Platform::MacOS => Self {
                native_notifications: true,
                system_tray: false,
                global_shortcuts: true,
                file_associations: true,
                auto_updater: true,
                window_controls: true,
                transparency: true,
                hardware_acceleration: true,
                multi_window: true,
                clipboard_access: true,
            },
            _ => Self::default(),
        }
    }

    /// Gibt eine Liste der aktivierten Features zurück
    pub fn enabled_features(&self) -> Vec<&'static str> {
        let mut features = Vec::new();
        
        if self.native_notifications { features.push("Native Notifications"); }
        if self.system_tray { features.push("System Tray"); }
        if self.global_shortcuts { features.push("Global Shortcuts"); }
        if self.file_associations { features.push("File Associations"); }
        if self.auto_updater { features.push("Auto Updater"); }
        if self.window_controls { features.push("Window Controls"); }
        if self.transparency { features.push("Transparency"); }
        if self.hardware_acceleration { features.push("Hardware Acceleration"); }
        if self.multi_window { features.push("Multi-Window"); }
        if self.clipboard_access { features.push("Clipboard Access"); }
        
        features
    }

    /// Zählt die Anzahl der aktivierten Features
    pub fn enabled_count(&self) -> usize {
        self.enabled_features().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let platform = Platform::current();
        assert_ne!(platform, Platform::Unknown);
        
        // Test platform properties
        if platform == Platform::Windows {
            assert!(platform.is_desktop());
            assert!(!platform.is_mobile());
            assert!(!platform.is_web());
        }
    }

    #[test]
    fn test_web_engine_recommendation() {
        let windows_engine = WebEngine::recommended_for_platform(&Platform::Windows);
        assert_eq!(windows_engine, WebEngine::WebView2);
        
        let linux_engine = WebEngine::recommended_for_platform(&Platform::Linux);
        assert_eq!(linux_engine, WebEngine::WebKit2GTK);
        
        let macos_engine = WebEngine::recommended_for_platform(&Platform::MacOS);
        assert_eq!(macos_engine, WebEngine::WKWebView);
    }

    #[test]
    fn test_gui_framework_recommendation() {
        let windows_gui = GUIFramework::recommended_for_platform(&Platform::Windows);
        assert_eq!(windows_gui, GUIFramework::WinAPI);
        
        let linux_gui = GUIFramework::recommended_for_platform(&Platform::Linux);
        assert_eq!(linux_gui, GUIFramework::GTK);
        
        let macos_gui = GUIFramework::recommended_for_platform(&Platform::MacOS);
        assert_eq!(macos_gui, GUIFramework::Cocoa);
    }

    #[test]
    fn test_platform_capability() {
        let mut capability = PlatformCapability::new(
            "WebView2".to_string(),
            "Microsoft WebView2 Runtime".to_string()
        );
        
        assert!(!capability.is_available());
        assert!(!capability.is_critical());
        
        capability = capability.with_support(true).required();
        assert!(capability.is_available());
        assert!(!capability.is_critical());
        
        capability.supported = false;
        assert!(capability.is_critical());
    }

    #[test]
    fn test_platform_stack() {
        let stack = PlatformStack::optimal();
        assert_eq!(stack.platform, Platform::current());
        
        let windows_stack = PlatformStack::for_platform(Platform::Windows);
        assert_eq!(windows_stack.platform, Platform::Windows);
        assert_eq!(windows_stack.web_engine, WebEngine::WebView2);
        assert_eq!(windows_stack.gui_framework, GUIFramework::WinAPI);
        
        assert!(windows_stack.is_fully_supported()); // No capabilities added yet
    }

    #[test]
    fn test_platform_features() {
        let windows_features = PlatformFeatures::for_platform(&Platform::Windows);
        assert!(windows_features.native_notifications);
        assert!(windows_features.system_tray);
        assert!(windows_features.transparency);
        
        let linux_features = PlatformFeatures::for_platform(&Platform::Linux);
        assert!(linux_features.native_notifications);
        assert!(!linux_features.transparency);
        assert!(!linux_features.global_shortcuts);
        
        assert!(windows_features.enabled_count() > linux_features.enabled_count());
    }

    #[test]
    fn test_display_names() {
        assert_eq!(Platform::Windows.display_name(), "Windows");
        assert_eq!(WebEngine::WebView2.display_name(), "Microsoft WebView2");
        assert_eq!(GUIFramework::WinAPI.display_name(), "Windows API");
    }
}