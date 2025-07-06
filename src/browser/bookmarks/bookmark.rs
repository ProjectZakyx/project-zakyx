// ⭐ BOOKMARK SYSTEM - Individual Bookmark
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