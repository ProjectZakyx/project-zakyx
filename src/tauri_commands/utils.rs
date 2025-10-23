#![allow(dead_code)] // Utility functions - kept for API completeness

use crate::browser_state::Tab;

/// URL-Validation
pub fn is_valid_url(url: &str) -> bool {
    url::Url::parse(url).is_ok()
}

/// Extract domain from URL
pub fn extract_domain(url: &str) -> Option<String> {
    url::Url::parse(url)
        .ok()?
        .host_str()
        .map(|s| s.to_string())
}

/// Create safe URL
pub fn create_safe_url(url: &str) -> String {
    if is_valid_url(url) {
        url.to_string()
    } else if !url.starts_with("http://") && !url.starts_with("https://") {
        format!("https://{}", url)
    } else {
        "about:blank".to_string()
    }
}

/// Get title from URL
pub fn get_title_from_url(url: &str) -> String {
    if let Some(domain) = extract_domain(url) {
        // Clean up domain
        let cleaned = domain.replace("www.", "");
        
        // Capitalize first letter
        let mut chars = cleaned.chars();
        match chars.next() {
            None => "Unknown".to_string(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    } else {
        "Unknown".to_string()
    }
}

/// Generate unique tab title
pub fn generate_unique_tab_title(base_title: &str, existing_tabs: &[Tab]) -> String {
    let mut counter = 1;
    let mut title = base_title.to_string();
    
    while existing_tabs.iter().any(|tab| tab.title == title) {
        counter += 1;
        title = format!("{} ({})", base_title, counter);
    }
    
    title
}

/// Validate tab ID
pub fn is_valid_tab_id(tab_id: &str) -> bool {
    tab_id.parse::<u32>().is_ok()
}

/// Find tab index by ID
pub fn find_tab_index(tabs: &[Tab], tab_id: &str) -> Option<usize> {
    tabs.iter().position(|tab| tab.id.to_string() == tab_id)
}

/// Format file size
pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    
    if size == 0 {
        return "0 B".to_string();
    }
    
    let mut size_f = size as f64;
    let mut unit_index = 0;
    
    while size_f >= 1024.0 && unit_index < UNITS.len() - 1 {
        size_f /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size_f, UNITS[unit_index])
    }
}

/// Format duration
pub fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

/// Sanitize filename
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            _ => c,
        })
        .collect()
}

/// Truncate string with ellipsis
pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length.saturating_sub(3)])
    }
}

/// Check if string is empty or whitespace
pub fn is_empty_or_whitespace(s: &str) -> bool {
    s.trim().is_empty()
}

/// Get file extension from URL
pub fn get_file_extension(url: &str) -> Option<String> {
    url.rfind('.')
        .and_then(|dot_index| {
            let extension = &url[dot_index + 1..];
            if extension.contains('/') || extension.contains('?') || extension.contains('#') {
                None
            } else {
                Some(extension.to_lowercase())
            }
        })
}

/// Check if URL is secure (HTTPS)
pub fn is_secure_url(url: &str) -> bool {
    url.starts_with("https://")
}

/// Clean URL for display
pub fn clean_url_for_display(url: &str) -> String {
    url.replace("https://", "")
        .replace("http://", "")
        .replace("www.", "")
        .trim_end_matches('/')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_url() {
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://example.com"));
        assert!(!is_valid_url("not-a-url"));
    }

    #[test]
    fn test_extract_domain() {
        assert_eq!(extract_domain("https://example.com/path"), Some("example.com".to_string()));
        assert_eq!(extract_domain("https://www.example.com"), Some("www.example.com".to_string()));
        assert_eq!(extract_domain("not-a-url"), None);
    }

    #[test]
    fn test_get_title_from_url() {
        assert_eq!(get_title_from_url("https://example.com"), "Example.com");
        assert_eq!(get_title_from_url("https://www.github.com"), "Github.com");
        assert_eq!(get_title_from_url("not-a-url"), "Unknown");
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1024 * 1024), "1.0 MB");
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("file<name>.txt"), "file_name_.txt");
        assert_eq!(sanitize_filename("normal_file.txt"), "normal_file.txt");
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("Hello World", 5), "He...");
        assert_eq!(truncate_string("Hello", 10), "Hello");
    }
} 
