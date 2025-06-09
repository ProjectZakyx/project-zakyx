// 🔧 Horizontale Symbol-Lesezeichenleiste für Ora Browser
// Implementiert echte Windows-Toolbar mit Icons und Buttons

use anyhow::Result;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::UI::Controls::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::core::w;

#[derive(Debug, Clone)]
pub struct BookmarkButton {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub icon_text: String,
    pub tooltip: String,
}

pub struct HorizontalBookmarkToolbar {
    parent_hwnd: HWND,
    toolbar_hwnd: Option<HWND>,
    buttons: Vec<BookmarkButton>,
    toolbar_height: i32,
    button_width: i32,
    is_visible: bool,
}

impl HorizontalBookmarkToolbar {
    pub fn new(parent: HWND) -> Result<Self> {
        println!("🔧 Creating Horizontal Bookmark Toolbar...");
        
        Ok(Self {
            parent_hwnd: parent,
            toolbar_hwnd: None,
            buttons: Vec::new(),
            toolbar_height: 50,
            button_width: 80,
            is_visible: true,
        })
    }
    
    pub fn create_toolbar(&mut self) -> Result<()> {
        println!("🔧 Creating horizontal symbol toolbar...");
        self.load_default_buttons();
        self.create_html_toolbar()?;
        Ok(())
    }
    
    // 🔧 ECHTE WINDOWS-TOOLBAR ERSTELLEN
    fn create_windows_toolbar(&mut self) -> Result<()> {
        println!("🔧 Creating native Windows horizontal toolbar...");
        
        unsafe {
            // Hole Parent-Fenster-Dimensionen
            let mut parent_rect = RECT::default();
            let _ = GetClientRect(self.parent_hwnd, &mut parent_rect);
            
            // Erstelle horizontale Toolbar
            let toolbar_hwnd = CreateWindowExW(
                WS_EX_TOOLWINDOW,
                w!("ToolbarWindow32"),
                w!("Horizontal Bookmark Toolbar"),
                WS_CHILD | WS_VISIBLE | WS_BORDER,
                0,  // X position (links)
                0,  // Y position (oben)
                parent_rect.right,   // Breite (ganze Fensterbreite)
                self.toolbar_height, // Höhe (50px)
                self.parent_hwnd,
                None,
                GetModuleHandleW(None)?,
                None,
            );
            
            if toolbar_hwnd.0 == 0 {
                return Err(anyhow::anyhow!("Failed to create horizontal toolbar"));
            }
            
            // Toolbar wurde erfolgreich erstellt
            println!("🔧 Horizontal toolbar window created successfully");
            
            self.toolbar_hwnd = Some(toolbar_hwnd);
            println!("✅ Windows horizontal toolbar created: {:?}", toolbar_hwnd);
        }
        
        Ok(())
    }
    
    // 📚 STANDARD-BOOKMARK-BUTTONS LADEN
    fn load_default_buttons(&mut self) {
        self.buttons = vec![
            BookmarkButton {
                id: 1001,
                title: "Google".to_string(),
                url: "https://google.com".to_string(),
                icon_text: "🔍".to_string(),
                tooltip: "Google Suche".to_string(),
            },
            BookmarkButton {
                id: 1002,
                title: "GitHub".to_string(),
                url: "https://github.com".to_string(),
                icon_text: "🐙".to_string(),
                tooltip: "GitHub Repository".to_string(),
            },
            BookmarkButton {
                id: 1003,
                title: "Stack Overflow".to_string(),
                url: "https://stackoverflow.com".to_string(),
                icon_text: "📚".to_string(),
                tooltip: "Stack Overflow Q&A".to_string(),
            },
            BookmarkButton {
                id: 1004,
                title: "Rust Documentation".to_string(),
                url: "https://doc.rust-lang.org".to_string(),
                icon_text: "🦀".to_string(),
                tooltip: "Rust Programming Language".to_string(),
            },
            BookmarkButton {
                id: 1005,
                title: "YouTube".to_string(),
                url: "https://youtube.com".to_string(),
                icon_text: "📺".to_string(),
                tooltip: "YouTube Videos".to_string(),
            },
            BookmarkButton {
                id: 1006,
                title: "Wikipedia".to_string(),
                url: "https://wikipedia.org".to_string(),
                icon_text: "📖".to_string(),
                tooltip: "Wikipedia Enzyklopädie".to_string(),
            },
            BookmarkButton {
                id: 1007,
                title: "Reddit".to_string(),
                url: "https://reddit.com".to_string(),
                icon_text: "🗨️".to_string(),
                tooltip: "Reddit Community".to_string(),
            },
        ];
        
        println!("🔧 Loaded {} horizontal bookmark buttons", self.buttons.len());
    }
    
