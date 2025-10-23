// 🔧 URL UTILITIES

// Hilfsfunktionen für problematische URLs
pub fn normalize_problematic_url(url: &str) -> String {
    let mut normalized = url.to_lowercase();
    
    // Protokoll hinzufügen falls nicht vorhanden
    if !normalized.starts_with("http://") && !normalized.starts_with("https://") {
        // Für bekannte HTTPS-Sites
        if normalized.contains("google") || normalized.contains("github") || normalized.contains("consent.yahoo") {
            normalized = format!("https://{}", normalized);
        } else {
            normalized = format!("http://{}", normalized);
        }
    }
    
    // Spezielle Behandlung für bekannte problematische Domains
    normalized = match normalized.as_str() {
        url if url.contains("google.de") || url.contains("google.com") => {
            if url.contains("google.de") {
                "https://www.google.de".to_string()
            } else {
                "https://www.google.com".to_string()
            }
        }
        url if url.contains("github.com") => "https://github.com".to_string(),
        url if url.contains("consent.yahoo.com") => "https://consent.yahoo.com".to_string(), 
        url if url.contains("bing.com") => "https://www.bing.com".to_string(),
        url if url.contains("dzen.ru") => "https://dzen.ru".to_string(),
        _ => normalized,
    };
    
    println!("🔧 URL normalized from '{}' to '{}'", url, normalized);
    normalized
}

pub fn should_use_proxy_for_url(url: &str) -> bool {
    let problematic_domains = [
        "consent.yahoo.com",
        "www.google.com", 
        "google.de",
        "www.google.de",
        "www.bing.com",
        "github.com",
        "dzen.ru"
    ];
    
    let should_proxy = problematic_domains.iter().any(|domain| url.contains(domain));
    println!("🤔 Should use proxy for '{}': {}", url, should_proxy);
    should_proxy
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_normalize_problematic_url() {
        // Test HTTPS addition for known sites
        assert_eq!(
            normalize_problematic_url("google.com"),
            "https://www.google.com"
        );
        
        assert_eq!(
            normalize_problematic_url("github.com"),
            "https://github.com"
        );
        
        // Test HTTP addition for unknown sites
        assert_eq!(
            normalize_problematic_url("example.com"),
            "http://example.com"
        );
        
        // Test already normalized URLs
        assert_eq!(
            normalize_problematic_url("https://www.google.com"),
            "https://www.google.com"
        );
        
        // Test special cases
        assert_eq!(
            normalize_problematic_url("consent.yahoo.com"),
            "https://consent.yahoo.com"
        );
    }
    
    #[test]
    fn test_should_use_proxy_for_url() {
        // Test problematic domains that should use proxy
        assert!(should_use_proxy_for_url("https://www.google.com"));
        assert!(should_use_proxy_for_url("https://github.com"));
        assert!(should_use_proxy_for_url("https://dzen.ru"));
        assert!(should_use_proxy_for_url("https://consent.yahoo.com"));
        
        // Test normal domains that shouldn't use proxy
        assert!(!should_use_proxy_for_url("https://example.com"));
        assert!(!should_use_proxy_for_url("https://stackoverflow.com"));
        assert!(!should_use_proxy_for_url("https://wikipedia.org"));
    }
    
    #[test]
    fn test_url_normalization_edge_cases() {
        // Test empty string
        assert_eq!(normalize_problematic_url(""), "http://");
        
        // Test with protocols already present
        assert_eq!(
            normalize_problematic_url("http://example.com"),
            "http://example.com"
        );
        
        // Test case insensitivity
        assert_eq!(
            normalize_problematic_url("GOOGLE.COM"),
            "https://www.google.com"
        );
        
        // Test with paths
        assert_eq!(
            normalize_problematic_url("github.com/user/repo"),
            "https://github.com"
        );
    }
} 
