// 🎨 HTML Generator für Bookmark Toolbar
// Generiert HTML-Inhalte für die Lesezeichenleiste

use crate::ui::bookmarks::BookmarkEntry;

pub struct BookmarkHtmlGenerator;

impl BookmarkHtmlGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Generiere HTML für die komplette Toolbar
    pub fn generate_toolbar_html(&self, bookmarks: &[BookmarkEntry]) -> String {
        let bookmarks_html = self.generate_bookmarks_html(bookmarks);
        
        format!(r#"<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Horizontale Lesezeichenleiste</title>
    <style>
        {}
    </style>
</head>
<body>
    <div class="toolbar-header">
        <div class="toolbar-title">📚 Lesezeichen</div>
        <div class="bookmark-count">({} Bookmarks)</div>
    </div>
    
    <div class="bookmarks-container">
        {}
    </div>
    
    <div class="add-bookmark" onclick="addNewBookmark()">
        <span>➕</span>
        <span class="add-bookmark-text">Hinzufügen</span>
    </div>
    
    <script>
        {}
    </script>
</body>
</html>"#, 
            self.generate_css(),
            bookmarks.len(),
            bookmarks_html,
            self.generate_javascript()
        )
    }

    /// Generiere HTML für einzelne Bookmarks
    fn generate_bookmarks_html(&self, bookmarks: &[BookmarkEntry]) -> String {
        bookmarks.iter()
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
            "#, 
                bookmark.id, bookmark.url, bookmark.favicon, 
                bookmark.title, bookmark.url, 
                bookmark.id, bookmark.id
            ))
            .collect::<Vec<_>>()
            .join("")
    }

    /// Generiere CSS-Styles
    fn generate_css(&self) -> String {
        r#"
        * { margin: 0; padding: 0; box-sizing: border-box; }
        
        body {
            font-family: 'Segoe UI', sans-serif;
            background: linear-gradient(90deg, #2D2D30 0%, #252526 100%);
            color: #CCCCCC;
            height: 60px;
            width: 100vw;
            overflow-x: auto;
            overflow-y: hidden;
            padding: 8px 15px;
            position: fixed;
            left: 0;
            top: 0;
            border-bottom: 2px solid #3C3C3C;
            box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
            display: flex;
            align-items: center;
            gap: 15px;
        }
        
        .toolbar-header {
            background: rgba(0, 122, 204, 0.1);
            border-radius: 8px;
            padding: 8px 12px;
            border: 1px solid rgba(0, 122, 204, 0.3);
            display: flex;
            align-items: center;
            gap: 8px;
            flex-shrink: 0;
        }
        
        .toolbar-title {
            font-size: 12px;
            font-weight: 600;
            color: #FFFFFF;
        }
        
        .bookmark-count {
            font-size: 10px;
            color: #888888;
        }
        
        .bookmarks-container {
            display: flex;
            align-items: center;
            gap: 8px;
            flex: 1;
            overflow-x: auto;
            overflow-y: hidden;
        }
        
        .bookmarks-container::-webkit-scrollbar {
            height: 4px;
        }
        
        .bookmarks-container::-webkit-scrollbar-track {
            background: rgba(255, 255, 255, 0.1);
            border-radius: 2px;
        }
        
        .bookmarks-container::-webkit-scrollbar-thumb {
            background: rgba(0, 122, 204, 0.6);
            border-radius: 2px;
        }
        
        .bookmark-item {
            background: rgba(255, 255, 255, 0.08);
            border: 1px solid rgba(255, 255, 255, 0.15);
            border-radius: 8px;
            padding: 6px 10px;
            cursor: pointer;
            transition: all 0.2s ease;
            flex-shrink: 0;
            height: 36px;
            display: flex;
            align-items: center;
            gap: 8px;
            max-width: 200px;
            position: relative;
        }
        
        .bookmark-item:hover {
            background: rgba(0, 122, 204, 0.2);
            border-color: rgba(0, 122, 204, 0.4);
            transform: translateY(-2px);
        }
        
        .bookmark-favicon {
            font-size: 14px;
            flex-shrink: 0;
        }
        
        .bookmark-content {
            flex: 1;
            overflow: hidden;
        }
        
        .bookmark-title {
            font-size: 11px;
            font-weight: 500;
            color: #FFFFFF;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }
        
        .bookmark-url {
            font-size: 9px;
            color: #888888;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }
        
        .bookmark-actions {
            display: none;
            gap: 4px;
        }
        
        .bookmark-item:hover .bookmark-actions {
            display: flex;
        }
        
        .action-btn {
            background: none;
            border: none;
            cursor: pointer;
            font-size: 10px;
            padding: 2px 4px;
            border-radius: 3px;
            transition: background 0.2s ease;
        }
        
        .action-btn:hover {
            background: rgba(255, 255, 255, 0.1);
        }
        
        .add-bookmark {
            background: rgba(0, 122, 204, 0.2);
            border: 1px solid rgba(0, 122, 204, 0.4);
            border-radius: 6px;
            padding: 6px 10px;
            cursor: pointer;
            transition: all 0.2s ease;
            flex-shrink: 0;
            height: 36px;
            display: flex;
            align-items: center;
            gap: 6px;
        }
        
        .add-bookmark:hover {
            background: rgba(0, 122, 204, 0.3);
            transform: translateY(-2px);
        }
        
        .add-bookmark-text {
            color: #FFFFFF;
            font-size: 11px;
            font-weight: 500;
        }
        "#.to_string()
    }

    /// Generiere JavaScript-Funktionen
    fn generate_javascript(&self) -> String {
        r#"
        function openBookmark(id, url) {
            console.log('📚 Opening bookmark:', id, url);
            if (window.external && window.external.invoke) {
                window.external.invoke('open_bookmark', { id: id, url: url });
            } else {
                window.open(url, '_blank');
            }
        }
        
        function editBookmark(id) {
            console.log('✏️ Editing bookmark:', id);
            if (window.external && window.external.invoke) {
                window.external.invoke('edit_bookmark', { id: id });
            }
        }
        
        function deleteBookmark(id) {
            console.log('🗑️ Deleting bookmark:', id);
            if (confirm('Bookmark wirklich löschen?')) {
                if (window.external && window.external.invoke) {
                    window.external.invoke('delete_bookmark', { id: id });
                }
            }
        }
        
        function addNewBookmark() {
            console.log('➕ Adding new bookmark');
            if (window.external && window.external.invoke) {
                window.external.invoke('add_bookmark', {});
            }
        }
        
        // Keyboard shortcuts
        document.addEventListener('keydown', function(e) {
            if (e.ctrlKey && e.key === 'd') {
                e.preventDefault();
                addNewBookmark();
            }
        });
        
        console.log('📚 Bookmark Toolbar JavaScript initialized');
        "#.to_string()
    }
}

impl Default for BookmarkHtmlGenerator {
    fn default() -> Self {
        Self::new()
    }
}