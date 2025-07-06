// 📂 TAB MANAGEMENT - Individual Tab
use anyhow::Result;

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