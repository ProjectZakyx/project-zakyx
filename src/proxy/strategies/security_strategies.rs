// 🔒 Security-oriented Browser Strategies
// Spezielle Strategien für Banking-, Government- und sicherheitskritische Websites

use super::browser_simulation::ConnectionStrategy;

/// Sicherheitsorientierte Strategien
pub fn get_security_strategies() -> Vec<ConnectionStrategy> {
    vec![
        // 🏦 Banking-Browser (hohe Sicherheit)
        ConnectionStrategy {
            description: "Secure Banking Browser".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            timeout: 45,
            connect_timeout: 20,
            redirects: 3, // Weniger Redirects für Sicherheit
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
                ("Accept-Language", "en-US,en;q=0.9"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
                ("Sec-Fetch-Dest", "document"),
                ("Sec-Fetch-Mode", "navigate"),
                ("Sec-Fetch-Site", "none"),
                ("Sec-Fetch-User", "?1"),
                ("Sec-CH-UA", "\"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\", \"Not_A Brand\";v=\"8\""),
                ("Sec-CH-UA-Mobile", "?0"),
                ("Sec-CH-UA-Platform", "\"Windows\""),
            ],
        },

        // 🏛️ Government-Browser (maximale Sicherheit)
        ConnectionStrategy {
            description: "Government Secure Browser".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Gov/1.0".to_string(),
            timeout: 60,
            connect_timeout: 25,
            redirects: 2, // Minimal redirects
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9"),
                ("Accept-Language", "en-US,en;q=0.9"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
                ("Sec-Fetch-Dest", "document"),
                ("Sec-Fetch-Mode", "navigate"),
                ("Sec-Fetch-Site", "none"),
                ("Sec-Fetch-User", "?1"),
                ("Cache-Control", "no-cache"),
                ("Pragma", "no-cache"),
            ],
        },

        // 🛡️ Enterprise Security Browser
        ConnectionStrategy {
            description: "Enterprise Security Browser".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Enterprise/1.0".to_string(),
            timeout: 40,
            connect_timeout: 18,
            redirects: 4,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"),
                ("Accept-Language", "en-US,en;q=0.9,de;q=0.8"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
                ("Sec-Fetch-Dest", "document"),
                ("Sec-Fetch-Mode", "navigate"),
                ("Sec-Fetch-Site", "none"),
                ("Sec-Fetch-User", "?1"),
                ("X-Forwarded-Proto", "https"),
            ],
        },
    ]
}

/// Healthcare-spezifische Strategien
pub fn get_healthcare_strategies() -> Vec<ConnectionStrategy> {
    vec![
        ConnectionStrategy {
            description: "Healthcare Compliant Browser".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 HIPAA/1.0".to_string(),
            timeout: 50,
            connect_timeout: 20,
            redirects: 3,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
                ("Accept-Language", "en-US,en;q=0.9"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
                ("Sec-Fetch-Dest", "document"),
                ("Sec-Fetch-Mode", "navigate"),
                ("Sec-Fetch-Site", "none"),
                ("Sec-Fetch-User", "?1"),
                ("Cache-Control", "private, no-cache, no-store"),
                ("Pragma", "no-cache"),
            ],
        },
    ]
}

/// Legal/Compliance Browser-Strategien
pub fn get_legal_strategies() -> Vec<ConnectionStrategy> {
    vec![
        ConnectionStrategy {
            description: "Legal Compliance Browser".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Legal/1.0".to_string(),
            timeout: 45,
            connect_timeout: 20,
            redirects: 3,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
                ("Accept-Language", "en-US,en;q=0.9"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
                ("Sec-Fetch-Dest", "document"),
                ("Sec-Fetch-Mode", "navigate"),
                ("Sec-Fetch-Site", "none"),
                ("Sec-Fetch-User", "?1"),
                ("DNT", "1"), // Do Not Track
                ("Sec-GPC", "1"), // Global Privacy Control
            ],
        },
    ]
} 
