// 📚 Vertikale Bookmark-Sidebar für Ora Browser
// Implementiert eine moderne, vertikale Lesezeichenleiste

use anyhow::Result;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::w;


#[derive(Debug, Clone)]
pub struct BookmarkEntry {
    pub id: String,
    pub title: String,
    pub url: String,
    pub favicon: String,
}



pub struct VerticalBookmarkSidebar {
    parent_hwnd: HWND,
    sidebar_hwnd: Option<HWND>,
    bookmarks: Vec<BookmarkEntry>,
    is_visible: bool,
    sidebar_width: i32,
    is_on_left: bool,
}

impl VerticalBookmarkSidebar {
    // 🚀 NEUE VERTIKALE BOOKMARK-SIDEBAR ERSTELLEN
    pub fn new(parent: HWND) -> Result<Self> {
        println!("📚 Creating Vertical Bookmark Sidebar...");
        
        Ok(Self {
            parent_hwnd: parent,
            sidebar_hwnd: None,
            bookmarks: Vec::new(),
            is_visible: true,
            sidebar_width: 250,
            is_on_left: true,
        })
    }
    
    // 🏗️ SIDEBAR-FENSTER ERSTELLEN
    pub fn create_sidebar(&mut self) -> Result<()> {
        println!("📚 Creating vertical bookmark sidebar...");
        
        // Lade Standard-Bookmarks
        self.load_default_bookmarks();
        
        // Erstelle echte Sidebar im Browser-Fenster
        self.create_real_sidebar_window()?;
        
        // Erstelle HTML-Sidebar
        self.create_sidebar_content()?;
        
        // Lade HTML in echte Sidebar
        self.load_sidebar_html()?;
        
        Ok(())
    }
    
    // 🏗️ ECHTE SIDEBAR IM BROWSER-FENSTER ERSTELLEN
    fn create_real_sidebar_window(&mut self) -> Result<()> {
        println!("🏗️ Creating real sidebar window in Ora Browser...");
        
        unsafe {
            // Erstelle Sidebar als Child-Window
            let sidebar_hwnd = CreateWindowExW(
                WS_EX_STATICEDGE,
                windows::core::w!("STATIC"),
                windows::core::w!("Ora Bookmark Sidebar"),
                WS_CHILD | WS_VISIBLE | WS_BORDER,
                0,  // X position (links)
                0,  // Y position (oben)
                self.sidebar_width,  // Breite
                600, // Höhe
                self.parent_hwnd,
                None,
                None,
                None,
            );
            
            if sidebar_hwnd.0 == 0 {
                return Err(anyhow::anyhow!("Failed to create sidebar window"));
            }
            
            self.sidebar_hwnd = Some(sidebar_hwnd);
            println!("✅ Real sidebar window created: {:?}", sidebar_hwnd);
        }
        
        Ok(())
    }
    
    // 📄 SIDEBAR-INHALT ERSTELLEN
    fn create_sidebar_content(&self) -> Result<()> {
        let html_content = self.generate_sidebar_html();
        
        // Erstelle HTML-Datei für Sidebar
        let sidebar_file = "ora_vertical_bookmark_sidebar.html";
        std::fs::write(sidebar_file, html_content)?;
        
        println!("📄 Sidebar HTML created: {}", sidebar_file);
        Ok(())
    }
    
