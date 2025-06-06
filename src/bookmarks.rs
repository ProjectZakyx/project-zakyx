use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub title: String,
    pub url: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookmarkManager {
    pub bookmarks: Vec<Bookmark>,
    file_path: String,
}

impl BookmarkManager {
    pub fn new() -> Self {
        let file_path = "bookmarks.json".to_string();
        let mut manager = Self {
            bookmarks: Vec::new(),
            file_path,
        };
        
        // Lade bestehende Lesezeichen
        if let Err(e) = manager.load_bookmarks() {
            println!("⚠️  Couldn't load existing bookmarks: {:?}", e);
            println!("📝 Starting with empty bookmark list");
        }
        
        manager
    }

    pub fn add_bookmark(&mut self, title: String, url: String) -> Result<()> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        // Prüfe ob URL bereits existiert
        if self.bookmarks.iter().any(|b| b.url == url) {
            println!("⚠️  Bookmark already exists for URL: {}", url);
            return Ok(());
        }
        
        let bookmark = Bookmark {
            title: if title.trim().is_empty() { url.clone() } else { title },
            url,
            created_at: now,
        };
        
        self.bookmarks.push(bookmark.clone());
        self.save_bookmarks()?;
        
        println!("⭐ Bookmark added: {} -> {}", bookmark.title, bookmark.url);
        Ok(())
    }

    pub fn remove_bookmark(&mut self, index: usize) -> Result<()> {
        if index < self.bookmarks.len() {
            let removed = self.bookmarks.remove(index);
            self.save_bookmarks()?;
            println!("🗑️  Bookmark removed: {}", removed.title);
        }
        Ok(())
    }

    pub fn get_bookmarks(&self) -> &Vec<Bookmark> {
        &self.bookmarks
    }

    pub fn get_bookmark_by_index(&self, index: usize) -> Option<&Bookmark> {
        self.bookmarks.get(index)
    }

    fn load_bookmarks(&mut self) -> Result<()> {
        if Path::new(&self.file_path).exists() {
            let content = fs::read_to_string(&self.file_path)?;
            self.bookmarks = serde_json::from_str(&content)?;
            println!("📚 Loaded {} bookmarks from {}", self.bookmarks.len(), self.file_path);
        }
        Ok(())
    }

    pub fn save_bookmarks(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.bookmarks)?;
        fs::write(&self.file_path, content)?;
        println!("💾 Saved {} bookmarks to {}", self.bookmarks.len(), self.file_path);
        Ok(())
    }

    pub fn search_bookmarks(&self, query: &str) -> Vec<(usize, &Bookmark)> {
        let query_lower = query.to_lowercase();
        self.bookmarks
            .iter()
            .enumerate()
            .filter(|(_, bookmark)| {
                bookmark.title.to_lowercase().contains(&query_lower) ||
                bookmark.url.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
}

impl Default for BookmarkManager {
    fn default() -> Self {
        Self::new()
    }
} 