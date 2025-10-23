// 📚 HISTORY SYSTEM - History Manager
use super::entry::HistoryEntry;
use anyhow::Result;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

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
    pub fn get_recent_history(&self, limit: usize) -> Vec<&HistoryEntry> {
        self.history.iter().take(limit).collect()
    }

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
    pub fn clear_history(&mut self) {
        self.history.clear();
        self.next_history_id = 1;
        println!("🧹 History cleared");
    }

    #[allow(dead_code)]
    pub fn get_history_count(&self) -> usize {
        self.history.len()
    }

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
