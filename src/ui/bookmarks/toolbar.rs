// 📚 Horizontale Bookmark-Toolbar für ZAKYX Browser
// Hauptklasse für die Lesezeichenleiste

use anyhow::Result;
use windows::Win32::Foundation::HWND;

use super::manager::{BookmarkManager, BookmarkEntry};
use super::html_generator::BookmarkHtmlGenerator;
use super::windows_impl::WindowsToolbarImpl;

pub struct HorizontalBookmarkToolbar {
    manager: BookmarkManager,
    html_generator: BookmarkHtmlGenerator,
    windows_impl: WindowsToolbarImpl,
    is_visible: bool,
}

impl HorizontalBookmarkToolbar {
    /// Erstelle neue Bookmark-Toolbar
    pub fn new(parent: HWND) -> Result<Self> {
        println!("📚 Creating Horizontal Bookmark Toolbar...");
        
        let mut manager = BookmarkManager::new();
        manager.load_default_bookmarks();
        
        Ok(Self {
            manager,
            html_generator: BookmarkHtmlGenerator::new(),
            windows_impl: WindowsToolbarImpl::new(parent),
            is_visible: true,
        })
    }
    
    /// Erstelle und zeige Toolbar
    pub fn create_toolbar(&mut self) -> Result<()> {
        println!("📚 Creating horizontal bookmark toolbar...");
        
        // 1. Erstelle Windows-Toolbar-Fenster
        self.windows_impl.create_toolbar_window()?;
        
        // 2. Erstelle Toolbar-Inhalt
        self.create_toolbar_content()?;
        
        // 3. Zeige Toolbar-Info
        self.display_toolbar_info()?;
        
        println!("✅ Horizontal bookmark toolbar created successfully!");
        Ok(())
    }
    
    /// Erstelle Toolbar-Inhalt
    fn create_toolbar_content(&self) -> Result<()> {
        // Generiere HTML
        let html_content = self.html_generator.generate_toolbar_html(
            self.manager.get_bookmarks()
        );
        
        // Erstelle HTML-Datei
        let file_path = self.windows_impl.create_html_file(&html_content)?;
        
        // Lade HTML in Toolbar
        self.windows_impl.load_html_in_toolbar(&file_path)?;
        
        println!("📄 Toolbar content created and loaded");
        Ok(())
    }
    
    /// Zeige Toolbar-Informationen
    fn display_toolbar_info(&self) -> Result<()> {
        self.windows_impl.display_toolbar_info(
            self.manager.get_bookmarks().len()
        )
    }

    /// Füge Bookmark hinzu
    pub fn add_bookmark(&mut self, title: &str, url: &str) -> Result<String> {
        let id = self.manager.add_bookmark(title, url)?;
        
        // Aktualisiere Toolbar-Anzeige
        self.refresh_toolbar()?;
        
        Ok(id)
    }

    /// Entferne Bookmark
    pub fn remove_bookmark(&mut self, bookmark_id: &str) -> bool {
        let removed = self.manager.remove_bookmark(bookmark_id);
        
        if removed {
            // Aktualisiere Toolbar-Anzeige
            if let Err(e) = self.refresh_toolbar() {
                eprintln!("❌ Failed to refresh toolbar after removing bookmark: {}", e);
            }
        }
        
        removed
    }

    /// Aktualisiere Bookmark
    pub fn update_bookmark(&mut self, bookmark_id: &str, title: Option<String>, url: Option<String>) -> bool {
        let updated = self.manager.update_bookmark(bookmark_id, title, url);
        
        if updated {
            // Aktualisiere Toolbar-Anzeige
            if let Err(e) = self.refresh_toolbar() {
                eprintln!("❌ Failed to refresh toolbar after updating bookmark: {}", e);
            }
        }
        
        updated
    }

    /// Aktualisiere Toolbar-Anzeige
    pub fn refresh_toolbar(&self) -> Result<()> {
        self.create_toolbar_content()
    }

    /// Resize Toolbar
    pub fn resize(&self, parent_width: i32, parent_height: i32) -> Result<()> {
        self.windows_impl.resize(parent_width, parent_height)
    }

