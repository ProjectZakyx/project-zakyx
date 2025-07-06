// ⭐ BOOKMARK SYSTEM - Bookmark Manager
use super::bookmark::Bookmark;
use anyhow::Result;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct BookmarkManager {
    bookmarks: Vec<Bookmark>,
    next_bookmark_id: u32,
    bookmarks_file: PathBuf,
}

impl BookmarkManager {
    pub fn new() -> Result<Self> {
        let bookmarks_file = std::env::current_dir()?.join("bookmarks.json");
        let mut manager = BookmarkManager {
            bookmarks: Vec::new(),
            next_bookmark_id: 1,
            bookmarks_file,
        };

        manager.load_bookmarks()?;
        manager.add_default_bookmarks();
        Ok(manager)
    }

    fn add_default_bookmarks(&mut self) {
        if self.bookmarks.is_empty() {
            self.add_bookmark("🔍 Google", "https://google.com");
            self.add_bookmark("👨‍💻 GitHub", "https://github.com");
            self.add_bookmark("📖 Wikipedia", "https://wikipedia.org");
            self.add_bookmark("🏠 HTML GUI", "gui");
            let _ = self.save_bookmarks();
        }
    }

    pub fn add_bookmark(&mut self, title: &str, url: &str) -> u32 {
        let bookmark_id = self.next_bookmark_id;
        self.next_bookmark_id += 1;

        let bookmark = Bookmark::new(bookmark_id, title, url);
        self.bookmarks.push(bookmark);
        
        println!("⭐ Bookmark added: {} -> {}", title, url);
        bookmark_id
    }

    pub fn remove_bookmark(&mut self, bookmark_id: u32) -> bool {
        if let Some(pos) = self.bookmarks.iter().position(|b| b.id == bookmark_id) {
            let bookmark = self.bookmarks.remove(pos);
            println!("🗑️ Bookmark removed: {}", bookmark.title);
            true
        } else {
            false
        }
    }

    pub fn get_bookmarks(&self) -> &Vec<Bookmark> {
        &self.bookmarks
    }

    #[allow(dead_code)]
    pub fn find_bookmark_by_url(&self, url: &str) -> Option<&Bookmark> {
        self.bookmarks.iter().find(|b| b.url == url)
    }

    #[allow(dead_code)]
    pub fn get_bookmark_count(&self) -> usize {
        self.bookmarks.len()
    }

    fn load_bookmarks(&mut self) -> Result<()> {
        if self.bookmarks_file.exists() {
            let content = fs::read_to_string(&self.bookmarks_file)?;
            if let Ok(data) = serde_json::from_str::<Value>(&content) {
                if let Some(bookmarks_array) = data["bookmarks"].as_array() {
                    for bookmark_data in bookmarks_array {
                        if let (Some(title), Some(url)) = (
                            bookmark_data["title"].as_str(),
                            bookmark_data["url"].as_str()
                        ) {
                            self.add_bookmark(title, url);
                        }
                    }
                }
                if let Some(next_id) = data["next_id"].as_u64() {
                    self.next_bookmark_id = next_id as u32;
                }
            }
        }
        Ok(())
    }

    pub fn save_bookmarks(&self) -> Result<()> {
        let bookmarks_data: Vec<Value> = self.bookmarks.iter().map(|b| {
            json!({
                "id": b.id,
                "title": b.title,
                "url": b.url,
                "folder": b.folder,
                "created_at": b.created_at,
                "tags": b.tags
            })
        }).collect();

        let data = json!({
            "bookmarks": bookmarks_data,
            "next_id": self.next_bookmark_id
        });

        fs::write(&self.bookmarks_file, serde_json::to_string_pretty(&data)?)?;
        println!("💾 Bookmarks saved to file");
        Ok(())
    }
} 