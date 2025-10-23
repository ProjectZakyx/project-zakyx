// 🔧 Advanced Browser Features für ZAKYX Browser
// LEGACY FILE - Verwende stattdessen src/browser/features/
// Diese Datei bleibt für Rückwärtskompatibilität

// Re-export der neuen modularen Struktur
pub use crate::browser::features::{
    AdvancedBrowserFeatures,
    DownloadManager,
    Download,
    DownloadStatus,
    DownloadStats,
    PasswordManager,
    SavedPassword,
    PasswordStrength,
    PasswordStats,
    ExtensionManager,
    Extension,
    ExtensionCategory,
    ExtensionStats,
    DeveloperTools,
    ConsoleLogEntry,
    NetworkLogEntry,
    LogLevel,
    LogFilters,
};