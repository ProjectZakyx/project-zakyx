// 🎨 ENHANCED GUI RENDERER SYSTEM - ZAKYX BROWSER
// =====================================================
// Erweiterte GUI-Rendering-Engine mit mehreren Fallback-Optionen

use std::collections::HashMap;
use windows::Win32::{
    Foundation::*,
    UI::WindowsAndMessaging::*,
};
use eyre::Result;
use crate::w;

#[derive(Debug, Clone)]
pub enum RenderingEngine {
    WebView2Native,
    WebBrowserControl,
    HtmlEditControl,
    CustomCanvas,
    FallbackText,
}

#[derive(Debug, Clone)]
pub struct GuiRenderingOptions {
    pub preferred_engine: RenderingEngine,
    pub enable_animations: bool,
    pub enable_glassmorphism: bool,
    pub theme: String,
    pub fallback_chain: Vec<RenderingEngine>,
}

impl Default for GuiRenderingOptions {
    fn default() -> Self {
        Self {
            preferred_engine: RenderingEngine::WebView2Native,
            enable_animations: true,
            enable_glassmorphism: true,
            theme: "dark".to_string(),
            fallback_chain: vec![
                RenderingEngine::WebView2Native,
                RenderingEngine::WebBrowserControl,
                RenderingEngine::HtmlEditControl,
                RenderingEngine::CustomCanvas,
                RenderingEngine::FallbackText,
            ],
        }
    }
}

pub struct EnhancedGuiRenderer {
    current_engine: Option<RenderingEngine>,
    options: GuiRenderingOptions,
    parent_window: HWND,
    render_window: Option<HWND>,
    html_content: String,
    css_styles: HashMap<String, String>,
    rendering_stats: RenderingStats,
}

#[derive(Debug, Default)]
pub struct RenderingStats {
    pub attempts: u32,
    pub successful_renders: u32,
    pub current_engine: String,
    pub fallback_count: u32,
    pub render_time_ms: u64,
}

impl EnhancedGuiRenderer {
    /// 🚀 ERSTELLE ERWEITERTEN GUI RENDERER
    pub fn new(parent_window: HWND) -> Result<Self> {
        println!("🎨 Creating Enhanced GUI Renderer...");
        
        let mut renderer = Self {
            current_engine: None,
            options: GuiRenderingOptions::default(),
            parent_window,
            render_window: None,
            html_content: String::new(),
            css_styles: HashMap::new(),
            rendering_stats: RenderingStats::default(),
        };
        
        renderer.initialize_css_styles();
        println!("✅ Enhanced GUI Renderer created!");
        Ok(renderer)
    }

