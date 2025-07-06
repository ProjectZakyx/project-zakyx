// 📱 Mobile Browser Strategies
// Strategien für Mobile Browser-Simulation als Fallback

use super::browser_simulation::ConnectionStrategy;

/// Mobile Browser-Strategien als Fallback
pub fn get_mobile_strategies() -> Vec<ConnectionStrategy> {
    vec![
        // 📱 Android Chrome Mobile
        ConnectionStrategy {
            description: "Android Chrome Mobile".to_string(),
            user_agent: "Mozilla/5.0 (Linux; Android 13; SM-G998B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36".to_string(),
            timeout: 25,
            connect_timeout: 10,
            redirects: 5,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8"),
                ("Accept-Language", "en-US,en;q=0.9"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
                ("Sec-Fetch-Dest", "document"),
                ("Sec-Fetch-Mode", "navigate"),
                ("Sec-Fetch-Site", "none"),
                ("Sec-Fetch-User", "?1"),
                ("Sec-CH-UA", "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\""),
                ("Sec-CH-UA-Mobile", "?1"),
                ("Sec-CH-UA-Platform", "\"Android\""),
            ],
        },

        // 🍎 iPhone Safari Mobile
        ConnectionStrategy {
            description: "iPhone Safari Mobile".to_string(),
            user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1".to_string(),
            timeout: 30,
            connect_timeout: 12,
            redirects: 6,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
                ("Accept-Language", "en-US,en;q=0.9"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
            ],
        },

        // 📱 Samsung Internet Mobile
        ConnectionStrategy {
            description: "Samsung Internet Mobile".to_string(),
            user_agent: "Mozilla/5.0 (Linux; Android 13; SM-G998B) AppleWebKit/537.36 (KHTML, like Gecko) SamsungBrowser/23.0 Chrome/115.0.0.0 Mobile Safari/537.36".to_string(),
            timeout: 25,
            connect_timeout: 10,
            redirects: 5,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8"),
                ("Accept-Language", "en-US,en;q=0.9"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
            ],
        },
    ]
}

/// Tablet-spezifische Strategien
pub fn get_tablet_strategies() -> Vec<ConnectionStrategy> {
    vec![
        // 📱 iPad Safari
        ConnectionStrategy {
            description: "iPad Safari".to_string(),
            user_agent: "Mozilla/5.0 (iPad; CPU OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1".to_string(),
            timeout: 35,
            connect_timeout: 15,
            redirects: 6,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
                ("Accept-Language", "en-US,en;q=0.9"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
            ],
        },

        // 🤖 Android Tablet
        ConnectionStrategy {
            description: "Android Tablet Chrome".to_string(),
            user_agent: "Mozilla/5.0 (Linux; Android 13; SM-T970) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            timeout: 30,
            connect_timeout: 12,
            redirects: 5,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8"),
                ("Accept-Language", "en-US,en;q=0.9"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
                ("Sec-CH-UA", "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\""),
                ("Sec-CH-UA-Mobile", "?0"),
                ("Sec-CH-UA-Platform", "\"Android\""),
            ],
        },
    ]
}

/// Progressive Web App Browser
pub fn get_pwa_strategies() -> Vec<ConnectionStrategy> {
    vec![
        ConnectionStrategy {
            description: "Progressive Web App Browser".to_string(),
            user_agent: "Mozilla/5.0 (Linux; Android 13; SM-G998B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36 PWA/1.0".to_string(),
            timeout: 20,
            connect_timeout: 8,
            redirects: 4,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"),
                ("Accept-Language", "en-US,en;q=0.9"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
                ("Sec-Fetch-Mode", "navigate"),
                ("Sec-Fetch-Dest", "document"),
                ("Sec-Fetch-Site", "none"),
            ],
        },
    ]
} 