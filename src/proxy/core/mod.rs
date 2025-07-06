// 🌐 CORE PROXY MODULES
// Hauptgeschäftslogik für den Proxy-Server

pub mod strategy_engine;
pub mod response_processor;
pub mod universal_handler;
pub mod cors_handler;

// Re-exports
pub use strategy_engine::StrategyEngine;
pub use response_processor::ResponseProcessor;
pub use universal_handler::UniversalResourceHandler;
pub use cors_handler::CorsHandler;

// 🚀 Smart Proxy Core Module
// Enthält die Hauptlogik für das Proxy-System

pub use strategy_engine::SmartProxy;
pub use response_processor::ProxyResponse; 