    /// Toggle Sichtbarkeit
    pub fn toggle_visibility(&mut self) -> bool {
        let new_visibility = self.windows_impl.toggle_visibility();
        self.is_visible = new_visibility;
        new_visibility
    }

    /// Toggle Position (oben/unten)
    pub fn toggle_position(&mut self) -> Result<()> {
        self.windows_impl.toggle_position()
    }

    /// Suche Bookmarks
    pub fn search_bookmarks(&self, query: &str) -> Vec<&BookmarkEntry> {
        self.manager.search_bookmarks(query)
    }

    /// Exportiere Bookmarks
    pub fn export_bookmarks(&self) -> Result<String> {
        self.manager.export_to_json()
    }

    /// Importiere Bookmarks
    pub fn import_bookmarks(&mut self, json: &str) -> Result<usize> {
        let imported = self.manager.import_from_json(json)?;
        
        if imported > 0 {
            self.refresh_toolbar()?;
        }
        
        Ok(imported)
    }

    /// Bereinige doppelte Bookmarks
    pub fn cleanup_duplicates(&mut self) -> usize {
        let removed = self.manager.cleanup_duplicates();
        
        if removed > 0 {
            if let Err(e) = self.refresh_toolbar() {
                eprintln!("❌ Failed to refresh toolbar after cleanup: {}", e);
            }
        }
        
        removed
    }

    /// Statistiken
    pub fn get_stats(&self) -> String {
        format!(
            "{}\n\
             \n\
             🪟 Windows-Toolbar:\n\
             • Sichtbar: {}\n\
             • Position: {}\n\
             • Höhe: {}px\n\
             • HWND: {:?}",
            self.manager.get_stats(),
            if self.is_visible { "Ja" } else { "Nein" },
            if self.windows_impl.is_on_top() { "Oben" } else { "Unten" },
            self.windows_impl.get_height(),
            self.windows_impl.get_hwnd()
        )
    }

    /// Cleanup
    pub fn cleanup(&mut self) -> Result<()> {
        self.windows_impl.cleanup()
    }

    // Getter
    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn get_bookmarks(&self) -> &[BookmarkEntry] {
        self.manager.get_bookmarks()
    }

    pub fn find_bookmark(&self, bookmark_id: &str) -> Option<&BookmarkEntry> {
        self.manager.find_bookmark(bookmark_id)
    }
}

/// Manager für mehrere Bookmark-Toolbars
pub struct BookmarkToolbarManager {
    toolbar: Option<HorizontalBookmarkToolbar>,
}

impl BookmarkToolbarManager {
    pub fn new() -> Self {
        Self {
            toolbar: None,
        }
    }

    pub fn initialize(&mut self, parent: HWND) -> Result<()> {
        let mut toolbar = HorizontalBookmarkToolbar::new(parent)?;
        toolbar.create_toolbar()?;
        self.toolbar = Some(toolbar);
        
        println!("📚 BookmarkToolbarManager initialized");
        Ok(())
    }

    pub fn get_toolbar(&mut self) -> Option<&mut HorizontalBookmarkToolbar> {
        self.toolbar.as_mut()
    }

    pub fn toggle_visibility(&mut self) -> bool {
        if let Some(toolbar) = &mut self.toolbar {
            toolbar.toggle_visibility()
        } else {
            false
        }
    }

    pub fn add_current_bookmark(&mut self, title: &str, url: &str) -> Result<()> {
        if let Some(toolbar) = &mut self.toolbar {
            toolbar.add_bookmark(title, url)?;
        }
        Ok(())
    }

    pub fn get_stats(&self) -> String {
        if let Some(toolbar) = &self.toolbar {
            toolbar.get_stats()
        } else {
            "📚 Keine Toolbar initialisiert".to_string()
        }
    }

    pub fn cleanup(&mut self) -> Result<()> {
        if let Some(toolbar) = &mut self.toolbar {
            toolbar.cleanup()?;
        }
        self.toolbar = None;
        Ok(())
    }
}

impl Default for BookmarkToolbarManager {
    fn default() -> Self {
        Self::new()
    }
}