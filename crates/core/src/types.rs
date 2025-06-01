pub struct BrowserSettings {
    pub window_size: (u32, u32),
    pub window_position: (i32, i32),
    pub user_agent: String,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            window_size: (1200, 800),
            window_position: (100, 100),
            user_agent: String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"),
        }
    }
}

#[derive(Debug)]
pub enum BrowserEvent {
    NavigationStarted(String),
    NavigationCompleted(String),
    Error(String),
    Close,
}
