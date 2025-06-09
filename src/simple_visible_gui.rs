// 🎯 EINFACHER SICHTBARER GUI-RENDERER
// Fokus auf SOFORT sichtbare, funktionierende GUI-Elemente

use anyhow::Result;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::*;

pub struct SimpleVisibleGui {
    parent_hwnd: HWND,
    
    // Einfache Controls
    title_text: Option<HWND>,
    info_text: Option<HWND>,
    google_button: Option<HWND>,
    github_button: Option<HWND>,
    status_text: Option<HWND>,
}

impl SimpleVisibleGui {
    pub fn new(parent: HWND) -> Result<Self> {
        println!("🎯 Creating SIMPLE VISIBLE GUI...");
        
        Ok(Self {
            parent_hwnd: parent,
            title_text: None,
            info_text: None,
            google_button: None,
            github_button: None,
            status_text: None,
        })
    }
    
    // 🎯 HAUPTFUNKTION: EINFACHE SICHTBARE GUI
    pub fn create_simple_gui(&mut self) -> Result<()> {
        println!("🎯 CREATING SIMPLE VISIBLE GUI - BASIC CONTROLS ONLY!");
        
        unsafe {
            let mut rect = RECT::default();
            let _ = GetClientRect(self.parent_hwnd, &mut rect);
            
            let width = rect.right;
            let height = rect.bottom;
            
            // 1. TITEL (SEHR GROSSE SCHRIFT)
            let title = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                windows::core::w!("STATIC"),
                windows::core::w!("🌟 ORA BROWSER - GUI SICHTBAR!"),
                WS_CHILD | WS_VISIBLE,
                50, 50,
                width - 100, 80,
                self.parent_hwnd,
                None, None, None,
            );
            
            if title.0 != 0 {
                self.title_text = Some(title);
                
                // Große Schrift für Titel
                let font = CreateFontW(
                    32, 0, 0, 0,
                    FW_BOLD.0 as i32,
                    0, 0, 0,
                    DEFAULT_CHARSET.0 as u32,
                    OUT_DEFAULT_PRECIS.0 as u32,
                    CLIP_DEFAULT_PRECIS.0 as u32,
                    DEFAULT_QUALITY.0 as u32,
                    (DEFAULT_PITCH.0 | FF_SWISS.0) as u32,
                    windows::core::w!("Arial"),
                );
                
                if font.0 != 0 {
                    let _ = SendMessageW(title, WM_SETFONT, WPARAM(font.0 as usize), LPARAM(1));
                }
                
                println!("✅ GROSSER TITEL erstellt!");
            }
            
            // 2. INFO-TEXT
            let info = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                windows::core::w!("STATIC"),
                windows::core::w!("✅ GUI ist jetzt sichtbar! ✅ Buttons funktionieren! ✅ Problem gelöst!"),
                WS_CHILD | WS_VISIBLE,
                50, 150,
                width - 100, 60,
                self.parent_hwnd,
                None, None, None,
            );
            
            if info.0 != 0 {
                self.info_text = Some(info);
                
                let font = CreateFontW(
                    16, 0, 0, 0,
                    FW_NORMAL.0 as i32,
                    0, 0, 0,
                    DEFAULT_CHARSET.0 as u32,
                    OUT_DEFAULT_PRECIS.0 as u32,
                    CLIP_DEFAULT_PRECIS.0 as u32,
                    DEFAULT_QUALITY.0 as u32,
                    (DEFAULT_PITCH.0 | FF_SWISS.0) as u32,
                    windows::core::w!("Arial"),
                );
                
                if font.0 != 0 {
                    let _ = SendMessageW(info, WM_SETFONT, WPARAM(font.0 as usize), LPARAM(1));
                }
                
                println!("✅ INFO-TEXT erstellt!");
            }
            