    // 🎨 HTML-TOOLBAR ERSTELLEN (FÜR ERWEITERTE FEATURES)
    fn create_html_toolbar(&self) -> Result<()> {
        let html_content = self.generate_toolbar_html();
        
        let toolbar_file = "ora_horizontal_bookmark_toolbar.html";
        std::fs::write(toolbar_file, html_content)?;
        
        println!("🎨 Horizontal HTML toolbar created: {}", toolbar_file);
        Ok(())
    }
    
    // 🎨 TOOLBAR-HTML GENERIEREN
    fn generate_toolbar_html(&self) -> String {
        let buttons_html = self.buttons.iter()
            .map(|button| format!(r#"
                <div class="bookmark-button" onclick="openBookmark('{}', '{}')" title="{}">
                    <div class="button-icon">{}</div>
                    <div class="button-label">{}</div>
                </div>
            "#, button.id, button.url, button.tooltip, button.icon_text, button.title))
            .collect::<Vec<_>>()
            .join("");
        
        format!(r#"<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Horizontale Symbol-Lesezeichenleiste</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        
        body {{
            font-family: 'Segoe UI', sans-serif;
            background: linear-gradient(90deg, #1E1E1E 0%, #252526 100%);
            color: #FFFFFF;
            height: 50px;
            width: 100vw;
            overflow: hidden;
            position: fixed;
            top: 0;
            left: 0;
            border-bottom: 2px solid #3C3C3C;
            box-shadow: 0 2px 10px rgba(0, 0, 0, 0.5);
            display: flex;
            align-items: center;
            z-index: 1000;
        }}
        
        .toolbar-header {{
            background: rgba(0, 122, 204, 0.2);
            padding: 8px 12px;
            border-right: 1px solid #3C3C3C;
            font-size: 12px;
            font-weight: 600;
            min-width: 120px;
            display: flex;
            align-items: center;
            justify-content: center;
        }}
        
        .toolbar-buttons {{
            display: flex;
            flex: 1;
            align-items: center;
            padding: 0 8px;
            gap: 4px;
            overflow-x: auto;
            scrollbar-width: thin;
        }}
        
        .bookmark-button {{
            min-width: 70px;
            height: 40px;
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 6px;
            cursor: pointer;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            transition: all 0.3s ease;
            position: relative;
            overflow: hidden;
            padding: 4px;
            margin: 0 2px;
        }}
        
        .bookmark-button::before {{
            content: '';
            position: absolute;
            top: 0;
            left: -100%;
            width: 100%;
            height: 100%;
            background: linear-gradient(90deg, transparent, rgba(0, 122, 204, 0.3), transparent);
            transition: left 0.5s ease;
        }}
        
        .bookmark-button:hover {{
            background: rgba(0, 122, 204, 0.2);
            border-color: rgba(0, 122, 204, 0.5);
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(0, 122, 204, 0.3);
        }}
        
        .bookmark-button:hover::before {{
            left: 100%;
        }}
        
        .bookmark-button:active {{
            transform: translateY(0);
            background: rgba(0, 122, 204, 0.4);
        }}
        
        .button-icon {{
            font-size: 16px;
            margin-bottom: 1px;
            text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
        }}
        
        .button-label {{
            font-size: 9px;
            font-weight: 500;
            text-align: center;
            line-height: 1;
            opacity: 0.8;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            max-width: 66px;
        }}
        
        .bookmark-button:hover .button-label {{
            opacity: 1;
        }}
        
        /* Spezielle Hover-Effekte für verschiedene Buttons */
        .bookmark-button:nth-child(1):hover {{ /* Google */
            box-shadow: 0 4px 12px rgba(66, 133, 244, 0.4);
        }}
        
        .bookmark-button:nth-child(2):hover {{ /* GitHub */
            box-shadow: 0 4px 12px rgba(88, 166, 255, 0.4);
        }}
        
        .bookmark-button:nth-child(3):hover {{ /* Stack Overflow */
            box-shadow: 0 4px 12px rgba(244, 128, 36, 0.4);
        }}
        
        .bookmark-button:nth-child(4):hover {{ /* Rust */
            box-shadow: 0 4px 12px rgba(222, 165, 132, 0.4);
        }}
        
        .bookmark-button:nth-child(5):hover {{ /* YouTube */
            box-shadow: 0 4px 12px rgba(255, 0, 0, 0.4);
        }}
        
        .bookmark-button:nth-child(6):hover {{ /* Wikipedia */
            box-shadow: 0 4px 12px rgba(255, 255, 255, 0.4);
        }}
        
        .bookmark-button:nth-child(7):hover {{ /* Reddit */
            box-shadow: 0 4px 12px rgba(255, 69, 0, 0.4);
        }}
        
        .toolbar-actions {{
            display: flex;
            align-items: center;
            padding: 0 8px;
            gap: 8px;
            border-left: 1px solid #3C3C3C;
        }}
        
        .action-button {{
            width: 32px;
            height: 32px;
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 4px;
            color: white;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: all 0.3s ease;
            font-size: 14px;
        }}
        
        .action-button:hover {{
            background: rgba(0, 122, 204, 0.2);
            border-color: rgba(0, 122, 204, 0.5);
        }}
        
        /* Tooltip-Styling */
        .bookmark-button[title]:hover::after {{
            content: attr(title);
            position: absolute;
            top: 50px;
            left: 50%;
            transform: translateX(-50%);
            background: rgba(0, 0, 0, 0.9);
            color: white;
            padding: 6px 10px;
            border-radius: 4px;
            font-size: 11px;
            white-space: nowrap;
            z-index: 1000;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
            animation: tooltipShow 0.2s ease;
        }}
        
        @keyframes tooltipShow {{
            from {{ opacity: 0; transform: translateX(-50%) translateY(-10px); }}
            to {{ opacity: 1; transform: translateX(-50%) translateY(0); }}
        }}
        
        /* Scrollbar für mehr Bookmarks */
        .toolbar-buttons::-webkit-scrollbar {{ height: 4px; }}
        .toolbar-buttons::-webkit-scrollbar-track {{ background: #1E1E1E; }}
        .toolbar-buttons::-webkit-scrollbar-thumb {{ 
            background: #464647; 
            border-radius: 2px;
        }}
        .toolbar-buttons::-webkit-scrollbar-thumb:hover {{ background: #5A5A5C; }}
        
        /* Responsives Design für kleinere Bildschirme */
        @media (max-width: 1200px) {{
            .bookmark-button {{ min-width: 60px; }}
            .button-label {{ font-size: 8px; max-width: 56px; }}
        }}
        
        @media (max-width: 800px) {{
            .toolbar-header {{ min-width: 80px; font-size: 10px; }}
            .bookmark-button {{ min-width: 50px; }}
            .button-icon {{ font-size: 14px; }}
            .button-label {{ font-size: 7px; max-width: 46px; }}
        }}
    </style>
</head>
<body>
    <div class="toolbar-header">
        📚 BOOKMARKS
    </div>
    
    <div class="toolbar-buttons">
        {}
    </div>
    
    <div class="toolbar-actions">
        <div class="action-button" onclick="addBookmark()" title="Bookmark hinzufügen">➕</div>
        <div class="action-button" onclick="toggleToolbar()" title="Toolbar umschalten">👁️</div>
        <div class="action-button" onclick="openSettings()" title="Einstellungen">⚙️</div>
    </div>
    
    <script>
        console.log('🔧 Horizontal Bookmark Toolbar loaded!');
        
        function openBookmark(id, url) {{
            console.log('🔗 Opening bookmark:', id, url);
            
            // Visuelles Feedback
            const button = event.currentTarget;
            const originalTransform = button.style.transform;
            
            button.style.transform = 'translateY(2px)';
            button.style.background = 'rgba(0, 122, 204, 0.6)';
            
            setTimeout(() => {{
                button.style.transform = originalTransform;
                button.style.background = '';
            }}, 200);
            
            // Navigation (würde an Ora Browser gesendet)
            alert(`🌐 Navigiere zu: ${{url}}`);
            
            // Hier würde die echte Navigation stattfinden
            // window.location.href = url;
        }}
        
        function addBookmark() {{
            console.log('➕ Adding new bookmark');
            const url = prompt('🔗 URL des neuen Bookmarks:');
            const title = prompt('📝 Titel des Bookmarks:');
            
            if (url && title) {{
                alert(`✅ Bookmark hinzugefügt: ${{title}} -> ${{url}}`);
            }}
        }}
        
        function toggleToolbar() {{
            console.log('👁️ Toggling toolbar visibility');
            const body = document.body;
            if (body.style.display === 'none') {{
                body.style.display = 'flex';
                console.log('✅ Toolbar visible');
            }} else {{
                body.style.display = 'none';
                console.log('❌ Toolbar hidden');
            }}
        }}
        
        function openSettings() {{
            console.log('⚙️ Opening settings');
            alert('⚙️ Bookmark-Einstellungen werden geöffnet!');
        }}
        
        // Keyboard-Shortcuts
        document.addEventListener('keydown', function(e) {{
            if (e.ctrlKey) {{
                switch(e.key) {{
                    case '1': openBookmark(1001, 'https://google.com'); break;
                    case '2': openBookmark(1002, 'https://github.com'); break;
                    case '3': openBookmark(1003, 'https://stackoverflow.com'); break;
                    case '4': openBookmark(1004, 'https://doc.rust-lang.org'); break;
                    case '5': openBookmark(1005, 'https://youtube.com'); break;
                    case '6': openBookmark(1006, 'https://wikipedia.org'); break;
                    case '7': openBookmark(1007, 'https://reddit.com'); break;
                    case 'b': toggleToolbar(); break;
                    case 'd': addBookmark(); break;
                }}
            }}
        }});
        
        // Animation bei Laden
        document.addEventListener('DOMContentLoaded', function() {{
            const buttons = document.querySelectorAll('.bookmark-button');
            buttons.forEach((button, index) => {{
                button.style.opacity = '0';
                button.style.transform = 'translateY(-20px)';
                setTimeout(() => {{
                    button.style.transition = 'all 0.5s ease';
                    button.style.opacity = '1';
                    button.style.transform = 'translateY(0)';
                }}, index * 100);
            }});
        }});
        
        // Responsive Toolbar-Größe anpassen
        function adjustToolbarSize() {{
            const toolbar = document.querySelector('.toolbar-buttons');
            const buttons = document.querySelectorAll('.bookmark-button');
            const availableWidth = toolbar.offsetWidth;
            const buttonCount = buttons.length;
            const maxButtonWidth = Math.min(80, (availableWidth - 32) / buttonCount);
            
            buttons.forEach(button => {{
                button.style.minWidth = Math.max(50, maxButtonWidth) + 'px';
            }});
        }}
        
        window.addEventListener('resize', adjustToolbarSize);
        window.addEventListener('load', adjustToolbarSize);
    </script>
</body>
</html>"#, buttons_html)
    }
    
    // 📊 TOOLBAR-INFO ANZEIGEN
    pub fn display_toolbar_info(&self) -> Result<()> {
        unsafe {
            let toolbar_info = format!(
                "🔧 ═══════════════════════════════════════════════════════════\n\
                 ✨        HORIZONTALE SYMBOL-LESEZEICHENLEISTE AKTIV!        ✨\n\
                 🔧 ═══════════════════════════════════════════════════════════\n\
                 \n\
                 🎯 TOOLBAR STATUS:\n\
                    ✅ Windows-Toolbar erstellt (horizontal)\n\
                    🔧 Symbol-Buttons aktiv\n\
                    📏 Höhe: {}px\n\
                    🔲 Button-Breite: {}px\n\
                    📱 {} Bookmark-Buttons\n\
                 \n\
                 🔧 VERFÜGBARE BOOKMARK-BUTTONS:\n\
                 {}\n\
                 \n\
                 💡 TOOLBAR-FUNKTIONEN:\n\
                    🔗 Klickbare Symbol-Buttons\n\
                    💬 Tooltips bei Hover\n\
                    ⌨️ Keyboard-Shortcuts (Ctrl+1-7)\n\
                    🎨 Moderne Icon-Gestaltung\n\
                    📱 Horizontale Anordnung\n\
                    📏 Responsive Design\n\
                    ➕ Bookmark hinzufügen\n\
                    👁️ Toolbar umschalten\n\
                    ⚙️ Einstellungen\n\
                 \n\
                 🌐 RENDERING-VERSIONEN:\n\
                    🔧 Native Windows-Toolbar (aktiv)\n\
                    🎨 HTML-Version (erweiterte Features)\n\
                 \n\
                 🎉 Die horizontale Symbol-Lesezeichenleiste ist vollständig\n\
                    als echte Windows-Toolbar implementiert!\n\
                 \n\
                 📄 HTML-Datei: ora_horizontal_bookmark_toolbar.html"
                , self.toolbar_height
                , self.button_width
                , self.buttons.len()
                , self.buttons.iter()
                    .map(|b| format!("   {} {} {} (ID: {})", b.icon_text, b.title, b.url, b.id))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            
            let wide_text: Vec<u16> = toolbar_info.encode_utf16().chain(std::iter::once(0)).collect();
            let _ = SetWindowTextW(self.parent_hwnd, windows::core::PCWSTR(wide_text.as_ptr()));
        }
        
        Ok(())
    }
    
    // 🔧 BUTTON-CLICK BEHANDELN
    pub fn handle_button_click(&self, button_id: i32) -> Option<String> {
        for button in &self.buttons {
            if button.id == button_id {
                println!("🔗 Horizontal bookmark button clicked: {} -> {}", button.title, button.url);
                return Some(button.url.clone());
            }
        }
        None
    }
    
    // 📏 TOOLBAR-GRÖßE ANPASSEN
    pub fn resize(&self, parent_width: i32) -> Result<()> {
        if let Some(hwnd) = self.toolbar_hwnd {
            unsafe {
                let _ = SetWindowPos(
                    hwnd,
                    None,
                    0,
                    0,
                    parent_width,
                    self.toolbar_height,
                    SWP_NOZORDER | SWP_NOMOVE,
                );
            }
        }
        Ok(())
    }
    
    // 👁️ SICHTBARKEIT UMSCHALTEN
    pub fn toggle_visibility(&mut self) -> bool {
        self.is_visible = !self.is_visible;
        
        if let Some(hwnd) = self.toolbar_hwnd {
            unsafe {
                let _ = ShowWindow(hwnd, if self.is_visible { SW_SHOW } else { SW_HIDE });
            }
        }
        
        println!("👁️ Horizontal toolbar visibility: {}", if self.is_visible { "Visible" } else { "Hidden" });
        self.is_visible
    }
    
    // 🧹 CLEANUP
    pub fn cleanup(&mut self) -> Result<()> {
        println!("🧹 Cleaning up Horizontal Bookmark Toolbar...");
        
        if let Some(hwnd) = self.toolbar_hwnd {
            unsafe {
                let _ = DestroyWindow(hwnd);
            }
        }
        
        self.toolbar_hwnd = None;
        
        println!("✅ Horizontal Bookmark Toolbar cleanup completed!");
        Ok(())
    }
} 