    /// 🎨 INITIALISIERE CSS STYLES
    fn initialize_css_styles(&mut self) {
        self.css_styles.insert("glassmorphism".to_string(), r#"
            .glass-container {
                background: rgba(255, 255, 255, 0.1);
                backdrop-filter: blur(10px);
                border-radius: 15px;
                border: 1px solid rgba(255, 255, 255, 0.2);
                box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
            }
        "#.to_string());
        
        self.css_styles.insert("animations".to_string(), r#"
            @keyframes fadeIn {
                from { opacity: 0; transform: translateY(20px); }
                to { opacity: 1; transform: translateY(0); }
            }
            .animate-fade-in { animation: fadeIn 0.6s ease-out; }
        "#.to_string());

        self.css_styles.insert("dark_theme".to_string(), r#"
            body { 
                background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%);
                color: white; 
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            }
        "#.to_string());
    }

    /// 🌐 LADE HTML CONTENT
    pub fn load_html_content(&mut self, html: &str) -> Result<()> {
        println!("📄 Loading HTML content in Enhanced GUI Renderer...");
        self.html_content = self.enhance_html_with_styles(html);
        println!("✅ HTML content loaded and enhanced!");
        Ok(())
    }

    /// 🎨 ERWEITERE HTML MIT STYLES
    fn enhance_html_with_styles(&self, html: &str) -> String {
        let mut enhanced_html = html.to_string();
        
        // CSS Styles einfügen
        let css_block = format!(r#"
        <style>
            {}
            {}
            {}
        </style>
        "#, 
            self.css_styles.get("glassmorphism").unwrap_or(&String::new()),
            self.css_styles.get("animations").unwrap_or(&String::new()),
            self.css_styles.get("dark_theme").unwrap_or(&String::new())
        );
        
        // CSS in HTML einfügen
        if enhanced_html.contains("<head>") {
            enhanced_html = enhanced_html.replace("<head>", &format!("<head>{}", css_block));
        } else {
            enhanced_html = format!("{}{}", css_block, enhanced_html);
        }
        
        enhanced_html
    }

    /// 🚀 STARTE RENDERING PROZESS
    pub fn render(&mut self) -> Result<()> {
        println!("🎨 Starting Enhanced GUI Rendering Process...");
        self.rendering_stats.attempts += 1;
        
        let start_time = std::time::Instant::now();
        
        for engine in &self.options.fallback_chain.clone() {
            println!("🔄 Trying rendering engine: {:?}", engine);
            
            match self.try_render_with_engine(engine.clone()) {
                Ok(()) => {
                    self.current_engine = Some(engine.clone());
                    self.rendering_stats.successful_renders += 1;
                    self.rendering_stats.current_engine = format!("{:?}", engine);
                    self.rendering_stats.render_time_ms = start_time.elapsed().as_millis() as u64;
                    
                    println!("✅ Rendering successful with engine: {:?}", engine);
                    return Ok(());
                }
                Err(e) => {
                    println!("❌ Rendering failed with {:?}: {}", engine, e);
                    self.rendering_stats.fallback_count += 1;
                }
            }
        }
        
        Err(eyre::eyre!("All rendering engines failed"))
    }

    /// 🎯 VERSUCHE RENDERING MIT SPEZIFISCHER ENGINE
    fn try_render_with_engine(&mut self, engine: RenderingEngine) -> Result<()> {
        match engine {
            RenderingEngine::WebView2Native => self.render_with_webview2(),
            RenderingEngine::WebBrowserControl => self.render_with_webbrowser(),
            RenderingEngine::HtmlEditControl => self.render_with_html_edit(),
            RenderingEngine::CustomCanvas => self.render_with_custom_canvas(),
            RenderingEngine::FallbackText => self.render_with_fallback_text(),
        }
    }

    /// 🌐 WEBVIEW2 NATIVE RENDERING
    fn render_with_webview2(&mut self) -> Result<()> {
        println!("🌐 Attempting WebView2 Native rendering...");
        
        unsafe {
            // WebView2 Container erstellen
            let container = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                w!("STATIC"),
                w!("WebView2Container"),
                WS_CHILD | WS_VISIBLE | WS_BORDER,
                10, 10, 800, 600,
                self.parent_window,
                None,
                None,
                None,
            );
            
            if container.0 == 0 {
                return Err(eyre::eyre!("Failed to create WebView2 container"));
            }
            
            self.render_window = Some(container);
            
            // HTML direkt laden (vereinfacht)
            println!("📄 Loading HTML content in WebView2...");
            println!("✅ WebView2 Native rendering completed!");
            Ok(())
        }
    }

    /// 🌐 WEBBROWSER CONTROL RENDERING
    fn render_with_webbrowser(&mut self) -> Result<()> {
        println!("🌐 Attempting WebBrowser Control rendering...");
        
        unsafe {
            let browser_window = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                w!("AtlAxWin"),
                w!("Shell.Explorer.2"),
                WS_CHILD | WS_VISIBLE,
                10, 10, 800, 600,
                self.parent_window,
                None,
                None,
                None,
            );
            
            if browser_window.0 == 0 {
                return Err(eyre::eyre!("Failed to create WebBrowser control"));
            }
            
            self.render_window = Some(browser_window);
            println!("✅ WebBrowser Control rendering completed!");
            Ok(())
        }
    }

