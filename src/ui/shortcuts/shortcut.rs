// ⌨️ SHORTCUTS SYSTEM - Keyboard Shortcut Definition
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardShortcut {
    pub key: u32,
    pub modifiers: u32,
    pub action: String,
    pub description: String,
} 
