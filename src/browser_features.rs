use anyhow::Result;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use windows::Win32::Foundation::HWND;

// 📂 TAB MANAGEMENT SYSTEM
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BrowserTab {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub active: bool,
    pub loading: bool,
    pub favicon: Option<String>,
    pub history_position: usize,
    pub history: Vec<String>,
}

impl BrowserTab {
    #[allow(dead_code)]
    pub fn new(id: u32, url: &str) -> Self {
        BrowserTab {
            id,
            title: format!("Tab {}", id),
            url: url.to_string(),
            active: false,
            loading: false,
            favicon: None,
            history_position: 0,
            history: vec![url.to_string()],
        }
    }

    #[allow(dead_code)]
    pub fn navigate_to(&mut self, url: &str) {
        // Add to history if different from current
        if self.history.is_empty() || self.history[self.history_position] != url {
            // Remove any forward history when navigating to new URL
            self.history.truncate(self.history_position + 1);
            self.history.push(url.to_string());
            self.history_position = self.history.len() - 1;
        }
        
        self.url = url.to_string();
        self.loading = true;
        self.update_title_from_url(url);
    }

    #[allow(dead_code)]
    pub fn can_go_back(&self) -> bool {
        self.history_position > 0
    }

    #[allow(dead_code)]
    pub fn can_go_forward(&self) -> bool {
        self.history_position < self.history.len() - 1
    }

