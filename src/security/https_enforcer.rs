// 🔒 HTTPS Enforcer für ZAKYX Browser
// LEGACY FILE - Verwende stattdessen src/security/https/
// Diese Datei bleibt für Rückwärtskompatibilität

// Re-export der neuen modularen Struktur
pub use crate::security::https::{
    HTTPSEnforcer,
    HttpsConfig,
    HttpsConfigManager,
    HttpsUrlRewriter,
    HttpsDomainManager,
    HttpsStatsReporter,
    SecurityConfig,
    DomainListType,
    DomainStats,
};