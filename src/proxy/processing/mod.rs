// 🔄 Content Processing Module
// Verarbeitet HTML-Content, CAPTCHA-Handling und Cookie-Enhancement

pub mod content_optimizer;
pub mod captcha_handler;
pub mod cookie_enhancer;

use content_optimizer::ContentOptimizer;
use captcha_handler::CaptchaHandler;
use cookie_enhancer::CookieEnhancer;

/// Haupt-Content-Processor
pub struct ContentProcessor {
    optimizer: ContentOptimizer,
    captcha_handler: CaptchaHandler,
    cookie_enhancer: CookieEnhancer,
}

impl ContentProcessor {
    pub fn new() -> Self {
        Self {
            optimizer: ContentOptimizer::new(),
            captcha_handler: CaptchaHandler::new(),
            cookie_enhancer: CookieEnhancer::new(),
        }
    }

    /// Verarbeite Content basierend auf Typ
    pub fn process_content(
        &self,
        content: &str,
        url: &str,
        content_type: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut processed_content = content.to_string();

        // HTML-spezifische Verarbeitung
        if content_type.contains("text/html") {
            processed_content = self.process_html_content(&processed_content, url)?;
        }

        // CAPTCHA-Erkennung und Handling
        if self.captcha_handler.is_captcha_page(&processed_content) {
            processed_content = self.captcha_handler.create_bypass_page(url, &processed_content)?;
        }

        // Content-Optimierung
        processed_content = self.optimizer.optimize_content(&processed_content, content_type)?;

        Ok(processed_content)
    }

    /// Verarbeite HTML-Content speziell
    fn process_html_content(
        &self,
        html: &str,
        url: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut processed = html.to_string();

        // Cookie-Enhancement
        processed = self.cookie_enhancer.inject_cookie_script(&processed)?;

        // iFrame-Blockierung
        processed = self.optimizer.strip_iframe_blocks(&processed, url)?;

        Ok(processed)
    }
}

impl Default for ContentProcessor {
    fn default() -> Self {
        Self::new()
    }
} 
