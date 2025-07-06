// 🎨 THEME SYSTEM - Color Schemes and Theme Types
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    Auto, // Folgt System-Theme
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub background: u32,
    pub foreground: u32,
    pub accent: u32,
    pub border: u32,
    pub highlight: u32,
    pub text_primary: u32,
    pub text_secondary: u32,
}

impl ColorScheme {
    pub fn light() -> Self {
        ColorScheme {
            background: 0xFFFFFF,     // Weiß
            foreground: 0xF5F5F5,     // Hellgrau
            accent: 0x0078D4,         // Microsoft Blau
            border: 0xE1E1E1,         // Grau
            highlight: 0xE3F2FD,      // Hellblau
            text_primary: 0x000000,   // Schwarz
            text_secondary: 0x666666, // Dunkelgrau
        }
    }

    pub fn dark() -> Self {
        ColorScheme {
            background: 0x1E1E1E,     // Dunkelgrau
            foreground: 0x2D2D30,     // Grau
            accent: 0x0078D4,         // Microsoft Blau
            border: 0x3E3E42,         // Grau
            highlight: 0x094771,      // Dunkelblau
            text_primary: 0xFFFFFF,   // Weiß
            text_secondary: 0xCCCCCC, // Hellgrau
        }
    }
} 