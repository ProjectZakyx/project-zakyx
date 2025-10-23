// 🌍 Regional Browser Strategies
// Spezielle Strategien für regionale Websites (Russland, China, etc.)

use super::browser_simulation::ConnectionStrategy;

/// Regionale Strategien basierend auf URL
pub fn get_regional_strategies(url: &str) -> Vec<ConnectionStrategy> {
    let mut strategies = Vec::new();

    if is_russian_url(url) {
        strategies.extend(get_russian_strategies());
    }

    if is_chinese_url(url) {
        strategies.extend(get_chinese_strategies());
    }

    if is_european_url(url) {
        strategies.extend(get_european_strategies());
    }

    strategies
}

/// Russische Browser-Strategien (Yandex Browser)
pub fn get_russian_strategies() -> Vec<ConnectionStrategy> {
    vec![
        ConnectionStrategy {
            description: "Yandex Browser (Russian)".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 YaBrowser/24.1.0.0 Safari/537.36".to_string(),
            timeout: 35,
            connect_timeout: 15,
            redirects: 8,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8"),
                ("Accept-Language", "ru-RU,ru;q=0.9,en;q=0.8"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
            ],
        },
        ConnectionStrategy {
            description: "Mail.ru Browser (Russian)".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Mailru/1.0".to_string(),
            timeout: 30,
            connect_timeout: 12,
            redirects: 6,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
                ("Accept-Language", "ru-RU,ru;q=0.9,en;q=0.8"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
            ],
        },
    ]
}

/// Chinesische Browser-Strategien
pub fn get_chinese_strategies() -> Vec<ConnectionStrategy> {
    vec![
        ConnectionStrategy {
            description: "QQ Browser (Chinese)".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 QQBrowser/11.9.5355.400".to_string(),
            timeout: 35,
            connect_timeout: 15,
            redirects: 8,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8"),
                ("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
            ],
        },
        ConnectionStrategy {
            description: "UC Browser (Chinese)".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 UCBrowser/15.0.0.0".to_string(),
            timeout: 30,
            connect_timeout: 12,
            redirects: 6,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
                ("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
            ],
        },
    ]
}

/// Europäische Browser-Strategien
pub fn get_european_strategies() -> Vec<ConnectionStrategy> {
    vec![
        ConnectionStrategy {
            description: "European Privacy Browser".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Privacy/1.0".to_string(),
            timeout: 40,
            connect_timeout: 15,
            redirects: 5,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8"),
                ("Accept-Language", "de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
                ("DNT", "1"),
                ("Sec-GPC", "1"),
            ],
        },
    ]
}

/// Prüfe ob URL russisch ist
fn is_russian_url(url: &str) -> bool {
    let domain = extract_domain(url);
    domain.contains(".ru") || 
    domain.contains("yandex") || 
    domain.contains("mail.ru") ||
    domain.contains("vk.com") ||
    domain.contains("ok.ru")
}

/// Prüfe ob URL chinesisch ist
fn is_chinese_url(url: &str) -> bool {
    let domain = extract_domain(url);
    domain.contains(".cn") || 
    domain.contains("baidu") || 
    domain.contains("weibo") ||
    domain.contains("qq.com") ||
    domain.contains("taobao") ||
    domain.contains("tmall")
}

/// Prüfe ob URL europäisch ist
fn is_european_url(url: &str) -> bool {
    let domain = extract_domain(url);
    domain.contains(".de") || 
    domain.contains(".fr") || 
    domain.contains(".it") ||
    domain.contains(".es") ||
    domain.contains(".nl") ||
    domain.contains(".eu")
}

/// Extrahiere Domain aus URL
fn extract_domain(url: &str) -> String {
    if let Ok(parsed) = url::Url::parse(url) {
        parsed.host_str().unwrap_or("").to_lowercase()
    } else {
        url.to_lowercase()
    }
} 
