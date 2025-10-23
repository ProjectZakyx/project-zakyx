// 🪟 Windows-spezifische Implementation für Bookmark Toolbar
// Enthält Windows API Calls und plattformspezifische Logik

use anyhow::Result;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::w;
use std::path::Path;

pub struct WindowsToolbarImpl {
    parent_hwnd: HWND,
    toolbar_hwnd: Option<HWND>,
    toolbar_height: i32,
    is_on_top: bool,
}

impl WindowsToolbarImpl {
    pub fn new(parent: HWND) -> Self {
        Self {
            parent_hwnd: parent,
            toolbar_hwnd: None,
            toolbar_height: 60,
            is_on_top: true,
        }
    }

    /// Erstelle Windows-Toolbar-Fenster
    pub fn create_toolbar_window(&mut self) -> Result<()> {
        println!("🏗️ Creating real toolbar window in ZAKYX Browser...");
        
        unsafe {
            // Erstelle Toolbar als Child-Window
            let toolbar_hwnd = CreateWindowExW(
                WS_EX_STATICEDGE,
                w!("STATIC"),
                w!("ZAKYX Bookmark Toolbar"),
                WS_CHILD | WS_VISIBLE | WS_BORDER,
                0,  // X position (links)
                if self.is_on_top { 0 } else { 540 }, // Y position (oben/unten)
                800, // Breite
                self.toolbar_height,  // Höhe
                Some(self.parent_hwnd),
                None,
                None,
                None,
            );
            
            let toolbar_hwnd = toolbar_hwnd?;
            
            if toolbar_hwnd.0 == std::ptr::null_mut() {
                return Err(anyhow::anyhow!("Failed to create toolbar window"));
            }
            
            self.toolbar_hwnd = Some(toolbar_hwnd);
            println!("✅ Real toolbar window created: {:?}", toolbar_hwnd);
        }
        
        Ok(())
    }

    /// Lade HTML in Toolbar-Fenster
    pub fn load_html_in_toolbar(&self, file_path: &str) -> Result<()> {
        if let Some(hwnd) = self.toolbar_hwnd {
            unsafe {
                // Zeige Toolbar-Info direkt im Browser-Fenster
                let toolbar_text = format!(
                    "📚 HORIZONTALE LESEZEICHENLEISTE\n\
                     \n\
                     ✅ AKTIV IM ZAKYX BROWSER!\n\
                     \n\
                     📄 HTML-Datei: {}\n\
                     \n\
                     📍 Position: {} Bereich\n\
                     📏 Höhe: {}px\n\
                     \n\
                     💡 Tipp: Öffne die HTML-Datei im Browser für die volle Funktionalität!",
                    file_path,
                    if self.is_on_top { "Oberer" } else { "Unterer" },
                    self.toolbar_height
                );

                // Setze Text in das Toolbar-Fenster
                let wide_text: Vec<u16> = toolbar_text.encode_utf16().chain([0]).collect();
                SetWindowTextW(hwnd, windows::core::PCWSTR(wide_text.as_ptr()));

                println!("📄 Toolbar content loaded in window: {:?}", hwnd);
            }
        }
        Ok(())
    }

    /// Resize Toolbar
    pub fn resize(&self, parent_width: i32, parent_height: i32) -> Result<()> {
        if let Some(hwnd) = self.toolbar_hwnd {
            unsafe {
                let y_pos = if self.is_on_top { 
                    0 
                } else { 
                    parent_height - self.toolbar_height 
                };

                SetWindowPos(
                    hwnd,
                    None,
                    0,
                    y_pos,
                    parent_width,
                    self.toolbar_height,
                    SWP_NOZORDER | SWP_NOACTIVATE,
                )?;

                println!("📏 Toolbar resized: {}x{} at y={}", 
                    parent_width, self.toolbar_height, y_pos);
            }
        }
        Ok(())
    }

    /// Toggle Sichtbarkeit
    pub fn toggle_visibility(&mut self) -> bool {
        if let Some(hwnd) = self.toolbar_hwnd {
            unsafe {
                let is_visible = IsWindowVisible(hwnd).as_bool();
                ShowWindow(hwnd, if is_visible { SW_HIDE } else { SW_SHOW });
                
                let new_visibility = !is_visible;
                println!("👁️ Toolbar visibility: {}", if new_visibility { "Visible" } else { "Hidden" });
                return new_visibility;
            }
        }
        false
    }

    /// Toggle Position (oben/unten)
    pub fn toggle_position(&mut self) -> Result<()> {
        self.is_on_top = !self.is_on_top;
        
        if let Some(hwnd) = self.toolbar_hwnd {
            unsafe {
                let mut rect = RECT::default();
                GetWindowRect(self.parent_hwnd, &mut rect)?;
                let parent_height = rect.bottom - rect.top;
                
                let y_pos = if self.is_on_top { 
                    0 
                } else { 
                    parent_height - self.toolbar_height 
                };

                SetWindowPos(
                    hwnd,
                    None,
                    0,
                    y_pos,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
                )?;

                println!("📍 Toolbar position: {}", 
                    if self.is_on_top { "Top" } else { "Bottom" });
            }
        }
        Ok(())
    }

    /// Cleanup Windows-Ressourcen
    pub fn cleanup(&mut self) -> Result<()> {
        if let Some(hwnd) = self.toolbar_hwnd.take() {
            unsafe {
                DestroyWindow(hwnd)?;
                println!("🧹 Toolbar window destroyed: {:?}", hwnd);
            }
        }
        Ok(())
    }

    /// Erstelle HTML-Datei
    pub fn create_html_file(&self, html_content: &str) -> Result<String> {
        let toolbar_file = "zakyx_horizontal_bookmark_toolbar.html";
        std::fs::write(toolbar_file, html_content)?;
        
        println!("📄 Toolbar HTML created: {}", toolbar_file);
        Ok(toolbar_file.to_string())
    }

    /// Zeige Toolbar-Info
    pub fn display_toolbar_info(&self, bookmark_count: usize) -> Result<()> {
        if let Some(hwnd) = self.toolbar_hwnd {
            unsafe {
                let info_text = format!(
                    "📚 ZAKYX BOOKMARK TOOLBAR\n\
                     \n\
                     ✅ Status: Aktiv\n\
                     🔖 Bookmarks: {}\n\
                     📍 Position: {}\n\
                     📏 Höhe: {}px\n\
                     👁️ Sichtbar: {}\n\
                     \n\
                     ⌨️ Shortcuts:\n\
                     • Ctrl+D: Bookmark hinzufügen\n\
                     • Ctrl+Shift+B: Sichtbarkeit umschalten\n\
                     • Ctrl+Shift+T: Position umschalten",
                    bookmark_count,
                    if self.is_on_top { "Oben" } else { "Unten" },
                    self.toolbar_height,
                    if IsWindowVisible(hwnd).as_bool() { "Ja" } else { "Nein" }
                );

                let wide_text: Vec<u16> = info_text.encode_utf16().chain([0]).collect();
                SetWindowTextW(hwnd, windows::core::PCWSTR(wide_text.as_ptr()));
            }
        }
        Ok(())
    }

    // Getter
    pub fn is_visible(&self) -> bool {
        if let Some(hwnd) = self.toolbar_hwnd {
            unsafe {
                IsWindowVisible(hwnd).as_bool()
            }
        } else {
            false
        }
    }

    pub fn get_hwnd(&self) -> Option<HWND> {
        self.toolbar_hwnd
    }

    pub fn get_height(&self) -> i32 {
        self.toolbar_height
    }

    pub fn is_on_top(&self) -> bool {
        self.is_on_top
    }
}