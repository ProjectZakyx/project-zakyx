// 🎨 UI/UX MODULE - Complete User Interface System
pub mod theme;
pub mod shortcuts;
pub mod drag;
pub mod fullscreen;
pub mod manager;

// Re-export all public types for easy access
pub use theme::{Theme, ColorScheme, ThemeManager};
pub use shortcuts::{KeyboardShortcut, ShortcutManager};
pub use drag::{TabDragInfo, TabDragManager};
pub use fullscreen::FullscreenManager;
pub use manager::UIUXImprovementsManager; 