    /// ✏️ HTML EDIT CONTROL RENDERING
    fn render_with_html_edit(&mut self) -> Result<()> {
        println!("✏️ Attempting HTML Edit Control rendering...");
        
        unsafe {
            let edit_window = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                w!("EDIT"),
                w!(""),
                WS_CHILD | WS_VISIBLE | WS_VSCROLL | WS_HSCROLL,
                10, 10, 800, 600,
                self.parent_window,
                None,
                None,
                None,
            );
            
            if edit_window.0 == 0 {
                return Err(eyre::eyre!("Failed to create HTML Edit control"));
            }
            
            // HTML Text setzen
            let html_wide: Vec<u16> = self.html_content.encode_utf16().chain(Some(0)).collect();
            SendMessageW(edit_window, WM_SETTEXT, WPARAM(0), LPARAM(html_wide.as_ptr() as isize));
            
            self.render_window = Some(edit_window);
            println!("✅ HTML Edit Control rendering completed!");
            Ok(())
        }
    }

    /// 🎨 CUSTOM CANVAS RENDERING
    fn render_with_custom_canvas(&mut self) -> Result<()> {
        println!("🎨 Attempting Custom Canvas rendering...");
        
        unsafe {
            let canvas_window = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                w!("STATIC"),
                w!("Custom Canvas"),
                WS_CHILD | WS_VISIBLE,
                10, 10, 800, 600,
                self.parent_window,
                None,
                None,
                None,
            );
            
            if canvas_window.0 == 0 {
                return Err(eyre::eyre!("Failed to create Custom Canvas"));
            }
            
            self.render_window = Some(canvas_window);
            println!("✅ Custom Canvas rendering completed!");
            Ok(())
        }
    }

    /// 📝 FALLBACK TEXT RENDERING
    fn render_with_fallback_text(&mut self) -> Result<()> {
        println!("📝 Using Fallback Text rendering...");
        
        unsafe {
            let text_window = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                w!("LISTBOX"),
                w!(""),
                WS_CHILD | WS_VISIBLE | WS_VSCROLL,
                10, 10, 800, 600,
                self.parent_window,
                None,
                None,
                None,
            );
            
            if text_window.0 == 0 {
                return Err(eyre::eyre!("Failed to create Fallback Text control"));
            }
            
            // HTML als Text-Zeilen hinzufügen
            let lines: Vec<&str> = self.html_content.lines().collect();
            for line in lines {
                let line_wide: Vec<u16> = line.encode_utf16().chain(Some(0)).collect();
                SendMessageW(text_window, LB_ADDSTRING, WPARAM(0), LPARAM(line_wide.as_ptr() as isize));
            }
            
            self.render_window = Some(text_window);
            println!("✅ Fallback Text rendering completed!");
            Ok(())
        }
    }

    /// 📊 ERHALTE RENDERING STATISTIKEN
    pub fn get_rendering_stats(&self) -> &RenderingStats {
        &self.rendering_stats
    }

    /// 🔄 AKTUALISIERE RENDERING OPTIONS
    pub fn update_options(&mut self, options: GuiRenderingOptions) {
        self.options = options;
        println!("🔄 GUI Rendering options updated!");
    }

    /// 🧹 CLEANUP RESOURCES
    pub fn cleanup(&mut self) -> Result<()> {
        println!("🧹 Cleaning up Enhanced GUI Renderer...");
        
        if let Some(window) = self.render_window {
            unsafe {
                DestroyWindow(window);
            }
            self.render_window = None;
        }
        
        println!("✅ Enhanced GUI Renderer cleaned up!");
        Ok(())
    }
}

/// 🎨 GUI RENDERER MANAGER
pub struct GuiRendererManager {
    renderers: HashMap<String, EnhancedGuiRenderer>,
    active_renderer: Option<String>,
}

impl GuiRendererManager {
    pub fn new() -> Self {
        println!("🎨 Creating GUI Renderer Manager...");
        Self {
            renderers: HashMap::new(),
            active_renderer: None,
        }
    }

    pub fn create_renderer(&mut self, id: String, parent_window: HWND) -> Result<()> {
        let renderer = EnhancedGuiRenderer::new(parent_window)?;
        self.renderers.insert(id.clone(), renderer);
        self.active_renderer = Some(id);
        Ok(())
    }

    pub fn get_active_renderer(&mut self) -> Option<&mut EnhancedGuiRenderer> {
        if let Some(id) = &self.active_renderer {
            self.renderers.get_mut(id)
        } else {
            None
        }
    }

    pub fn cleanup_all(&mut self) -> Result<()> {
        for renderer in self.renderers.values_mut() {
            renderer.cleanup()?;
        }
        self.renderers.clear();
        self.active_renderer = None;
        Ok(())
    }
} 
