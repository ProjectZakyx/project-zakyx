//! # Ora Browser Library
//! 
//! Ora Browser ist ein moderner, sicherheitsorientierter Web-Browser
//! mit erweiterten Features und plattformübergreifender Unterstützung.

#![allow(dead_code)]
#![allow(unused_imports)]

// Core modules
pub mod webview_integration;
pub mod browser_features_simple;
pub mod performance_optimizer;
pub mod advanced_browser_features;
pub mod ui_ux_improvements;
pub mod security_features;
pub mod platform_extension;

// Re-exports for testing
pub use webview_integration::*;
pub use browser_features_simple::*;
pub use performance_optimizer::*;
pub use advanced_browser_features::*;
pub use ui_ux_improvements::*;
pub use security_features::*;
pub use platform_extension::*;

// Test utilities
#[cfg(test)]
pub mod test_utils {
    use std::sync::Once;
    use once_cell::sync::Lazy;
    use tokio::runtime::Runtime;
    
    static INIT: Once = Once::new();
    static RT: Lazy<Runtime> = Lazy::new(|| {
        Runtime::new().expect("Failed to create test runtime")
    });
    
    pub fn init_test_env() {
        INIT.call_once(|| {
            env_logger::init();
        });
    }
    
    pub fn test_runtime() -> &'static Runtime {
        &RT
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::test_utils::*;
    
    #[test]
    fn test_library_initialization() {
        init_test_env();
        // Basic smoke test
        assert_eq!(1 + 1, 2);
    }
} 