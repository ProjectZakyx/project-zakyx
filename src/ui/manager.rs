// 🎨 MAIN UI/UX IMPROVEMENTS MANAGER - Central Coordinator
use super::theme::{Theme, ThemeManager};
use super::shortcuts::ShortcutManager;
use super::drag::TabDragManager;
use super::fullscreen::FullscreenManager;
use anyhow::Result;
use windows::Win32::Foundation::HWND;

pub struct UIUXImprovementsManager {
    theme_manager: ThemeManager,
    shortcut_manager: ShortcutManager,
    tab_drag_manager: TabDragManager,
    fullscreen_manager: FullscreenManager,
    window_handle: HWND,
}

impl UIUXImprovementsManager {
    pub fn new(window_handle: HWND) -> Result<Self> {
        println!("🎨 Initializing UI/UX Improvements...");

        Ok(UIUXImprovementsManager {
            theme_manager: ThemeManager::new(),
            shortcut_manager: ShortcutManager::new(),
            tab_drag_manager: TabDragManager::new(),
            fullscreen_manager: FullscreenManager::new(),
            window_handle,
        })
    }

    // Theme Management
    pub fn toggle_theme(&mut self) -> Theme {
        let new_theme = self.theme_manager.toggle_theme();
        self.apply_theme();
        new_theme
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme_manager.set_theme(theme);
        self.apply_theme();
    }

    fn apply_theme(&self) {
        let scheme = self.theme_manager.get_current_scheme();
        println!(
            "🎨 Applying theme with background: #{:06X}",
            scheme.background
        );

        // Hier würde man die UI-Elemente mit den neuen Farben aktualisieren
        // Das ist eine vereinfachte Implementation
    }

    // Keyboard Shortcuts
    pub fn handle_keypress(&self, key: u32, modifiers: u32) -> Option<String> {
        self.shortcut_manager.handle_keypress(key, modifiers)
    }

    pub fn toggle_shortcuts(&mut self) -> bool {
        self.shortcut_manager.toggle_shortcuts()
    }

    // Tab Drag & Drop
    pub fn start_tab_drag(&mut self, tab_id: String, x: i32, y: i32) {
        self.tab_drag_manager.start_drag(tab_id, x, y);
    }

    pub fn update_tab_drag(&mut self, x: i32, y: i32) {
        self.tab_drag_manager.update_drag(x, y);
    }

    pub fn end_tab_drag(&mut self) -> Option<String> {
        self.tab_drag_manager.end_drag()
    }

    // Fullscreen
    pub fn toggle_fullscreen(&mut self) -> Result<bool> {
        self.fullscreen_manager
            .toggle_fullscreen(self.window_handle)
    }

    // Display Methods
    pub fn get_theme_display(&self) -> Vec<String> {
        self.theme_manager.get_theme_display()
    }

    pub fn get_shortcuts_display(&self) -> Vec<String> {
        self.shortcut_manager.get_shortcuts_display()
    }

    pub fn get_ui_improvements_display(&self) -> Vec<String> {
        vec![
            "🎨 UI/UX IMPROVEMENTS".to_string(),
            "=====================".to_string(),
            "".to_string(),
            "💬 VERFÜGBARE KOMMANDOS:".to_string(),
            "• 'theme' → Theme Manager".to_string(),
            "• 'shortcuts' → Keyboard Shortcuts".to_string(),
            "• 'fullscreen' → Fullscreen Toggle".to_string(),
            "".to_string(),
            "🎯 FEATURES:".to_string(),
            "• 🎨 Dark/Light Theme System".to_string(),
            "• ⌨️ Anpassbare Keyboard Shortcuts".to_string(),
            "• 🖱️ Drag & Drop für Tabs".to_string(),
            "• 🖥️ Fullscreen-Modus".to_string(),
            "• 🎭 Auto Theme Detection".to_string(),
            "".to_string(),
            format!("Current Theme: {:?}", self.theme_manager.get_current_theme()),
            format!(
                "Shortcuts: {}",
                if self.shortcut_manager.is_enabled() {
                    "✅"
                } else {
                    "❌"
                }
            ),
            format!(
                "Tab Drag: {}",
                if self.tab_drag_manager.is_enabled() {
                    "✅"
                } else {
                    "❌"
                }
            ),
            format!(
                "Fullscreen: {}",
                if self.fullscreen_manager.is_fullscreen() {
                    "✅"
                } else {
                    "❌"
                }
            ),
            "".to_string(),
            "⚡ Kommando eingeben und Enter drücken!".to_string(),
        ]
    }

    pub async fn handle_command(&mut self, command: &str) -> Result<Vec<String>> {
        match command.trim().to_lowercase().as_str() {
            "theme" => Ok(self.get_theme_display()),
            "shortcuts" => Ok(self.get_shortcuts_display()),

            "theme toggle" => {
                let new_theme = self.toggle_theme();
                Ok(vec![format!("🎨 Theme changed to: {:?}", new_theme)])
            }

            "theme light" => {
                self.set_theme(Theme::Light);
                Ok(vec!["🎨 Light theme activated".to_string()])
            }

            "theme dark" => {
                self.set_theme(Theme::Dark);
                Ok(vec!["🎨 Dark theme activated".to_string()])
            }

            "theme auto" => {
                self.set_theme(Theme::Auto);
                Ok(vec!["🎨 Auto theme activated".to_string()])
            }

            "shortcuts toggle" => {
                let enabled = self.toggle_shortcuts();
                Ok(vec![format!(
                    "⌨️ Shortcuts: {}",
                    if enabled { "Enabled" } else { "Disabled" }
                )])
            }

            "fullscreen" => match self.toggle_fullscreen() {
                Ok(is_fullscreen) => Ok(vec![format!(
                    "🖥️ Fullscreen: {}",
                    if is_fullscreen { "Enabled" } else { "Disabled" }
                )]),
                Err(e) => Ok(vec![format!("❌ Fullscreen error: {:?}", e)]),
            },

            _ => Ok(vec![
                "❓ Unknown UI command. Try: theme, shortcuts, fullscreen".to_string(),
            ]),
        }
    }

    pub async fn cleanup(&self) -> Result<()> {
        println!("🧹 UI/UX Improvements cleanup...");
        Ok(())
    }
} 