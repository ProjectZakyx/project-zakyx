// 🎨 THEME SYSTEM - Theme Manager
use super::color_scheme::{Theme, ColorScheme};

pub struct ThemeManager {
    current_theme: Theme,
    light_scheme: ColorScheme,
    dark_scheme: ColorScheme,
    auto_detect: bool,
}

impl ThemeManager {
    pub fn new() -> Self {
        ThemeManager {
            current_theme: Theme::Light,
            light_scheme: ColorScheme::light(),
            dark_scheme: ColorScheme::dark(),
            auto_detect: true,
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.current_theme = theme;
        println!("🎨 Theme changed to: {:?}", self.current_theme);
    }

    pub fn get_current_scheme(&self) -> &ColorScheme {
        match self.current_theme {
            Theme::Light => &self.light_scheme,
            Theme::Dark => &self.dark_scheme,
            Theme::Auto => {
                // Vereinfacht: Immer Light für Auto
                &self.light_scheme
            }
        }
    }

    pub fn toggle_theme(&mut self) -> Theme {
        self.current_theme = match self.current_theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
            Theme::Auto => Theme::Light,
        };

        println!("🎨 Theme toggled to: {:?}", self.current_theme);
        self.current_theme.clone()
    }

    pub fn get_current_theme(&self) -> &Theme {
        &self.current_theme
    }

    pub fn get_theme_display(&self) -> Vec<String> {
        let scheme = self.get_current_scheme();

        vec![
            "🎨 THEME MANAGER".to_string(),
            "================".to_string(),
            "".to_string(),
            format!("Current Theme: {:?}", self.current_theme),
            format!(
                "Auto-Detect: {}",
                if self.auto_detect { "✅" } else { "❌" }
            ),
            "".to_string(),
            "🎨 COLOR SCHEME:".to_string(),
            format!("   Background: #{:06X}", scheme.background),
            format!("   Foreground: #{:06X}", scheme.foreground),
            format!("   Accent: #{:06X}", scheme.accent),
            format!("   Border: #{:06X}", scheme.border),
            format!("   Highlight: #{:06X}", scheme.highlight),
            format!("   Text Primary: #{:06X}", scheme.text_primary),
            format!("   Text Secondary: #{:06X}", scheme.text_secondary),
            "".to_string(),
            "💡 COMMANDS:".to_string(),
            "• 'theme toggle' → Theme wechseln".to_string(),
            "• 'theme light' → Light Theme".to_string(),
            "• 'theme dark' → Dark Theme".to_string(),
            "• 'theme auto' → Auto Theme".to_string(),
        ]
    }
} 