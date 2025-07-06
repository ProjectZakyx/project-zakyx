// 🌐 Smart Proxy - Refactored Modular Architecture
// Ersetzt die monolithische smart_proxy.rs mit einer sauberen, modularen Struktur

pub mod core;
pub mod strategies;
pub mod classification;
pub mod processing;
pub mod utils;

use crate::error::OraBrowserError;

// Re-export der wichtigsten Typen für Rückwärtskompatibilität
pub use core::response_processor::ProxyResponse;
pub use core::strategy_engine::SmartProxy;

// Convenience-Funktionen für den einfachen Zugriff
pub async fn fetch_and_strip_headers(url: &str) -> Result<ProxyResponse, OraBrowserError> {
    SmartProxy::fetch_and_strip_headers(url).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_proxy_basic_functionality() {
        let result = fetch_and_strip_headers("https://httpbin.org/get").await;
        assert!(result.is_ok());
    }
} 