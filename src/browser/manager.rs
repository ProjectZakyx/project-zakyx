// 🚀 BROWSER FEATURES MANAGER - Central Coordinator
use super::tabs::TabManager;
use super::bookmarks::BookmarkManager;
use super::history::HistoryManager;
use anyhow::Result;
use std::collections::HashMap;
use windows::Win32::Foundation::HWND;

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
    pub fn cleanup(&self) -> Result<()> {
        self.bookmark_manager.save_bookmarks()?;
        self.history_manager.save_history()?;
        println!("🧹 Browser features cleaned up");
        Ok(())
    }
} 