    #[allow(dead_code)]
    pub fn go_back(&mut self) -> Option<String> {
        if self.can_go_back() {
            self.history_position -= 1;
            let url = self.history[self.history_position].clone();
            self.url = url.clone();
            self.update_title_from_url(&url);
            Some(url)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn go_forward(&mut self) -> Option<String> {
        if self.can_go_forward() {
            self.history_position += 1;
            let url = self.history[self.history_position].clone();
            self.url = url.clone();
            self.update_title_from_url(&url);
            Some(url)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn update_title_from_url(&mut self, url: &str) {
        self.title = if url == "gui" || url == "home" {
            "🏠 HTML GUI".to_string()
        } else if url.contains("google") {
            "🔍 Google".to_string()
        } else if url.contains("github") {
            "👨‍💻 GitHub".to_string()
        } else if url.contains("wikipedia") {
            "📖 Wikipedia".to_string()
        } else if url.starts_with("http") {
            url.replace("https://", "").replace("http://", "").split('/').next().unwrap_or(url).to_string()
        } else {
            format!("📄 {}", url)
        };
    }

    #[allow(dead_code)]
    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }

    #[allow(dead_code)]
    pub fn set_title(&mut self, title: &str) {
        if !title.trim().is_empty() {
            self.title = title.to_string();
        }
    }
}

// 📂 TAB MANAGER
#[derive(Debug)]
#[allow(dead_code)]
pub struct TabManager {
    tabs: Vec<BrowserTab>,
    active_tab_id: Option<u32>,
    next_tab_id: u32,
    max_tabs: usize,
}

impl TabManager {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut manager = TabManager {
            tabs: Vec::new(),
            active_tab_id: None,
            next_tab_id: 1,
            max_tabs: 10,
        };

        // Create initial tab
        manager.create_new_tab("google.com");
        manager
    }

    #[allow(dead_code)]
    pub fn create_new_tab(&mut self, url: &str) -> u32 {
        if self.tabs.len() >= self.max_tabs {
            println!("⚠️ Maximum tabs ({}) reached!", self.max_tabs);
            return 0;
        }

        let tab_id = self.next_tab_id;
        self.next_tab_id += 1;

        let mut new_tab = BrowserTab::new(tab_id, url);
        
        // Deactivate all other tabs
        for tab in &mut self.tabs {
            tab.active = false;
        }

        new_tab.active = true;
        self.tabs.push(new_tab);
        self.active_tab_id = Some(tab_id);

        println!("📂 New tab created: {} ({})", tab_id, url);
        tab_id
    }

    #[allow(dead_code)]
    pub fn close_tab(&mut self, tab_id: u32) -> bool {
        if self.tabs.len() <= 1 {
            println!("⚠️ Cannot close last tab!");
            return false;
        }

        if let Some(pos) = self.tabs.iter().position(|t| t.id == tab_id) {
            let was_active = self.tabs[pos].active;
            self.tabs.remove(pos);

            if was_active {
                // Activate the tab to the left, or the first tab if none to the left
                let new_active_pos = if pos > 0 { pos - 1 } else { 0 };
                if let Some(tab) = self.tabs.get_mut(new_active_pos) {
                    tab.active = true;
                    self.active_tab_id = Some(tab.id);
                }
            }

            println!("🗑️ Tab closed: {}", tab_id);
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn switch_to_tab(&mut self, tab_id: u32) -> Option<&BrowserTab> {
        // Deactivate all tabs
        for tab in &mut self.tabs {
            tab.active = false;
        }

        // Activate the selected tab
        if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
            tab.active = true;
            self.active_tab_id = Some(tab_id);
            println!("🔄 Switched to tab: {} ({})", tab_id, tab.url);
            Some(tab)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn get_active_tab(&mut self) -> Option<&mut BrowserTab> {
        self.active_tab_id.and_then(|id| self.tabs.iter_mut().find(|t| t.id == id))
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn get_active_tab_readonly(&self) -> Option<&BrowserTab> {
        self.active_tab_id.and_then(|id| self.tabs.iter().find(|t| t.id == id))
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn get_all_tabs(&self) -> &Vec<BrowserTab> {
        &self.tabs
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn navigate_active_tab(&mut self, url: &str) -> Option<&BrowserTab> {
        if let Some(tab) = self.get_active_tab() {
            tab.navigate_to(url);
            Some(tab)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn go_back_active_tab(&mut self) -> Option<String> {
        self.get_active_tab().and_then(|tab| tab.go_back())
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn go_forward_active_tab(&mut self) -> Option<String> {
        self.get_active_tab().and_then(|tab| tab.go_forward())
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn get_tab_count(&self) -> usize {
        self.tabs.len()
    }
}

// ⭐ BOOKMARK SYSTEM
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Bookmark {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub folder: String,
    pub favicon: Option<String>,
    pub created_at: String,
    pub tags: Vec<String>,
}

impl Bookmark {
    pub fn new(id: u32, title: &str, url: &str) -> Self {
        Bookmark {
            id,
            title: title.to_string(),
            url: url.to_string(),
            folder: "Default".to_string(),
            favicon: None,
            created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            tags: Vec::new(),
        }
    }
}

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
    #[allow(dead_code)]
    pub fn find_bookmark_by_url(&self, url: &str) -> Option<&Bookmark> {
        self.bookmarks.iter().find(|b| b.url == url)
    }

    #[allow(dead_code)]
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

// 📚 HISTORY SYSTEM
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HistoryEntry {
    pub id: u32,
    pub url: String,
    pub title: String,
    pub visited_at: String,
    pub visit_count: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct HistoryManager {
    history: Vec<HistoryEntry>,
    next_history_id: u32,
    history_file: PathBuf,
    max_history_entries: usize,
}

#[allow(dead_code)]
impl HistoryManager {
    #[allow(dead_code)]
    pub fn new() -> Result<Self> {
        let history_file = std::env::current_dir()?.join("history.json");
        let mut manager = HistoryManager {
            history: Vec::new(),
            next_history_id: 1,
            history_file,
            max_history_entries: 1000,
        };

        manager.load_history()?;
        Ok(manager)
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn add_history_entry(&mut self, url: &str, title: &str) {
        // Check if URL already exists in recent history
        if let Some(entry) = self.history.iter_mut().find(|h| h.url == url) {
            entry.visit_count += 1;
            entry.visited_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            entry.title = title.to_string();
        } else {
            let entry = HistoryEntry {
                id: self.next_history_id,
                url: url.to_string(),
                title: title.to_string(),
                visited_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                visit_count: 1,
            };

            self.next_history_id += 1;
            self.history.insert(0, entry); // Add to beginning for chronological order

            // Limit history size
            if self.history.len() > self.max_history_entries {
                self.history.truncate(self.max_history_entries);
            }
        }

        println!("📚 History entry added: {} -> {}", title, url);
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn get_recent_history(&self, limit: usize) -> Vec<&HistoryEntry> {
        self.history.iter().take(limit).collect()
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn search_history(&self, query: &str) -> Vec<&HistoryEntry> {
        let query_lower = query.to_lowercase();
        self.history.iter()
            .filter(|entry| 
                entry.url.to_lowercase().contains(&query_lower) ||
                entry.title.to_lowercase().contains(&query_lower)
            )
            .take(10)
            .collect()
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn clear_history(&mut self) {
        self.history.clear();
        self.next_history_id = 1;
        println!("🧹 History cleared");
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn get_history_count(&self) -> usize {
        self.history.len()
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    fn load_history(&mut self) -> Result<()> {
        if self.history_file.exists() {
            let content = fs::read_to_string(&self.history_file)?;
            if let Ok(data) = serde_json::from_str::<Value>(&content) {
                if let Some(history_array) = data["history"].as_array() {
                    for history_data in history_array {
                        if let (Some(url), Some(title), Some(visited_at)) = (
                            history_data["url"].as_str(),
                            history_data["title"].as_str(),
                            history_data["visited_at"].as_str()
                        ) {
                            let entry = HistoryEntry {
                                id: self.next_history_id,
                                url: url.to_string(),
                                title: title.to_string(),
                                visited_at: visited_at.to_string(),
                                visit_count: history_data["visit_count"].as_u64().unwrap_or(1) as u32,
                            };
                            self.next_history_id += 1;
                            self.history.push(entry);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn save_history(&self) -> Result<()> {
        let history_data: Vec<Value> = self.history.iter().map(|h| {
            json!({
                "id": h.id,
                "url": h.url,
                "title": h.title,
                "visited_at": h.visited_at,
                "visit_count": h.visit_count
            })
        }).collect();

        let data = json!({
            "history": history_data,
            "next_id": self.next_history_id
        });

        fs::write(&self.history_file, serde_json::to_string_pretty(&data)?)?;
        println!("💾 History saved to file");
        Ok(())
    }
}

// 🚀 BROWSER FEATURES MANAGER
#[derive(Debug)]
#[allow(dead_code)]
pub struct BrowserFeaturesManager {
    pub tab_manager: TabManager,
    pub bookmark_manager: BookmarkManager,
    pub history_manager: HistoryManager,
    hwnd: HWND,
}

#[allow(dead_code)]
impl BrowserFeaturesManager {
    #[allow(dead_code)]
    pub fn new(hwnd: HWND) -> Result<Self> {
        Ok(BrowserFeaturesManager {
            tab_manager: TabManager::new(),
            bookmark_manager: BookmarkManager::new()?,
            history_manager: HistoryManager::new()?,
            hwnd,
        })
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn navigate_to_url(&mut self, url: &str) -> Result<()> {
        // Add to history
        let title = if let Some(tab) = self.tab_manager.get_active_tab_readonly() {
            tab.title.clone()
        } else {
            url.to_string()
        };

        self.history_manager.add_history_entry(url, &title);

        // Navigate active tab
        if let Some(tab) = self.tab_manager.navigate_active_tab(url) {
            println!("🧭 Navigation: {} -> {}", tab.title, url);
        }

        Ok(())
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn add_bookmark_current_tab(&mut self) -> Result<Option<u32>> {
        if let Some(tab) = self.tab_manager.get_active_tab_readonly() {
            let bookmark_id = self.bookmark_manager.add_bookmark(&tab.title, &tab.url);
            self.bookmark_manager.save_bookmarks()?;
            Ok(Some(bookmark_id))
        } else {
            Ok(None)
        }
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn get_status_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();

        info.insert("tab_count".to_string(), self.tab_manager.get_tab_count().to_string());
        info.insert("bookmark_count".to_string(), self.bookmark_manager.get_bookmark_count().to_string());
        info.insert("history_count".to_string(), self.history_manager.get_history_count().to_string());

        if let Some(tab) = self.tab_manager.get_active_tab_readonly() {
            info.insert("current_url".to_string(), tab.url.clone());
            info.insert("current_title".to_string(), tab.title.clone());
            info.insert("can_go_back".to_string(), tab.can_go_back().to_string());
            info.insert("can_go_forward".to_string(), tab.can_go_forward().to_string());
        }

        info
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn cleanup(&self) -> Result<()> {
        self.bookmark_manager.save_bookmarks()?;
        self.history_manager.save_history()?;
        println!("🧹 Browser features cleaned up");
        Ok(())
    }
}

// 🎯 URL SUGGESTIONS SYSTEM
#[derive(Debug)]
#[allow(dead_code)]
pub struct UrlSuggestionEngine {
    bookmark_manager: *const BookmarkManager,
    history_manager: *const HistoryManager,
}

#[allow(dead_code)]
impl UrlSuggestionEngine {
    #[allow(dead_code)]
    pub fn new(bookmark_manager: &BookmarkManager, history_manager: &HistoryManager) -> Self {
        UrlSuggestionEngine {
            bookmark_manager,
            history_manager,
        }
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn get_suggestions(&self, query: &str) -> Vec<String> {
        let mut suggestions = Vec::new();

        if query.is_empty() {
            return suggestions;
        }

        // Add static suggestions
        let static_suggestions = vec![
            "google.com",
            "github.com", 
            "wikipedia.org",
            "gui",
            "webview"
        ];

        for suggestion in static_suggestions {
            if suggestion.contains(query) {
                suggestions.push(suggestion.to_string());
            }
        }

        unsafe {
            // Add bookmark suggestions
            if !self.bookmark_manager.is_null() {
                let bookmarks = (*self.bookmark_manager).get_bookmarks();
                for bookmark in bookmarks {
                    if bookmark.url.to_lowercase().contains(&query.to_lowercase()) ||
                       bookmark.title.to_lowercase().contains(&query.to_lowercase()) {
                        if !suggestions.contains(&bookmark.url) {
                            suggestions.push(bookmark.url.clone());
                        }
                    }
                }
            }

            // Add history suggestions
            if !self.history_manager.is_null() {
                let recent_history = (*self.history_manager).get_recent_history(10);
                for entry in recent_history {
                    if entry.url.to_lowercase().contains(&query.to_lowercase()) ||
                       entry.title.to_lowercase().contains(&query.to_lowercase()) {
                        if !suggestions.contains(&entry.url) {
                            suggestions.push(entry.url.clone());
                        }
                    }
                }
            }
        }

        suggestions.truncate(5); // Limit to 5 suggestions
        suggestions
    }
} 