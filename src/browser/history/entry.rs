// 📚 HISTORY SYSTEM - Individual History Entry
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HistoryEntry {
    pub id: u32,
    pub url: String,
    pub title: String,
    pub visited_at: String,
    pub visit_count: u32,
} 