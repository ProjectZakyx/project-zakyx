// ⌨️ SHORTCUTS SYSTEM - Shortcut Manager
use super::shortcut::KeyboardShortcut;
use std::collections::HashMap;

pub struct ShortcutManager {
    shortcuts: HashMap<String, KeyboardShortcut>,
    enabled: bool,
}

impl ShortcutManager {
    pub fn new() -> Self {
        let mut shortcuts = HashMap::new();

        // Standard-Shortcuts definieren
        shortcuts.insert(
            "new_tab".to_string(),
            KeyboardShortcut {
                key: 0x54,         // T
                modifiers: 0x0002, // Ctrl
                action: "newtab".to_string(),
                description: "Neuen Tab öffnen".to_string(),
            },
        );

        shortcuts.insert(
            "close_tab".to_string(),
            KeyboardShortcut {
                key: 0x57,         // W
                modifiers: 0x0002, // Ctrl
                action: "closetab".to_string(),
                description: "Tab schließen".to_string(),
            },
        );

        shortcuts.insert(
            "bookmarks".to_string(),
            KeyboardShortcut {
                key: 0x42,         // B
                modifiers: 0x0002, // Ctrl
                action: "bookmarks".to_string(),
                description: "Bookmarks anzeigen".to_string(),
            },
        );

        shortcuts.insert(
            "history".to_string(),
            KeyboardShortcut {
                key: 0x48,         // H
                modifiers: 0x0002, // Ctrl
                action: "history".to_string(),
                description: "Verlauf anzeigen".to_string(),
            },
        );

        shortcuts.insert(
            "downloads".to_string(),
            KeyboardShortcut {
                key: 0x4A,         // J
                modifiers: 0x0002, // Ctrl
                action: "downloads".to_string(),
                description: "Downloads anzeigen".to_string(),
            },
        );

        shortcuts.insert(
            "devtools".to_string(),
            KeyboardShortcut {
                key: 0x49,         // I
                modifiers: 0x000A, // Ctrl+Shift
                action: "devtools".to_string(),
                description: "Developer Tools".to_string(),
            },
        );

        shortcuts.insert(
            "theme_toggle".to_string(),
            KeyboardShortcut {
                key: 0x54,         // T
                modifiers: 0x000A, // Ctrl+Shift
                action: "theme toggle".to_string(),
                description: "Theme wechseln".to_string(),
            },
        );

        ShortcutManager {
            shortcuts,
            enabled: true,
        }
    }

    pub fn handle_keypress(&self, key: u32, modifiers: u32) -> Option<String> {
        if !self.enabled {
            return None;
        }

        for shortcut in self.shortcuts.values() {
            if shortcut.key == key && shortcut.modifiers == modifiers {
                println!(
                    "⌨️ Shortcut triggered: {} -> {}",
                    shortcut.description, shortcut.action
                );
                return Some(shortcut.action.clone());
            }
        }

        None
    }

    pub fn add_shortcut(&mut self, id: String, shortcut: KeyboardShortcut) {
        self.shortcuts.insert(id.clone(), shortcut);
        println!("⌨️ Shortcut added: {}", id);
    }

    pub fn remove_shortcut(&mut self, id: &str) -> bool {
        if self.shortcuts.remove(id).is_some() {
            println!("⌨️ Shortcut removed: {}", id);
            true
        } else {
            false
        }
    }

    pub fn toggle_shortcuts(&mut self) -> bool {
        self.enabled = !self.enabled;
        println!(
            "⌨️ Shortcuts: {}",
            if self.enabled { "Enabled" } else { "Disabled" }
        );
        self.enabled
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_shortcuts_display(&self) -> Vec<String> {
        let mut result = vec![
            "⌨️ KEYBOARD SHORTCUTS".to_string(),
            "=====================".to_string(),
            "".to_string(),
            format!(
                "Status: {}",
                if self.enabled {
                    "✅ Enabled"
                } else {
                    "❌ Disabled"
                }
            ),
            "".to_string(),
            "📋 VERFÜGBARE SHORTCUTS:".to_string(),
        ];

        for shortcut in self.shortcuts.values() {
            let key_name = match shortcut.key {
                0x54 => "T",
                0x57 => "W",
                0x42 => "B",
                0x48 => "H",
                0x4A => "J",
                0x49 => "I",
                _ => "?",
            };

            let modifier_name = match shortcut.modifiers {
                0x0002 => "Ctrl",
                0x000A => "Ctrl+Shift",
                _ => "?",
            };

            result.push(format!(
                "   {}+{}: {}",
                modifier_name, key_name, shortcut.description
            ));
            result.push(format!("      → {}", shortcut.action));
        }

        result.push("".to_string());
        result.push("💡 COMMANDS:".to_string());
        result.push("• 'shortcuts toggle' → Shortcuts ein/aus".to_string());
        result.push("• 'shortcuts' → Diese Liste anzeigen".to_string());

        result
    }
} 