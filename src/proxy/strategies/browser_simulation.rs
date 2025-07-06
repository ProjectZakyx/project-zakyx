// 🌐 Browser Simulation Strategies
// Verschiedene Browser-Simulationsstrategien für unterschiedliche Websites

#[derive(Debug, Clone)]
pub struct ConnectionStrategy {
    pub description: String,
    pub user_agent: String,
    pub timeout: u64,
    pub connect_timeout: u64,
    pub redirects: usize,
    pub accept_invalid_certs: bool,
    pub use_http2: bool,
    pub headers: Vec<(&'static str, &'static str)>,
}

/// Standard Browser-Simulationsstrategien
pub fn get_standard_strategies() -> Vec<ConnectionStrategy> {
    vec![
        // 🎯 STRATEGIE 1: NATIVE BROWSER SIMULATION (universell für alle Websites)
        ConnectionStrategy {
            description: "Native Browser Simulation".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            timeout: 45,
            connect_timeout: 15,
            redirects: 10,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"),
                ("Accept-Language", "de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
                ("Sec-Fetch-Dest", "document"),
                ("Sec-Fetch-Mode", "navigate"),
                ("Sec-Fetch-Site", "none"),
                ("Sec-Fetch-User", "?1"),
                ("Sec-Ch-Ua", "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\""),
                ("Sec-Ch-Ua-Mobile", "?0"),
                ("Sec-Ch-Ua-Platform", "\"Windows\""),
                ("Cache-Control", "max-age=0"),
            ],
        },

        // 🌐 STRATEGIE 2: STANDARD CHROME (universell)
        ConnectionStrategy {
            description: "Standard Chrome Browser".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            timeout: 30,
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

        // 🦊 STRATEGIE 3: FIREFOX SIMULATION
        ConnectionStrategy {
            description: "Firefox Browser Simulation".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0".to_string(),
            timeout: 35,
            connect_timeout: 12,
            redirects: 6,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"),
                ("Accept-Language", "en-US,en;q=0.5"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
                ("Sec-Fetch-Dest", "document"),
                ("Sec-Fetch-Mode", "navigate"),
                ("Sec-Fetch-Site", "none"),
                ("Sec-Fetch-User", "?1"),
            ],
        },

        // 🔵 STRATEGIE 4: EDGE BROWSER SIMULATION
        ConnectionStrategy {
            description: "Microsoft Edge Browser".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0".to_string(),
            timeout: 30,
            connect_timeout: 12,
            redirects: 5,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8"),
                ("Accept-Language", "en-US,en;q=0.9,de;q=0.8"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
                ("Sec-Fetch-Dest", "document"),
                ("Sec-Fetch-Mode", "navigate"),
                ("Sec-Fetch-Site", "none"),
                ("Sec-Fetch-User", "?1"),
            ],
        },
    ]
}

/// Developer-orientierte Browser-Strategien
pub fn get_developer_strategies() -> Vec<ConnectionStrategy> {
    vec![
        ConnectionStrategy {
            description: "Developer Tools Browser".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 DevTools/120.0.0.0".to_string(),
            timeout: 30,
            connect_timeout: 12,
            redirects: 5,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8"),
                ("Accept-Language", "en-US,en;q=0.9,de;q=0.8"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
                ("Sec-Fetch-Dest", "document"),
                ("Sec-Fetch-Mode", "navigate"),
                ("Sec-Fetch-Site", "none"),
                ("Sec-Fetch-User", "?1"),
            ],
        },
    ]
}

/// Ältere Browser-Strategien für Legacy-Kompatibilität
pub fn get_legacy_strategies() -> Vec<ConnectionStrategy> {
    vec![
        ConnectionStrategy {
            description: "Legacy Browser Compatibility".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.212 Safari/537.36".to_string(),
            timeout: 25,
            connect_timeout: 8,
            redirects: 3,
            accept_invalid_certs: true,
            use_http2: false,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
                ("Accept-Language", "en-US,en;q=0.8"),
                ("Accept-Encoding", "gzip, deflate"),
                ("Connection", "keep-alive"),
            ],
        },
    ]
} 