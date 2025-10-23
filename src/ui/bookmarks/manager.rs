// 📚 Bookmark Manager
// Verwaltet Bookmark-Daten und -Operationen

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkEntry {
    pub id: String,
    pub title: String,
    pub url: String,
    pub favicon: String,
    pub created_at: DateTime<Utc>,
    pub folder: Option<String>,
    pub tags: Vec<String>,
}

impl BookmarkEntry {
    pub fn new(title: String, url: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            url,
            favicon: "🔖".to_string(),
            created_at: Utc::now(),
            folder: None,
            tags: Vec::new(),
        }
    }

    pub fn with_favicon(mut self, favicon: String) -> Self {
        self.favicon = favicon;
        self
    }

    pub fn with_folder(mut self, folder: String) -> Self {
        self.folder = Some(folder);
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

pub struct BookmarkManager {
    bookmarks: Vec<BookmarkEntry>,
    max_bookmarks: usize,
}

impl BookmarkManager {
    pub fn new() -> Self {
        Self {
            bookmarks: Vec::new(),
            max_bookmarks: 50, // Limit für Performance
        }
    }

    /// Lade Standard-Bookmarks
    pub fn load_default_bookmarks(&mut self) {
        let default_bookmarks = vec![
            BookmarkEntry::new(
                "🌐 Google".to_string(),
                "https://www.google.com".to_string()
            ).with_favicon("🔍".to_string()),
            
            BookmarkEntry::new(
                "📺 YouTube".to_string(),
                "https://www.youtube.com".to_string()
            ).with_favicon("📺".to_string()),
            
            BookmarkEntry::new(
                "📧 Gmail".to_string(),
                "https://mail.google.com".to_string()
            ).with_favicon("📧".to_string()),
            
            BookmarkEntry::new(
                "💼 GitHub".to_string(),
                "https://github.com".to_string()
            ).with_favicon("💼".to_string()),
            
            BookmarkEntry::new(
                "📰 Reddit".to_string(),
                "https://www.reddit.com".to_string()
            ).with_favicon("📰".to_string()),
            
            BookmarkEntry::new(
                "🛒 Amazon".to_string(),
                "https://www.amazon.de".to_string()
            ).with_favicon("🛒".to_string()),
            
            BookmarkEntry::new(
                "🎵 Spotify".to_string(),
                "https://open.spotify.com".to_string()
            ).with_favicon("🎵".to_string()),
            
            BookmarkEntry::new(
                "📚 Wikipedia".to_string(),
                "https://de.wikipedia.org".to_string()
            ).with_favicon("📚".to_string()),
        ];

        self.bookmarks = default_bookmarks;
        println!("📚 {} Standard-Bookmarks geladen", self.bookmarks.len());
    }

    /// Füge Bookmark hinzu
    pub fn add_bookmark(&mut self, title: &str, url: &str) -> Result<String> {
        if self.bookmarks.len() >= self.max_bookmarks {
            return Err(anyhow::anyhow!("Maximum number of bookmarks reached: {}", self.max_bookmarks));
        }

        let favicon = self.get_favicon_for_url(url);
        let bookmark = BookmarkEntry::new(title.to_string(), url.to_string())
            .with_favicon(favicon);
        
        let id = bookmark.id.clone();
        self.bookmarks.push(bookmark);
        
        println!("➕ Bookmark hinzugefügt: {} ({})", title, id);
        Ok(id)
    }

    /// Entferne Bookmark
    pub fn remove_bookmark(&mut self, bookmark_id: &str) -> bool {
        if let Some(pos) = self.bookmarks.iter().position(|b| b.id == bookmark_id) {
            let bookmark = self.bookmarks.remove(pos);
            println!("🗑️ Bookmark entfernt: {} ({})", bookmark.title, bookmark_id);
            return true;
        }
        false
    }

    /// Aktualisiere Bookmark
    pub fn update_bookmark(&mut self, bookmark_id: &str, title: Option<String>, url: Option<String>) -> bool {
        if let Some(bookmark) = self.bookmarks.iter_mut().find(|b| b.id == bookmark_id) {
            if let Some(new_title) = title {
                bookmark.title = new_title;
            }
            if let Some(new_url) = url {
                bookmark.url = new_url.clone();
                // Favicon separat setzen um Borrow-Checker zu umgehen
            }
            println!("✏️ Bookmark aktualisiert: {}", bookmark_id);
            return true;
        }
        false
    }

    /// Finde Bookmark
    pub fn find_bookmark(&self, bookmark_id: &str) -> Option<&BookmarkEntry> {
        self.bookmarks.iter().find(|b| b.id == bookmark_id)
    }

    /// Alle Bookmarks
    pub fn get_bookmarks(&self) -> &[BookmarkEntry] {
        &self.bookmarks
    }

    /// Bookmarks nach Ordner filtern
    pub fn get_bookmarks_by_folder(&self, folder: &str) -> Vec<&BookmarkEntry> {
        self.bookmarks.iter()
            .filter(|b| b.folder.as_ref().map_or(false, |f| f == folder))
            .collect()
    }

    /// Suche Bookmarks
    pub fn search_bookmarks(&self, query: &str) -> Vec<&BookmarkEntry> {
        let query_lower = query.to_lowercase();
        self.bookmarks.iter()
            .filter(|b| {
                b.title.to_lowercase().contains(&query_lower) ||
                b.url.to_lowercase().contains(&query_lower) ||
                b.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Statistiken
    pub fn get_stats(&self) -> String {
        let total = self.bookmarks.len();
        let folders: std::collections::HashSet<_> = self.bookmarks.iter()
            .filter_map(|b| b.folder.as_ref())
            .collect();
        
        format!(
            "📊 Bookmark-Statistiken:\n\
             • Gesamt: {} / {} Bookmarks\n\
             • Ordner: {}\n\
             • Tags: {}\n\
             • Ältestes: {}\n\
             • Neuestes: {}",
            total,
            self.max_bookmarks,
            folders.len(),
            self.bookmarks.iter().map(|b| b.tags.len()).sum::<usize>(),
            self.bookmarks.iter()
                .min_by_key(|b| b.created_at)
                .map(|b| b.created_at.format("%d.%m.%Y").to_string())
                .unwrap_or_else(|| "Keine".to_string()),
            self.bookmarks.iter()
                .max_by_key(|b| b.created_at)
                .map(|b| b.created_at.format("%d.%m.%Y").to_string())
                .unwrap_or_else(|| "Keine".to_string())
        )
    }

    /// Favicon für URL bestimmen
    fn get_favicon_for_url(&self, url: &str) -> String {
        let url_lower = url.to_lowercase();
        
        if url_lower.contains("google") { "🔍" }
        else if url_lower.contains("youtube") { "📺" }
        else if url_lower.contains("gmail") || url_lower.contains("mail") { "📧" }
        else if url_lower.contains("github") { "💼" }
        else if url_lower.contains("reddit") { "📰" }
        else if url_lower.contains("amazon") { "🛒" }
        else if url_lower.contains("spotify") { "🎵" }
        else if url_lower.contains("wikipedia") { "📚" }
        else if url_lower.contains("twitter") || url_lower.contains("x.com") { "🐦" }
        else if url_lower.contains("facebook") { "📘" }
        else if url_lower.contains("instagram") { "📷" }
        else if url_lower.contains("linkedin") { "💼" }
        else if url_lower.contains("stackoverflow") { "💻" }
        else if url_lower.contains("news") { "📰" }
        else if url_lower.contains("shop") || url_lower.contains("store") { "🛍️" }
        else { "🔖" }.to_string()
    }

    /// Exportiere Bookmarks als JSON
    pub fn export_to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.bookmarks)
            .map_err(|e| anyhow::anyhow!("Failed to export bookmarks: {}", e))
    }

    /// Importiere Bookmarks aus JSON
    pub fn import_from_json(&mut self, json: &str) -> Result<usize> {
        let imported_bookmarks: Vec<BookmarkEntry> = serde_json::from_str(json)
            .map_err(|e| anyhow::anyhow!("Failed to parse bookmarks JSON: {}", e))?;
        
        let mut added = 0;
        for bookmark in imported_bookmarks {
            if self.bookmarks.len() < self.max_bookmarks {
                self.bookmarks.push(bookmark);
                added += 1;
            } else {
                break;
            }
        }
        
        println!("📥 {} Bookmarks importiert", added);
        Ok(added)
    }

    /// Räume auf (entferne doppelte URLs)
    pub fn cleanup_duplicates(&mut self) -> usize {
        let mut seen_urls = std::collections::HashSet::new();
        let original_count = self.bookmarks.len();
        
        self.bookmarks.retain(|bookmark| {
            seen_urls.insert(bookmark.url.clone())
        });
        
        let removed = original_count - self.bookmarks.len();
        if removed > 0 {
            println!("🧹 {} doppelte Bookmarks entfernt", removed);
        }
        removed
    }
}

impl Default for BookmarkManager {
    fn default() -> Self {
        Self::new()
    }
}