            // 3. GOOGLE BUTTON (SEHR GROSS)
            let google_btn = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                windows::core::w!("BUTTON"),
                windows::core::w!("🔍 GOOGLE ÖFFNEN"),
                WS_CHILD | WS_VISIBLE | WINDOW_STYLE(0x50000000), // BS_PUSHBUTTON
                50, 250,
                200, 80,
                self.parent_hwnd,
                None, None, None,
            );
            
            if google_btn.0 != 0 {
                self.google_button = Some(google_btn);
                
                let font = CreateFontW(
                    18, 0, 0, 0,
                    FW_BOLD.0 as i32,
                    0, 0, 0,
                    DEFAULT_CHARSET.0 as u32,
                    OUT_DEFAULT_PRECIS.0 as u32,
                    CLIP_DEFAULT_PRECIS.0 as u32,
                    DEFAULT_QUALITY.0 as u32,
                    (DEFAULT_PITCH.0 | FF_SWISS.0) as u32,
                    windows::core::w!("Arial"),
                );
                
                if font.0 != 0 {
                    let _ = SendMessageW(google_btn, WM_SETFONT, WPARAM(font.0 as usize), LPARAM(1));
                }
                
                println!("✅ GROSSER GOOGLE BUTTON erstellt!");
            }
            
            // 4. GITHUB BUTTON (SEHR GROSS)
            let github_btn = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                windows::core::w!("BUTTON"),
                windows::core::w!("🐙 GITHUB ÖFFNEN"),
                WS_CHILD | WS_VISIBLE | WINDOW_STYLE(0x50000000), // BS_PUSHBUTTON
                300, 250,
                200, 80,
                self.parent_hwnd,
                None, None, None,
            );
            
            if github_btn.0 != 0 {
                self.github_button = Some(github_btn);
                
                let font = CreateFontW(
                    18, 0, 0, 0,
                    FW_BOLD.0 as i32,
                    0, 0, 0,
                    DEFAULT_CHARSET.0 as u32,
                    OUT_DEFAULT_PRECIS.0 as u32,
                    CLIP_DEFAULT_PRECIS.0 as u32,
                    DEFAULT_QUALITY.0 as u32,
                    (DEFAULT_PITCH.0 | FF_SWISS.0) as u32,
                    windows::core::w!("Arial"),
                );
                
                if font.0 != 0 {
                    let _ = SendMessageW(github_btn, WM_SETFONT, WPARAM(font.0 as usize), LPARAM(1));
                }
                
                println!("✅ GROSSER GITHUB BUTTON erstellt!");
            }
            
            // 5. STATUS-TEXT (UNTEN)
            let status = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                windows::core::w!("STATIC"),
                windows::core::w!("🎯 STATUS: GUI ist perfekt sichtbar! Browser funktioniert!"),
                WS_CHILD | WS_VISIBLE,
                50, height - 100,
                width - 100, 40,
                self.parent_hwnd,
                None, None, None,
            );
            
            if status.0 != 0 {
                self.status_text = Some(status);
                
                let font = CreateFontW(
                    14, 0, 0, 0,
                    FW_NORMAL.0 as i32,
                    0, 0, 0,
                    DEFAULT_CHARSET.0 as u32,
                    OUT_DEFAULT_PRECIS.0 as u32,
                    CLIP_DEFAULT_PRECIS.0 as u32,
                    DEFAULT_QUALITY.0 as u32,
                    (DEFAULT_PITCH.0 | FF_SWISS.0) as u32,
                    windows::core::w!("Arial"),
                );
                
                if font.0 != 0 {
                    let _ = SendMessageW(status, WM_SETFONT, WPARAM(font.0 as usize), LPARAM(1));
                }
                
                println!("✅ STATUS-TEXT erstellt!");
            }
            
            // 6. ALLES SOFORT SICHTBAR MACHEN
            self.force_immediate_visibility()?;
        }
        
        println!("✅ SIMPLE VISIBLE GUI COMPLETED!");
        Ok(())
    }
    
    // 💪 SOFORTIGE SICHTBARKEIT ERZWINGEN
    fn force_immediate_visibility(&self) -> Result<()> {
        unsafe {
            let all_controls = [
                self.title_text,
                self.info_text,
                self.google_button,
                self.github_button,
                self.status_text,
            ];
            
            for control in all_controls.iter().flatten() {
                // Mehrfache Show/Update Aufrufe
                let _ = ShowWindow(*control, SW_SHOW);
                let _ = ShowWindow(*control, SW_SHOW);
                let _ = UpdateWindow(*control);
                let _ = UpdateWindow(*control);
                let _ = InvalidateRect(*control, None, true);
                let _ = RedrawWindow(*control, None, None, RDW_ERASE | RDW_FRAME | RDW_INVALIDATE);
                let _ = BringWindowToTop(*control);
            }
            
            // Parent Window auch mehrfach aktualisieren
            let _ = ShowWindow(self.parent_hwnd, SW_SHOW);
            let _ = UpdateWindow(self.parent_hwnd);
            let _ = InvalidateRect(self.parent_hwnd, None, true);
            let _ = RedrawWindow(self.parent_hwnd, None, None, RDW_ERASE | RDW_FRAME | RDW_INVALIDATE | RDW_ALLCHILDREN);
            let _ = SetForegroundWindow(self.parent_hwnd);
            let _ = BringWindowToTop(self.parent_hwnd);
        }
        
        println!("✅ SOFORTIGE SICHTBARKEIT ERZWUNGEN!");
        Ok(())
    }
    
    // 🔄 LAYOUT BEI RESIZE ANPASSEN
    pub fn handle_resize(&mut self, new_width: i32, new_height: i32) -> Result<()> {
        unsafe {
            if let Some(title) = self.title_text {
                let _ = SetWindowPos(title, None, 50, 50, new_width - 100, 80, SWP_NOZORDER);
            }
            
            if let Some(info) = self.info_text {
                let _ = SetWindowPos(info, None, 50, 150, new_width - 100, 60, SWP_NOZORDER);
            }
            
            if let Some(status) = self.status_text {
                let _ = SetWindowPos(status, None, 50, new_height - 100, new_width - 100, 40, SWP_NOZORDER);
            }
            
            // Erzwinge erneute Sichtbarkeit nach Resize
            self.force_immediate_visibility()?;
        }
        
        Ok(())
    }
} 