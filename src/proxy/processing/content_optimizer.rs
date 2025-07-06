// 🚀 Content Optimizer Module
// Optimiert und filtert HTML-Content

pub struct ContentOptimizer;

impl ContentOptimizer {
    pub fn new() -> Self {
        Self
    }

    /// Optimiere Content basierend auf Typ
    pub fn optimize_content(&self, content: &str, content_type: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        if content_type.contains("text/html") {
            self.optimize_html(content)
        } else if content_type.contains("text/css") {
            self.optimize_css(content)
        } else if content_type.contains("application/javascript") {
            self.optimize_javascript(content)
        } else {
            Ok(content.to_string())
        }
    }

    /// Optimiere HTML-Content
    fn optimize_html(&self, html: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut optimized = html.to_string();
        
        // Entferne problematische Meta-Tags
        optimized = self.remove_problematic_meta_tags(&optimized);
        
        // Füge Sicherheits-Headers hinzu
        optimized = self.add_security_headers(&optimized);
        
        // Optimiere Bilder-Loading
        optimized = self.optimize_image_loading(&optimized);
        
        Ok(optimized)
    }

    /// Optimiere CSS-Content
    fn optimize_css(&self, css: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut optimized = css.to_string();
        
        // Entferne problematische CSS-Regeln
        optimized = optimized.replace("@import", "/* @import blocked */");
        optimized = optimized.replace("expression(", "/* expression blocked */");
        
        Ok(optimized)
    }

    /// Optimiere JavaScript-Content
    fn optimize_javascript(&self, js: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut optimized = js.to_string();
        
        // Entferne problematische JavaScript-Aufrufe
        optimized = optimized.replace("eval(", "/* eval blocked */");
        optimized = optimized.replace("Function(", "/* Function blocked */");
        
        Ok(optimized)
    }

    /// Entferne iFrame-Blöcke
    pub fn strip_iframe_blocks(&self, html: &str, _url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut result = html.to_string();
        
        // Entferne alle iframes
        result = regex::Regex::new(r"<iframe[^>]*>.*?</iframe>")
            .unwrap()
            .replace_all(&result, "<!-- iframe removed for compatibility -->")
            .to_string();
        
        // Entferne auch selbstschließende iframes
        result = regex::Regex::new(r"<iframe[^>]*/>")
            .unwrap()
            .replace_all(&result, "<!-- iframe removed for compatibility -->")
            .to_string();
        
        Ok(result)
    }

    /// Entferne problematische Meta-Tags
    fn remove_problematic_meta_tags(&self, html: &str) -> String {
        let mut result = html.to_string();
        
        // Entferne X-Frame-Options
        result = regex::Regex::new(r#"<meta[^>]*http-equiv=["']?X-Frame-Options["']?[^>]*>"#)
            .unwrap()
            .replace_all(&result, "")
            .to_string();
        
        // Entferne strenge CSP-Policies
        result = regex::Regex::new(r#"<meta[^>]*Content-Security-Policy[^>]*>"#)
            .unwrap()
            .replace_all(&result, "")
            .to_string();
        
        result
    }

    /// Füge Sicherheits-Headers hinzu
    fn add_security_headers(&self, html: &str) -> String {
        let security_headers = r#"
<meta name="referrer" content="no-referrer">
<meta http-equiv="X-Content-Type-Options" content="nosniff">
<meta http-equiv="Content-Security-Policy" content="default-src * 'unsafe-inline' 'unsafe-eval' data: blob:;">
"#;
        
        if let Some(head_pos) = html.find("<head>") {
            let mut result = html.to_string();
            result.insert_str(head_pos + 6, security_headers);
            result
        } else {
            html.to_string()
        }
    }

    /// Optimiere Bilder-Loading
    fn optimize_image_loading(&self, html: &str) -> String {
        // Füge lazy loading für Bilder hinzu
        regex::Regex::new(r#"<img([^>]*?)>"#)
            .unwrap()
            .replace_all(html, |caps: &regex::Captures| {
                let attrs = &caps[1];
                if attrs.contains("loading=") {
                    format!("<img{}>", attrs)
                } else {
                    format!("<img{} loading=\"lazy\">", attrs)
                }
            })
            .to_string()
    }

    /// Prüfe ob Major-Website
    pub fn is_major_website(&self, domain: &str) -> bool {
        let major_patterns = [
            "google", "yahoo", "bing", "microsoft", "github", "stackoverflow",
            "wikipedia", "facebook", "twitter", "youtube", "amazon", "apple",
            "netflix", "reddit", "linkedin", "instagram", "tiktok", "discord",
            "dzen", "yandex", "baidu", "qq", "weibo", "vk", "consent",
            "cloudflare", "akamai", "fastly", "jsdelivr", "cdnjs"
        ];
        
        major_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    /// Prüfe ob Social Media-Website
    pub fn is_social_media_site(&self, domain: &str) -> bool {
        let social_patterns = [
            "facebook", "instagram", "twitter", "x.com", "linkedin", 
            "tiktok", "snapchat", "pinterest", "reddit", "discord",
            "telegram", "whatsapp", "youtube", "vimeo", "twitch"
        ];
        
        social_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    /// Komprimiere HTML (entferne unnötige Whitespace)
    pub fn compress_html(&self, html: &str) -> String {
        // Entferne überschüssige Whitespace zwischen Tags
        regex::Regex::new(r">\s+<")
            .unwrap()
            .replace_all(html, "><")
            .to_string()
    }

    /// Entferne Kommentare aus HTML
    pub fn remove_html_comments(&self, html: &str) -> String {
        regex::Regex::new(r"<!--.*?-->")
            .unwrap()
            .replace_all(html, "")
            .to_string()
    }

    /// Ersetze relative URLs mit absoluten URLs
    pub fn fix_relative_urls(&self, html: &str, base_url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut result = html.to_string();
        
        if let Ok(parsed_base) = url::Url::parse(base_url) {
            // Fixe src-Attribute
            result = regex::Regex::new(r#"src=["']([^"']+)["']"#)
                .unwrap()
                .replace_all(&result, |caps: &regex::Captures| {
                    let src = &caps[1];
                    if src.starts_with("http") || src.starts_with("//") {
                        format!(r#"src="{}""#, src)
                    } else {
                        match parsed_base.join(src) {
                            Ok(absolute_url) => format!(r#"src="{}""#, absolute_url),
                            Err(_) => format!(r#"src="{}""#, src),
                        }
                    }
                })
                .to_string();
            
            // Fixe href-Attribute
            result = regex::Regex::new(r#"href=["']([^"']+)["']"#)
                .unwrap()
                .replace_all(&result, |caps: &regex::Captures| {
                    let href = &caps[1];
                    if href.starts_with("http") || href.starts_with("//") || href.starts_with("#") {
                        format!(r#"href="{}""#, href)
                    } else {
                        match parsed_base.join(href) {
                            Ok(absolute_url) => format!(r#"href="{}""#, absolute_url),
                            Err(_) => format!(r#"href="{}""#, href),
                        }
                    }
                })
                .to_string();
        }
        
        Ok(result)
    }
}

impl Default for ContentOptimizer {
    fn default() -> Self {
        Self::new()
    }
} 