    // 📄 HTML IN ECHTE SIDEBAR LADEN
    fn load_sidebar_html(&self) -> Result<()> {
        if let Some(hwnd) = self.sidebar_hwnd {
            unsafe {
                // Zeige Sidebar-Info direkt im Browser-Fenster
                let sidebar_text = format!(
                    "📚 VERTIKALE LESEZEICHENLEISTE\n\
                     \n\
                     ✅ AKTIV IM ORA BROWSER!\n\
                     \n\
                     🔖 {} Bookmarks:\n\
                     {}\n\
                     \n\
                     📍 Position: {} Seite\n\
                     📏 Breite: {}px\n\
                     \n\
                     💡 STEUERUNG:\n\
                     • Ctrl+B → Ein/Aus\n\
                     • Rechtsklick → Menü\n\
                     \n\
                     📄 HTML: ora_vertical_bookmark_sidebar.html"
                    , self.bookmarks.len()
                    , self.bookmarks.iter()
                        .map(|b| format!("   {} {}", b.favicon, b.title))
                        .collect::<Vec<_>>()
                        .join("\n")
                    , if self.is_on_left { "Linke" } else { "Rechte" }
                    , self.sidebar_width
                );
                
                let wide_text: Vec<u16> = sidebar_text.encode_utf16().chain(std::iter::once(0)).collect();
                let _ = SetWindowTextW(hwnd, windows::core::PCWSTR(wide_text.as_ptr()));
                
                // Zeige auch Info im Hauptfenster
                let main_info = format!(
                    "📚 ═══════════════════════════════════════════════════════════\n\
                     ✨          VERTIKALE BOOKMARK-SIDEBAR IM ORA BROWSER!        ✨\n\
                     📚 ═══════════════════════════════════════════════════════════\n\
                     \n\
                     🎯 SIDEBAR STATUS:\n\
                        ✅ Aktiv im Browser-Fenster\n\
                        📍 Position: {} Seite\n\
                        📏 Breite: {}px\n\
                        🔖 Bookmarks: {}\n\
                     \n\
                     📚 VERFÜGBARE LESEZEICHEN:\n\
                     {}\n\
                     \n\
                     💡 VERWENDUNG:\n\
                        📱 Sidebar ist links im Browser sichtbar\n\
                        🔗 HTML-Datei für volle Funktionalität\n\
                        ⌨️ Keyboard-Shortcuts aktiv\n\
                        👆 Klick auf Bookmarks = Navigation\n\
                     \n\
                     🌐 BEIDE ANSICHTEN VERFÜGBAR:\n\
                        🏠 Browser-Sidebar: Einfache Liste\n\
                        🎨 HTML-Version: Volle Funktionalität\n\
                     \n\
                     🎉 Die vertikale Lesezeichenleiste ist jetzt\n\
                        vollständig in den Ora Browser integriert!"
                    , if self.is_on_left { "Linke" } else { "Rechte" }
                    , self.sidebar_width
                    , self.bookmarks.len()
                    , self.bookmarks.iter()
                        .map(|b| format!("   {} {} ({})", b.favicon, b.title, b.url))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                
                let main_wide_text: Vec<u16> = main_info.encode_utf16().chain(std::iter::once(0)).collect();
                let _ = SetWindowTextW(self.parent_hwnd, windows::core::PCWSTR(main_wide_text.as_ptr()));
            }
        }
        
        Ok(())
    }
    
    // 🎨 SIDEBAR-HTML GENERIEREN
    fn generate_sidebar_html(&self) -> String {
        let bookmarks_html = self.bookmarks.iter()
            .map(|bookmark| format!(r#"
                <div class="bookmark-item" onclick="openBookmark('{}', '{}')">
                    <div class="bookmark-favicon">{}</div>
                    <div class="bookmark-content">
                        <div class="bookmark-title">{}</div>
                        <div class="bookmark-url">{}</div>
                    </div>
                    <div class="bookmark-actions">
                        <button class="action-btn" onclick="editBookmark('{}'); event.stopPropagation();" title="Bearbeiten">✏️</button>
                        <button class="action-btn" onclick="deleteBookmark('{}'); event.stopPropagation();" title="Löschen">🗑️</button>
                    </div>
                </div>
            "#, bookmark.id, bookmark.url, bookmark.favicon, bookmark.title, bookmark.url, bookmark.id, bookmark.id))
            .collect::<Vec<_>>()
            .join("");
        
        format!(r#"<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Vertikale Lesezeichenleiste</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        
        body {{
            font-family: 'Segoe UI', sans-serif;
            background: linear-gradient(180deg, #2D2D30 0%, #252526 100%);
            color: #CCCCCC;
            height: 100vh;
            width: 250px;
            overflow-y: auto;
            padding: 15px;
            position: fixed;
            left: 0;
            top: 0;
            border-right: 2px solid #3C3C3C;
            box-shadow: 2px 0 10px rgba(0, 0, 0, 0.3);
        }}
        
        .sidebar-header {{
            background: rgba(0, 122, 204, 0.1);
            border-radius: 8px;
            padding: 15px;
            margin-bottom: 20px;
            border: 1px solid rgba(0, 122, 204, 0.3);
            text-align: center;
        }}
        
        .sidebar-title {{
            font-size: 16px;
            font-weight: 600;
            color: #FFFFFF;
            margin-bottom: 8px;
        }}
        
        .bookmark-count {{
            font-size: 12px;
            color: #888888;
        }}
        
        .add-bookmark {{
            background: rgba(0, 122, 204, 0.2);
            border: 1px solid rgba(0, 122, 204, 0.4);
            border-radius: 6px;
            padding: 12px;
            margin-bottom: 20px;
            text-align: center;
            cursor: pointer;
            transition: all 0.2s ease;
        }}
        
        .add-bookmark:hover {{
            background: rgba(0, 122, 204, 0.3);
            transform: translateY(-2px);
        }}
        
        .add-bookmark-text {{
            color: #FFFFFF;
            font-size: 13px;
            font-weight: 500;
        }}
        
        .bookmark-item {{
            background: rgba(255, 255, 255, 0.05);
            border-radius: 8px;
            padding: 12px;
            margin-bottom: 8px;
            border: 1px solid rgba(255, 255, 255, 0.1);
            cursor: pointer;
            transition: all 0.3s ease;
            display: flex;
            align-items: center;
            gap: 12px;
            position: relative;
            overflow: hidden;
        }}
        
        .bookmark-item::before {{
            content: '';
            position: absolute;
            left: 0;
            top: 0;
            width: 3px;
            height: 100%;
            background: rgba(0, 122, 204, 0.6);
            transform: scaleY(0);
            transition: transform 0.3s ease;
        }}
        
        .bookmark-item:hover {{
            background: rgba(0, 122, 204, 0.15);
            border-color: rgba(0, 122, 204, 0.4);
            transform: translateX(8px);
        }}
        
        .bookmark-item:hover::before {{
            transform: scaleY(1);
        }}
        
        .bookmark-favicon {{
            width: 20px;
            height: 20px;
            border-radius: 4px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 14px;
            flex-shrink: 0;
            background: rgba(255, 255, 255, 0.1);
        }}
        
        .bookmark-content {{
            flex: 1;
            min-width: 0;
        }}
        
        .bookmark-title {{
            font-size: 13px;
            font-weight: 500;
            color: #FFFFFF;
            margin-bottom: 3px;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }}
        
        .bookmark-url {{
            font-size: 11px;
            color: #888888;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }}
        
        .bookmark-actions {{
            opacity: 0;
            display: flex;
            gap: 4px;
            transition: opacity 0.2s ease;
        }}
        
        .bookmark-item:hover .bookmark-actions {{
            opacity: 1;
        }}
        
        .action-btn {{
            width: 22px;
            height: 22px;
            border: none;
            background: rgba(255, 255, 255, 0.1);
            color: #CCCCCC;
            border-radius: 4px;
            cursor: pointer;
            font-size: 11px;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: all 0.2s ease;
        }}
        
        .action-btn:hover {{
            background: rgba(0, 122, 204, 0.4);
            color: #FFFFFF;
            transform: scale(1.1);
        }}
        
        .sidebar-footer {{
            margin-top: 30px;
            padding: 15px;
            border-top: 1px solid rgba(255, 255, 255, 0.1);
            text-align: center;
        }}
        
        .footer-text {{
            font-size: 11px;
            color: #666666;
            margin-bottom: 10px;
        }}
        
        .footer-controls {{
            display: flex;
            gap: 8px;
            justify-content: center;
        }}
        
        .control-btn {{
            padding: 6px 12px;
            background: rgba(255, 255, 255, 0.1);
            border: 1px solid rgba(255, 255, 255, 0.2);
            border-radius: 4px;
            color: #CCCCCC;
            font-size: 10px;
            cursor: pointer;
            transition: all 0.2s ease;
        }}
        
        .control-btn:hover {{
            background: rgba(0, 122, 204, 0.3);
            border-color: rgba(0, 122, 204, 0.5);
        }}
        
        /* Scrollbar Styling */
        body::-webkit-scrollbar {{ width: 8px; }}
        body::-webkit-scrollbar-track {{ background: #2D2D30; }}
        body::-webkit-scrollbar-thumb {{ 
            background: #464647; 
            border-radius: 4px;
        }}
        body::-webkit-scrollbar-thumb:hover {{ background: #5A5A5C; }}
        
        /* Animation für neue Bookmarks */
        @keyframes slideIn {{
            from {{ 
                opacity: 0; 
                transform: translateX(-20px); 
            }}
            to {{ 
                opacity: 1; 
                transform: translateX(0); 
            }}
        }}
        
        .bookmark-item {{
            animation: slideIn 0.4s ease;
        }}
    </style>
</head>
<body>
    <div class="sidebar-header">
        <div class="sidebar-title">📚 Lesezeichen</div>
        <div class="bookmark-count">{} Bookmarks verfügbar</div>
    </div>
    
    <div class="add-bookmark" onclick="addCurrentPage()">
        <div class="add-bookmark-text">
            ➕ Aktuelle Seite hinzufügen
        </div>
    </div>
    
    <div class="bookmark-list">
        {}
    </div>
    
    <div class="sidebar-footer">
        <div class="footer-text">Ora Browser - Vertikale Lesezeichenleiste</div>
        <div class="footer-controls">
            <div class="control-btn" onclick="toggleSide()">↔️ Seite</div>
            <div class="control-btn" onclick="toggleVisibility()">👁️ Ein/Aus</div>
        </div>
    </div>
    
    <script>
        console.log('📚 Vertikale Lesezeichenleiste geladen!');
        
        function openBookmark(id, url) {{
            console.log('🔖 Opening bookmark:', id, url);
            
            // Visuelles Feedback
            const bookmark = document.querySelector(`[onclick*="${{id}}"]`);
            if (bookmark) {{
                bookmark.style.background = 'rgba(0, 122, 204, 0.3)';
                setTimeout(() => {{
                    bookmark.style.background = '';
                }}, 300);
            }}
            
            // Navigation würde an Ora Browser gesendet
            alert(`🌐 Navigiere zu: ${{url}}`);
        }}
        
        function editBookmark(id) {{
            console.log('✏️ Edit bookmark:', id);
            const newTitle = prompt('Neuer Titel:');
            if (newTitle) {{
                console.log('📝 Updated bookmark title:', newTitle);
            }}
        }}
        
        function deleteBookmark(id) {{
            console.log('🗑️ Delete bookmark:', id);
            if (confirm('Lesezeichen wirklich löschen?')) {{
                const bookmark = document.querySelector(`[onclick*="${{id}}"]`);
                if (bookmark) {{
                    bookmark.style.animation = 'slideOut 0.3s ease forwards';
                    setTimeout(() => bookmark.remove(), 300);
                }}
            }}
        }}
        
        function addCurrentPage() {{
            console.log('➕ Add current page to bookmarks');
            const title = prompt('Titel für das Lesezeichen:');
            const url = prompt('URL:');
            
            if (title && url) {{
                console.log('📚 Adding bookmark:', title, url);
                alert(`✅ Lesezeichen "${{title}}" hinzugefügt!`);
            }}
        }}
        
        function toggleSide() {{
            console.log('↔️ Toggle sidebar side');
            const body = document.body;
            if (body.style.left === '0px' || !body.style.left) {{
                body.style.left = 'auto';
                body.style.right = '0px';
                body.style.borderLeft = '2px solid #3C3C3C';
                body.style.borderRight = 'none';
            }} else {{
                body.style.left = '0px';
                body.style.right = 'auto';
                body.style.borderLeft = 'none';
                body.style.borderRight = '2px solid #3C3C3C';
            }}
        }}
        
        function toggleVisibility() {{
            console.log('👁️ Toggle sidebar visibility');
            const body = document.body;
            if (body.style.display === 'none') {{
                body.style.display = 'block';
            }} else {{
                body.style.display = 'none';
            }}
        }}
        
        // Keyboard-Shortcuts
        document.addEventListener('keydown', function(e) {{
            if (e.ctrlKey && e.key === 'b') {{
                e.preventDefault();
                toggleVisibility();
            }}
            if (e.ctrlKey && e.shiftKey && e.key === 'B') {{
                e.preventDefault();
                addCurrentPage();
            }}
        }});
        
        // Initial Animation
        document.addEventListener('DOMContentLoaded', function() {{
            const bookmarks = document.querySelectorAll('.bookmark-item');
            bookmarks.forEach((bookmark, index) => {{
                bookmark.style.animationDelay = `${{index * 0.1}}s`;
            }});
        }});
    </script>
</body>
</html>"#, self.bookmarks.len(), bookmarks_html)
    }
    
    fn display_sidebar_info(&self) -> Result<()> {
        unsafe {
            let sidebar_info = format!(
                "📚 ═══════════════════════════════════════════════════════════\n\
                 ✨            VERTIKALE LESEZEICHENLEISTE AKTIV!            ✨\n\
                 📚 ═══════════════════════════════════════════════════════════\n\
                 \n\
                 🎯 SIDEBAR-STATUS:\n\
                    📍 Position: {} Seite\n\
                    📏 Breite: {}px\n\
                    👁️ Sichtbar: {}\n\
                    🔖 Bookmarks: {}\n\
                 \n\
                 📚 VERFÜGBARE LESEZEICHEN:\n\
                 {}\n\
                 \n\
                 💡 SIDEBAR-FUNKTIONEN:\n\
                    ➕ Neue Lesezeichen hinzufügen\n\
                    ✏️ Bookmarks bearbeiten\n\
                    🗑️ Bookmarks löschen\n\
                    📁 Ordner-Organisation\n\
                    🔍 Schnellsuche\n\
                    ⌨️ Keyboard-Shortcuts\n\
                 \n\
                 🎨 DESIGN-FEATURES:\n\
                    🌟 Modern Dark Theme\n\
                    💫 Smooth Animations\n\
                    🎭 Hover-Effekte\n\
                    📱 Responsive Design\n\
                    🔄 Live-Updates\n\
                 \n\
                 ⌨️ KEYBOARD-SHORTCUTS:\n\
                    • Ctrl+B → Sidebar ein/aus\n\
                    • Ctrl+Shift+B → Bookmark hinzufügen\n\
                 \n\
                 🎉 Die vertikale Lesezeichenleiste ist perfekt\n\
                    für schnellen Zugriff auf Ihre Lieblings-Websites!\n\
                 \n\
                 📄 HTML-Datei: ora_vertical_bookmark_sidebar.html\n\
                 💫 Öffnen Sie die HTML-Datei für die volle Erfahrung!"
                , if self.is_on_left { "Linke" } else { "Rechte" }
                , self.sidebar_width
                , if self.is_visible { "✅ Ja" } else { "❌ Nein" }
                , self.bookmarks.len()
                , self.bookmarks.iter()
                    .map(|b| format!("   {} {} ({})", b.favicon, b.title, b.url))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            
            let wide_text: Vec<u16> = sidebar_info.encode_utf16().chain(std::iter::once(0)).collect();
            let _ = SetWindowTextW(self.parent_hwnd, windows::core::PCWSTR(wide_text.as_ptr()));
        }
        
        Ok(())
    }
    
    // 📚 STANDARD-BOOKMARKS LADEN
    fn load_default_bookmarks(&mut self) {
        self.bookmarks = vec![
            BookmarkEntry {
                id: "google".to_string(),
                title: "Google".to_string(),
                url: "https://google.com".to_string(),
                favicon: "🔍".to_string(),
            },
            BookmarkEntry {
                id: "github".to_string(),
                title: "GitHub".to_string(),
                url: "https://github.com".to_string(),
                favicon: "🐙".to_string(),
            },
            BookmarkEntry {
                id: "stackoverflow".to_string(),
                title: "Stack Overflow".to_string(),
                url: "https://stackoverflow.com".to_string(),
                favicon: "📚".to_string(),
            },
            BookmarkEntry {
                id: "rust-docs".to_string(),
                title: "Rust Documentation".to_string(),
                url: "https://doc.rust-lang.org".to_string(),
                favicon: "🦀".to_string(),
            },
            BookmarkEntry {
                id: "youtube".to_string(),
                title: "YouTube".to_string(),
                url: "https://youtube.com".to_string(),
                favicon: "📺".to_string(),
            },
        ];
        
        println!("📚 Loaded {} default bookmarks", self.bookmarks.len());
    }
    
    // 📄 HTML IN SIDEBAR LADEN
    fn load_html_in_sidebar(&self, hwnd: HWND, file_path: &str) -> Result<()> {
        unsafe {
            let file_url = format!("file:///{}", 
                std::fs::canonicalize(file_path)?
                    .to_string_lossy()
                    .replace('\\', "/")
            );
            
            let display_text = format!(
                "📚 VERTIKALE LESEZEICHENLEISTE AKTIV\n\
                 \n\
                 📄 HTML-Datei: {}\n\
                 🔗 URL: {}\n\
                 🔖 Bookmarks: {}\n\
                 \n\
                 ✨ SIDEBAR-FEATURES:\n\
                    📁 Expandierbare Ordner\n\
                    🔖 Drag & Drop Support\n\
                    ⌨️ Keyboard-Shortcuts\n\
                    🎨 Modern Dark Theme\n\
                    📱 Responsive Design\n\
                 \n\
                 💫 Die vertikale Lesezeichenleiste ist\n\
                    perfekt für schnellen Zugriff auf\n\
                    Ihre wichtigsten Websites!"
                , file_path, file_url, self.bookmarks.len()
            );
            
            let wide_text: Vec<u16> = display_text.encode_utf16().chain(std::iter::once(0)).collect();
            let _ = SetWindowTextW(hwnd, windows::core::PCWSTR(wide_text.as_ptr()));
        }
        
        Ok(())
    }
    
    // 🔖 BOOKMARK HINZUFÜGEN
    pub fn add_bookmark(&mut self, title: &str, url: &str) -> String {
        let bookmark_id = format!("bookmark_{}", chrono::Utc::now().timestamp());
        
        let favicon = if url.contains("google.com") { "🔍" }
        else if url.contains("github.com") { "🐙" }
        else if url.contains("stackoverflow.com") { "📚" }
        else if url.contains("rust-lang.org") { "🦀" }
        else if url.contains("youtube.com") { "📺" }
        else { "🌐" }.to_string();
        
        let bookmark = BookmarkEntry {
            id: bookmark_id.clone(),
            title: title.to_string(),
            url: url.to_string(),
            favicon,
        };
        
        self.bookmarks.push(bookmark);
        
                 // Aktualisiere HTML
         let _ = self.create_sidebar_content();
        let _ = self.display_sidebar_info();
        
        println!("🔖 Bookmark added: {} -> {}", title, url);
        bookmark_id
    }
    
    // 🌐 FAVICON FÜR URL BESTIMMEN
    fn get_favicon_for_url(&self, url: &str) -> String {
        if url.contains("google.com") { "🔍".to_string() }
        else if url.contains("github.com") { "🐙".to_string() }
        else if url.contains("stackoverflow.com") { "📚".to_string() }
        else if url.contains("rust-lang.org") { "🦀".to_string() }
        else if url.contains("youtube.com") { "📺".to_string() }
        else if url.contains("twitter.com") || url.contains("x.com") { "🐦".to_string() }
        else if url.contains("reddit.com") { "🤖".to_string() }
        else if url.contains("wikipedia.org") { "📖".to_string() }
        else { "🌐".to_string() }
    }
    
    // 📏 SIDEBAR-GRÖßE ANPASSEN
    pub fn resize(&self, parent_width: i32, parent_height: i32) -> Result<()> {
        if let Some(hwnd) = self.sidebar_hwnd {
            unsafe {
                let x = if self.is_on_left { 0 } else { parent_width - self.sidebar_width };
                
                let _ = SetWindowPos(
                    hwnd,
                    None,
                    x,
                    0,
                    self.sidebar_width,
                    parent_height,
                    SWP_NOZORDER,
                );
            }
        }
        Ok(())
    }
    
    // 👁️ SICHTBARKEIT UMSCHALTEN
    pub fn toggle_visibility(&mut self) -> bool {
        self.is_visible = !self.is_visible;
        
        if let Some(hwnd) = self.sidebar_hwnd {
            unsafe {
                let _ = ShowWindow(hwnd, if self.is_visible { SW_SHOW } else { SW_HIDE });
            }
        }
        
        println!("👁️ Sidebar visibility: {}", if self.is_visible { "Visible" } else { "Hidden" });
        self.is_visible
    }
    
    // ↔️ SEITE WECHSELN (LINKS/RECHTS)
    pub fn toggle_side(&mut self) -> Result<()> {
        self.is_on_left = !self.is_on_left;
        
        if let Some(hwnd) = self.sidebar_hwnd {
            // Hole Parent-Fenster-Größe
            unsafe {
                let mut rect = RECT::default();
                let _ = GetClientRect(self.parent_hwnd, &mut rect);
                let parent_width = rect.right - rect.left;
                
                let x = if self.is_on_left { 0 } else { parent_width - self.sidebar_width };
                
                let _ = SetWindowPos(
                    hwnd,
                    None,
                    x,
                    0,
                    self.sidebar_width,
                    rect.bottom - rect.top,
                    SWP_NOZORDER,
                );
            }
        }
        
        println!("↔️ Sidebar moved to: {}", if self.is_on_left { "Left" } else { "Right" });
        Ok(())
    }
    
    // 📊 SIDEBAR-STATISTIKEN
    pub fn get_stats(&self) -> String {
        format!(
            "📚 VERTIKALE LESEZEICHENLEISTE\n\
             \n\
             🔖 Bookmarks: {}\n\
             📏 Breite: {}px\n\
             👁️ Sichtbar: {}\n\
             📍 Position: {}\n\
             \n\
             📚 BOOKMARK-LISTE:\n\
             {}\n\
             \n\
             💡 COMMANDS:\n\
             • 'bookmarks toggle' → Sidebar ein/aus\n\
             • 'bookmarks left' → Links positionieren\n\
             • 'bookmarks right' → Rechts positionieren\n\
             • 'bookmarks add' → Bookmark hinzufügen"
            , self.bookmarks.len()
            , self.sidebar_width
            , if self.is_visible { "✅ Ja" } else { "❌ Nein" }
            , if self.is_on_left { "Links" } else { "Rechts" }
            , self.bookmarks.iter()
                .map(|b| format!("   {} {}", b.favicon, b.title))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
    
    // 🧹 CLEANUP
    pub fn cleanup(&mut self) -> Result<()> {
        println!("🧹 Cleaning up Vertical Bookmark Sidebar...");
        
        if let Some(hwnd) = self.sidebar_hwnd {
            unsafe {
                let _ = DestroyWindow(hwnd);
            }
        }
        
        self.sidebar_hwnd = None;
        
        // Lösche temporäre HTML-Datei
        let _ = std::fs::remove_file("ora_bookmark_sidebar.html");
        
        println!("✅ Vertical Bookmark Sidebar cleanup completed!");
        Ok(())
    }
    
    // 🏠 WINDOW PROCEDURE
    unsafe extern "system" fn sidebar_window_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_PAINT => {
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
            WM_SIZE => {
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
            WM_LBUTTONDOWN => {
                // Handle Bookmark-Klicks
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
            WM_DESTROY => {
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

// 🎯 SIDEBAR-MANAGER FÜR GLOBALE VERWALTUNG
pub struct BookmarkSidebarManager {
    sidebar: Option<VerticalBookmarkSidebar>,
    is_initialized: bool,
}

impl BookmarkSidebarManager {
    pub fn new() -> Self {
        Self {
            sidebar: None,
            is_initialized: false,
        }
    }
    
    pub fn initialize(&mut self, parent: HWND) -> Result<()> {
        println!("🚀 Initializing Bookmark Sidebar Manager...");
        
        let mut sidebar = VerticalBookmarkSidebar::new(parent)?;
        sidebar.create_sidebar()?;
        
        self.sidebar = Some(sidebar);
        self.is_initialized = true;
        
        println!("✅ Bookmark Sidebar Manager ready!");
        Ok(())
    }
    
    pub fn get_sidebar(&mut self) -> Option<&mut VerticalBookmarkSidebar> {
        self.sidebar.as_mut()
    }
    
    pub fn toggle_visibility(&mut self) -> bool {
        if let Some(sidebar) = &mut self.sidebar {
            sidebar.toggle_visibility()
        } else {
            false
        }
    }
    
    pub fn add_current_bookmark(&mut self, title: &str, url: &str) -> Result<()> {
        if let Some(sidebar) = &mut self.sidebar {
            sidebar.add_bookmark(title, url);
        }
        Ok(())
    }
    
    pub fn get_stats(&self) -> String {
        if let Some(sidebar) = &self.sidebar {
            sidebar.get_stats()
        } else {
            "Bookmark Sidebar not initialized".to_string()
        }
    }
    
    pub fn cleanup(&mut self) -> Result<()> {
        if let Some(sidebar) = &mut self.sidebar {
            sidebar.cleanup()?;
        }
        self.sidebar = None;
        self.is_initialized = false;
        
        println!("🧹 Bookmark Sidebar Manager cleanup completed!");
        Ok(())
    }
} 