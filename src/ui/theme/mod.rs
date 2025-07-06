// 🎨 THEME SYSTEM MODULE
pub mod color_scheme;
pub mod manager;

// Re-export public types
pub use color_scheme::{Theme, ColorScheme};
pub use manager::ThemeManager; 