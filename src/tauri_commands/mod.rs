// 🎯 Tauri Commands - Refactored Modular Architecture
// Ersetzt die monolithische tauri_commands.rs mit einer sauberen, modularen Struktur

pub mod tab_management;
pub mod navigation;
pub mod bookmark_management;
pub mod plugin_management;
pub mod settings;
pub mod history;
pub mod utils;

// Re-export aller Commands für einfachen Zugriff
pub use tab_management::*;
pub use navigation::*;
pub use bookmark_management::*;
pub use plugin_management::*;
pub use settings::*;
pub use history::*;
pub use utils::*; 