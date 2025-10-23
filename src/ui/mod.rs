// 🎨 UI Module für ZAKYX Browser
// Benutzeroberflächen-Komponenten

pub mod bookmarks;

// Weitere UI-Module können hier hinzugefügt werden:
// pub mod tabs;
// pub mod settings;
// pub mod downloads;

// Re-exports für einfachen Zugriff
pub use bookmarks::{
    BookmarkEntry, 
    BookmarkManager, 
    HorizontalBookmarkToolbar, 
    BookmarkToolbarManager,
    BookmarkHtmlGenerator
};

#[cfg(target_os = "windows")]
pub use bookmarks::WindowsToolbarImpl;