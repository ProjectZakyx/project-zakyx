// 🔌 Plugin Manager für ZAKYX Browser
// LEGACY FILE - Verwende stattdessen src/plugin/management/
// Diese Datei bleibt für Rückwärtskompatibilität

// Re-export der neuen modularen Struktur
pub use crate::plugin::management::{
    PluginManager,
    PluginRegistry,
    PluginLifecycleManager,
    PluginOperationsManager,
    DetailedPluginInfo,
    ExtendedPluginStats,
    ExportFormat,
    DependencyIssue,
    DependencyIssueType,
};