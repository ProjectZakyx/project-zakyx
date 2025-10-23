// 🌐 CORE PROXY MODULES
// Hauptgeschäftslogik für den Proxy-Server

pub mod strategy_engine;
pub mod response_processor;
pub mod universal_handler;
pub mod cors_handler;

// Re-exports (currently unused but kept for future API)
#[allow(unused_imports)]
pub use strategy_engine::StrategyEngine;
#[allow(unused_imports)]
pub use response_processor::ResponseProcessor;
#[allow(unused_imports)]
pub use universal_handler::UniversalResourceHandler;
#[allow(unused_imports)]
pub use cors_handler::CorsHandler;

// 🚀 Smart Proxy Core Module
// Enthält die Hauptlogik für das Proxy-System
#[allow(unused_imports)]
pub use strategy_engine::SmartProxy;
#[allow(unused_imports)]
pub use response_processor::ProxyResponse; 
