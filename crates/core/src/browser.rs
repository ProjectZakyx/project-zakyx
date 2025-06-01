use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSettings {
    pub window_size: (u32, u32),
    pub window_position: (i32, i32),
    pub user_agent: String,
    pub enable_devtools: bool,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            window_size: (1200, 800),
            window_position: (100, 100),
            user_agent: String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"),
            enable_devtools: false,
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

pub trait Browser: Send + Sync {
    fn new(settings: BrowserSettings) -> Result<Arc<Self>>;
    fn run(&self) -> Result<()>;
    fn navigate(&self, url: &str) -> Result<()>;
    fn reload(&self) -> Result<()>;
    fn stop(&self) -> Result<()>;
    fn close(&self) -> Result<()>;
    fn settings(&self) -> BrowserSettings;
    fn on_event(&self, event: BrowserEvent);
}
