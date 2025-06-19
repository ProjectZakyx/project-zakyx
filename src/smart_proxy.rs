use reqwest;

#[derive(Debug, Clone)]
pub struct ProxyResponse {
    pub content: String,
    #[allow(dead_code)]
    #[allow(dead_code)]
    pub content_type: String,
    #[allow(dead_code)]
    pub status_code: u16,
}

#[derive(Debug, Clone)]
struct ConnectionStrategy {
    description: String,
    user_agent: String,
    timeout: u64,
    connect_timeout: u64,
    redirects: usize,
    accept_invalid_certs: bool,
    use_http2: bool,
    headers: Vec<(&'static str, &'static str)>,
}

pub struct SmartProxy;

impl SmartProxy {
    pub async fn fetch_and_strip_headers(url: &str) -> Result<ProxyResponse, Box<dyn std::error::Error + Send + Sync>> {
        println!("🌐 Smart fetching URL: {}", url);

        let strategies = Self::generate_adaptive_strategies(url);
        
        for (strategy_idx, strategy) in strategies.iter().enumerate() {
            println!("🔄 Trying strategy {} for {}: {}", strategy_idx + 1, url, strategy.description);
            
            match Self::try_strategy(url, strategy).await {
                Ok(response) => {
                    println!("✅ Strategy {} succeeded for {}", strategy_idx + 1, url);
                    return Self::process_response(response, url).await;
                }
                Err(e) => {
                    println!("❌ Strategy {} failed for {}: {}", strategy_idx + 1, url, e);
                    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                }
            }
        }

        Err("All adaptive strategies failed".into())
    }

    fn generate_adaptive_strategies(url: &str) -> Vec<ConnectionStrategy> {
        let domain = Self::extract_domain(url);
        let mut strategies = Vec::new();

        // 🧠 INTELLIGENTE ALLGEMEINE STRATEGIEN
        // Basierend auf Website-Eigenschaften, nicht spezifischen Domains
        
        let is_banking = Self::is_banking_site(&domain);
        let is_government = Self::is_government_site(&domain);
        let is_russian = domain.contains(".ru") || domain.contains("yandex") || domain.contains("mail.ru");
        let is_chinese = domain.contains(".cn") || domain.contains("baidu") || domain.contains("weibo");
        let is_developer_site = domain.contains("github") || domain.contains("gitlab") || domain.contains("stackoverflow") || domain.contains("dev");

        // 🎯 STRATEGIE 1: NATIVE BROWSER SIMULATION (universell für alle Websites)
        strategies.push(ConnectionStrategy {
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
        });

        // 🌍 STRATEGIE 2: REGIONALE BROWSER (für regionale Websites)
        if is_russian {
            strategies.push(ConnectionStrategy {
                description: "Regional Browser (Russian)".to_string(),
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
            });
        } else if is_chinese {
            strategies.push(ConnectionStrategy {
                description: "Regional Browser (Chinese)".to_string(),
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
            });
        }

        // 🔒 STRATEGIE 3: SICHERHEITSORIENTIERT (für Banking/Government)
        if is_banking || is_government {
            strategies.push(ConnectionStrategy {
                description: "Secure Browser (Banking/Gov)".to_string(),
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
                ],
            });
        }

        // 👨‍💻 STRATEGIE 4: ENTWICKLER-ORIENTIERT (für Developer-Sites)
        if is_developer_site {
            strategies.push(ConnectionStrategy {
                description: "Developer Tools Browser".to_string(),
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
            });
        }

        // 📱 STRATEGIE 5: MOBILE FALLBACK (für alle Websites)
        strategies.push(ConnectionStrategy {
            description: "Mobile Browser Fallback".to_string(),
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
            ],
        });

        // 🌐 STRATEGIE 6: STANDARD CHROME (universell)
        strategies.push(ConnectionStrategy {
            description: "Standard Chrome Browser".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            timeout: 30,
            connect_timeout: 10,
            redirects: 5,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8"),
                ("Accept-Language", "en-US,en;q=0.9,de;q=0.8"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
                ("Sec-Fetch-Dest", "document"),
                ("Sec-Fetch-Mode", "navigate"),
                ("Sec-Fetch-Site", "none"),
                ("Sec-Fetch-User", "?1"),
            ],
        });

        // 🦊 STRATEGIE 7: FIREFOX FALLBACK
        strategies.push(ConnectionStrategy {
            description: "Firefox Browser Fallback".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0".to_string(),
            timeout: 25,
            connect_timeout: 8,
            redirects: 3,
            accept_invalid_certs: false,
            use_http2: true,
            headers: vec![
                ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"),
                ("Accept-Language", "en-US,en;q=0.5"),
                ("Accept-Encoding", "gzip, deflate, br"),
                ("Connection", "keep-alive"),
                ("Upgrade-Insecure-Requests", "1"),
            ],
        });

        // 🔄 STRATEGIE 8: HTTP FALLBACK (für HTTPS-Probleme)
        if url.starts_with("https://") {
            strategies.push(ConnectionStrategy {
                description: "HTTP Fallback".to_string(),
                user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                timeout: 20,
                connect_timeout: 8,
                redirects: 3,
                accept_invalid_certs: true, // Für HTTP-Fallback
                use_http2: false,
                headers: vec![
                    ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
                    ("Accept-Language", "en-US,en;q=0.9"),
                    ("Accept-Encoding", "gzip, deflate"),
                    ("Connection", "keep-alive"),
                ],
            });
        }

        println!("🎯 Generated {} adaptive strategies for {}", strategies.len(), domain);
        strategies
    }

    fn extract_domain(url: &str) -> String {
        if let Ok(parsed_url) = url::Url::parse(url) {
            parsed_url.host_str().unwrap_or("").to_lowercase()
        } else {
            String::new()
        }
    }

    fn is_banking_site(domain: &str) -> bool {
        let banking_patterns = [
            "bank", "sparkasse", "volksbank", "commerzbank", "deutsche-bank",
            "ing", "dkb", "comdirect", "consorsbank", "1822direkt",
            "paypal", "klarna", "stripe", "wise", "revolut",
            "chase", "wellsfargo", "bankofamerica", "citibank",
            "hsbc", "barclays", "lloyds", "santander",
            "bnpparibas", "creditsuisse", "ubs"
        ];
        
        banking_patterns.iter().any(|pattern| domain.contains(pattern))
    }
    
    fn is_government_site(domain: &str) -> bool {
        let government_patterns = [
            // Deutsche Behörden
            "bund.de", "bundesregierung.de", "bundestag.de", "bundesrat.de",
            "elster.de", "personalausweis.de", "service-bw.de",
            
            // Internationale Regierungen
            "gov.uk", "gov.fr", "gov.it", "gov.es", "gov.au", "gov.ca",
            "whitehouse.gov", "congress.gov", "senate.gov",
            
            // EU-Institutionen
            "europa.eu", "ec.europa.eu", "europarl.europa.eu"
        ];
        
        government_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    #[allow(dead_code)]
    fn has_csp_frame_issues(domain: &str) -> bool {
        let csp_problematic_domains = [
            // Bekannte CSP frame-ancestors Probleme
            "dzen.ru", "sso.dzen.ru", "passport.yandex",
            "accounts.google", "login.microsoft", "auth.apple",
            "facebook.com", "instagram.com", "twitter.com", "x.com",
            
            // SSO und Auth-Services
            "okta", "auth0", "onelogin", "ping", "adfs",
            "saml", "oauth", "openid", "sso", "login",
            
            // Banking SSO
            "banking", "secure", "portal", "customer",
            "online-banking", "ebanking", "netbanking",
            
            // Government SSO
            "bund.de", "gov.uk", "gov.fr", "gov.it",
            "elster", "personalausweis", "id.gov"
        ];
        
        csp_problematic_domains.iter().any(|pattern| domain.contains(pattern))
    }

    async fn try_strategy(url: &str, strategy: &ConnectionStrategy) -> Result<reqwest::Response, Box<dyn std::error::Error + Send + Sync>> {
        let mut client_builder = reqwest::Client::builder()
            .user_agent(&strategy.user_agent)
            .timeout(std::time::Duration::from_secs(strategy.timeout))
            .connect_timeout(std::time::Duration::from_secs(strategy.connect_timeout))
            .redirect(reqwest::redirect::Policy::limited(strategy.redirects))
            .danger_accept_invalid_certs(strategy.accept_invalid_certs)
            .use_rustls_tls();

        if strategy.use_http2 {
            client_builder = client_builder.http2_prior_knowledge();
        } else {
            client_builder = client_builder.http1_only();
        }

        let client = client_builder.build()?;
        let url_variants = Self::generate_url_variants(url, strategy);
        
        for variant in url_variants {
            let mut request = client.get(&variant);
            
            for (key, value) in &strategy.headers {
                request = request.header(*key, *value);
            }
            
            request = request
                .header("DNT", "1")
                .header("Cache-Control", "no-cache")
                .header("Pragma", "no-cache");
            
            match request.send().await {
                Ok(response) => {
                    if response.status().is_success() || response.status().is_redirection() {
                        println!("🎯 URL variant {} succeeded with status: {}", variant, response.status());
                        return Ok(response);
                    }
                }
                Err(e) => {
                    println!("🔄 URL variant {} failed: {}", variant, e);
                    continue;
                }
            }
        }

        Err("All URL variants failed for this strategy".into())
    }

    fn generate_url_variants(url: &str, strategy: &ConnectionStrategy) -> Vec<String> {
        let mut variants = vec![url.to_string()];
        
        // 🌐 ALLGEMEINE URL-VARIANTEN (für alle Websites)
        if let Ok(parsed_url) = url::Url::parse(url) {
            let host = parsed_url.host_str().unwrap_or("");
            let path = parsed_url.path();
            let query = parsed_url.query().unwrap_or("");
            
            // 🔄 HTTP/HTTPS Varianten
            if strategy.description.contains("HTTP Fallback") && url.starts_with("https://") {
                let http_url = format!("http://{}{}{}", 
                    host, 
                    path,
                    if query.is_empty() { "".to_string() } else { format!("?{}", query) }
                );
                variants.push(http_url);
            }
            
            // 🌍 WWW Varianten
            if !host.starts_with("www.") && !host.starts_with("m.") && !host.is_empty() {
                let www_url = format!("{}://www.{}{}{}", 
                    parsed_url.scheme(), 
                    host, 
                    path,
                    if query.is_empty() { "".to_string() } else { format!("?{}", query) }
                );
                variants.push(www_url);
            }
            
            if host.starts_with("www.") {
                let no_www_host = &host[4..];
                let no_www_url = format!("{}://{}{}{}", 
                    parsed_url.scheme(), 
                    no_www_host, 
                    path,
                    if query.is_empty() { "".to_string() } else { format!("?{}", query) }
                );
                variants.push(no_www_url);
            }
            
            // 📱 Mobile Varianten (für Mobile-Strategien)
            if strategy.description.contains("Mobile") && !host.starts_with("m.") {
                let mobile_url = format!("{}://m.{}{}{}", 
                    parsed_url.scheme(), 
                    host.replace("www.", ""), 
                    path,
                    if query.is_empty() { "".to_string() } else { format!("?{}", query) }
                );
                variants.push(mobile_url);
            }
            
            // 🌍 Regionale Varianten (für regionale Browser)
            if strategy.description.contains("Russian") {
                // .com zu .ru und umgekehrt
                if host.ends_with(".com") {
                    let ru_host = host.replace(".com", ".ru");
                    let ru_url = format!("{}://{}{}{}", 
                        parsed_url.scheme(), 
                        ru_host, 
                        path,
                        if query.is_empty() { "".to_string() } else { format!("?{}", query) }
                    );
                    variants.push(ru_url);
                }
                if host.ends_with(".ru") {
                    let com_host = host.replace(".ru", ".com");
                    let com_url = format!("{}://{}{}{}", 
                        parsed_url.scheme(), 
                        com_host, 
                        path,
                        if query.is_empty() { "".to_string() } else { format!("?{}", query) }
                    );
                    variants.push(com_url);
                }
            }
            
            // 🔒 Sichere Varianten (für Banking/Government)
            if strategy.description.contains("Secure") && !url.starts_with("https://") {
                let https_url = url.replace("http://", "https://");
                variants.push(https_url);
            }
        }
        
        // Duplikate entfernen und begrenzen
        variants.sort();
        variants.dedup();
        variants.truncate(6); // Maximal 6 Varianten für bessere Performance
        
        if variants.len() > 1 {
            println!("🎯 Generated {} URL variants", variants.len());
        }
        
        variants
    }

    async fn process_response(response: reqwest::Response, url: &str) -> Result<ProxyResponse, Box<dyn std::error::Error + Send + Sync>> {
        let status_code = response.status().as_u16();
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("text/html")
            .to_string();

        println!("📡 Response status: {}", status_code);

        let mut content = response.text().await?;
        
        // Prüfe auf CAPTCHA-Seiten
        if Self::is_captcha_page(&content) {
            println!("🤖 CAPTCHA-Seite erkannt, erstelle Benutzerfreundliche Nachricht...");
            content = Self::create_captcha_bypass_page(url, &content);
        } else {
            content = Self::strip_all_iframe_blocks(&content, url);
            // 🍪 Füge Cookie-Banner Auto-Handler hinzu
            content = Self::inject_cookie_acceptance_script(&content);
        }
        
        println!("✅ Successfully proxied {} ({} bytes, headers stripped, cookie handling added)", url, content.len());

        Ok(ProxyResponse {
            content,
            content_type,
            status_code,
        })
    }
    
    fn is_captcha_page(content: &str) -> bool {
        let content_lower = content.to_lowercase();
        
        // 🚫 DEAKTIVIERE CAPTCHA-ERKENNUNG FÜR NORMALE WEBSITES
        // Nur bei sehr offensichtlichen Challenge-Seiten aktivieren
        
        // Sehr spezifische Cloudflare-Challenge-Seiten (vollständige Seiten, nicht eingebettete Elemente)
        let is_cloudflare_challenge = content_lower.contains("checking your browser before accessing") && 
                                     content_lower.contains("cloudflare") &&
                                     content_lower.contains("please wait") &&
                                     content.len() < 10000; // Challenge-Seiten sind meist klein
        
        // Nur echte CAPTCHA-Challenge-Seiten (nicht normale Seiten mit CAPTCHA-Elementen)
        let is_pure_captcha_challenge = (content_lower.contains("complete the captcha") ||
                                        content_lower.contains("solve the captcha") ||
                                        content_lower.contains("verify you are human")) &&
                                       content.len() < 15000 && // Challenge-Seiten sind klein
                                       !content_lower.contains("stackoverflow") && // Normale Websites ausschließen
                                       !content_lower.contains("github") &&
                                       !content_lower.contains("google") &&
                                       !content_lower.contains("youtube");
        
        // Nur als CAPTCHA behandeln wenn es eine reine Challenge-Seite ist
        let is_captcha = is_cloudflare_challenge || is_pure_captcha_challenge;
        
        if is_captcha {
            println!("🤖 Echte CAPTCHA-Challenge-Seite erkannt (Größe: {} bytes)", content.len());
        } else if content_lower.contains("recaptcha") || content_lower.contains("captcha") {
            println!("✅ Normale Website mit CAPTCHA-Elementen erkannt - wird NICHT blockiert");
        }
        
        is_captcha
    }
    
    fn create_captcha_bypass_page(original_url: &str, original_content: &str) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html><html><head><meta charset=\"UTF-8\">");
        html.push_str("<title>Anti-Bot Protection Detected - Ora Browser</title>");
        html.push_str("<style>");
        html.push_str("body{font-family:'Segoe UI',Tahoma,Geneva,Verdana,sans-serif;background:linear-gradient(135deg,#667eea 0%,#764ba2 100%);color:white;text-align:center;padding:40px;margin:0;}");
        html.push_str(".container{background:rgba(255,255,255,0.1);backdrop-filter:blur(10px);padding:40px;border-radius:20px;max-width:700px;margin:0 auto;box-shadow:0 8px 32px rgba(0,0,0,0.3);}");
        html.push_str(".btn{display:inline-block;padding:15px 30px;margin:10px;background:#4CAF50;color:white;text-decoration:none;border-radius:10px;font-weight:bold;transition:all 0.3s ease;box-shadow:0 4px 15px rgba(0,0,0,0.2);}");
        html.push_str(".btn:hover{background:#45a049;transform:translateY(-2px);box-shadow:0 6px 20px rgba(0,0,0,0.3);}");
        html.push_str(".btn.secondary{background:#ff6b6b;}.btn.secondary:hover{background:#ff5252;}");
        html.push_str(".btn.tertiary{background:#4ecdc4;}.btn.tertiary:hover{background:#26d0ce;}");
        html.push_str(".info-section{text-align:left;margin-top:30px;background:rgba(255,255,255,0.05);padding:20px;border-radius:10px;}");
        html.push_str(".protection-type{background:#ff9800;color:white;padding:5px 15px;border-radius:20px;font-size:12px;font-weight:bold;display:inline-block;margin-bottom:15px;}");
        html.push_str("</style></head><body><div class=\"container\">");
        
        // Erkenne Art des Schutzes
        let protection_type = if original_content.to_lowercase().contains("cloudflare") {
            "🛡️ CLOUDFLARE PROTECTION"
        } else if original_content.to_lowercase().contains("smartcaptcha") || original_content.to_lowercase().contains("yandex") {
            "🤖 YANDEX SMARTCAPTCHA"
        } else if original_content.to_lowercase().contains("ddos") {
            "⚡ DDOS PROTECTION"
        } else {
            "🔒 ANTI-BOT PROTECTION"
        };
        
        html.push_str(&format!("<div class=\"protection-type\">{}</div>", protection_type));
        html.push_str("<h1>🚫 Anti-Bot Protection Detected</h1>");
        html.push_str(&format!("<p><strong>{}</strong> uses advanced bot protection and requires human verification.</p>", original_url));
        
        html.push_str("<div>");
        html.push_str(&format!("<a href=\"#\" onclick=\"window.open('{}', '_blank')\" class=\"btn\">🌐 Open in Browser</a>", original_url));
        html.push_str("<a href=\"#\" onclick=\"location.reload()\" class=\"btn secondary\">🔄 Try Again</a>");
        html.push_str("<a href=\"#\" onclick=\"history.back()\" class=\"btn tertiary\">⬅️ Go Back</a>");
        html.push_str("</div>");
        
        html.push_str("<div class=\"info-section\">");
        html.push_str("<h3>🤔 Why does this happen?</h3>");
        html.push_str("<p><strong>🛡️ Advanced Protection:</strong> Modern websites use sophisticated anti-bot systems (Cloudflare, Yandex SmartCaptcha, etc.) to detect automated requests.</p>");
        html.push_str("<p><strong>🔍 Proxy Detection:</strong> The Ora Browser proxy is recognized as an automated system by these protection services.</p>");
        html.push_str("<p><strong>🎯 Common Triggers:</strong> High request frequency, missing browser fingerprints, or suspicious traffic patterns.</p>");
        html.push_str("<p><strong>✅ Solution:</strong> Click 'Open in Browser' to load the page directly in your default browser where you can complete the verification.</p>");
        html.push_str("</div>");
        
        html.push_str("<div class=\"info-section\">");
        html.push_str("<h3>🔧 Technical Details</h3>");
        html.push_str("<p><strong>Protection System:</strong> ");
        html.push_str(protection_type);
        html.push_str("</p>");
        html.push_str("<p><strong>URL:</strong> ");
        html.push_str(original_url);
        html.push_str("</p>");
        html.push_str("<p><strong>Bypass Status:</strong> Manual verification required</p>");
        html.push_str("</div>");
        
        html.push_str("</div></body></html>");
        html
    }

    fn strip_all_iframe_blocks(html: &str, _url: &str) -> String {
        let mut cleaned = html.to_string();

        // 🛡️ MINIMALE FRAME-BEHANDLUNG (Cookie-Banner bleiben vollständig funktional)
        println!("🧹 Minimal iframe processing for cookie-banner compatibility...");
        
        // Entferne NUR X-Frame-Options Header (nicht Meta-Tags)
        // Lasse alle CSP-Regeln und JavaScript intakt
        
        // Nur diese eine Änderung: Entferne X-Frame-Options Meta-Tags
        let xframe_pattern = r#"<meta[^>]*http-equiv\s*=\s*["']?X-Frame-Options["']?[^>]*>"#;
        if let Ok(re) = regex::Regex::new(xframe_pattern) {
            cleaned = re.replace_all(&cleaned, "<!-- X-Frame-Options removed -->").to_string();
        }

        // Füge nur minimale Kompatibilität hinzu (ohne Cookie-Banner zu beeinträchtigen)
        let minimal_script = r#"
<script>
// 🛡️ MINIMALE KOMPATIBILITÄT (Cookie-Banner bleiben unberührt)
(function() {
    try {
        // Nur frameElement auf null setzen (alles andere bleibt)
        if (window.frameElement) {
            Object.defineProperty(window, 'frameElement', { 
                value: null, 
                configurable: false 
            });
        }
        console.log('✅ Minimal compatibility loaded - Cookie banners fully functional');
    } catch (e) {
        // Ignoriere alle Fehler
    }
})();
</script>"#;

        // Füge Script nur am Ende hinzu (nicht in head)
        if let Some(body_end) = cleaned.rfind("</body>") {
            cleaned.insert_str(body_end, minimal_script);
        } else {
            cleaned.push_str(minimal_script);
        }

        println!("✅ Minimal processing complete - Cookie banners should work perfectly");
        cleaned
    }

    fn inject_cookie_acceptance_script(html: &str) -> String {
        let cookie_script = r#"
<script>
// 🍪 COOKIE-BANNER-KOMPATIBILITÄT + POPUP-BLOCKIERUNG
(function() {
    console.log('🍪 Ora Browser - Cookie-Banner-Kompatibilität + Popup-Kontrolle aktiv');
    
    // 🚫 POPUP-BLOCKIERUNG UND WINDOW-KONTROLLE
    function blockPopupsAndRedirects() {
        try {
            // Überschreibe window.open (verhindert neue Fenster)
            const originalWindowOpen = window.open;
            window.open = function(url, name, features) {
                console.log('🚫 Popup blockiert:', url);
                
                // Statt neues Fenster zu öffnen, navigiere im aktuellen Fenster
                if (url && url !== 'about:blank') {
                    // Proxye die URL durch unseren Browser
                    const proxyUrl = `http://localhost:3030/proxy?url=${encodeURIComponent(url)}`;
                    window.location.href = proxyUrl;
                }
                
                // Gib ein Dummy-Fenster zurück
                return {
                    closed: false,
                    close: () => {},
                    focus: () => {},
                    blur: () => {},
                    postMessage: () => {},
                    location: { href: url || 'about:blank' }
                };
            };
            
            // Überschreibe window.location.assign und replace
            const originalAssign = window.location.assign;
            const originalReplace = window.location.replace;
            
            window.location.assign = function(url) {
                console.log('🔄 Navigation abgefangen (assign):', url);
                const proxyUrl = `http://localhost:3030/proxy?url=${encodeURIComponent(url)}`;
                return originalAssign.call(this, proxyUrl);
            };
            
            window.location.replace = function(url) {
                console.log('🔄 Navigation abgefangen (replace):', url);
                const proxyUrl = `http://localhost:3030/proxy?url=${encodeURIComponent(url)}`;
                return originalReplace.call(this, proxyUrl);
            };
            
            // Blockiere automatische Redirects durch Meta-Tags
            const metaRefreshElements = document.querySelectorAll('meta[http-equiv="refresh"]');
            metaRefreshElements.forEach(meta => {
                console.log('🚫 Meta-Refresh blockiert:', meta.getAttribute('content'));
                meta.remove();
            });
            
            // Überwache neue Meta-Refresh-Tags
            const observer = new MutationObserver(mutations => {
                mutations.forEach(mutation => {
                    mutation.addedNodes.forEach(node => {
                        if (node.nodeType === 1) { // Element node
                            if (node.tagName === 'META' && node.getAttribute('http-equiv') === 'refresh') {
                                console.log('🚫 Dynamisches Meta-Refresh blockiert');
                                node.remove();
                            }
                            // Suche auch in Kindern
                            const metaRefresh = node.querySelectorAll && node.querySelectorAll('meta[http-equiv="refresh"]');
                            if (metaRefresh) {
                                metaRefresh.forEach(meta => {
                                    console.log('🚫 Dynamisches Meta-Refresh in Kind blockiert');
                                    meta.remove();
                                });
                            }
                        }
                    });
                });
            });
            
            observer.observe(document.head || document.documentElement, {
                childList: true,
                subtree: true
            });
            
            console.log('✅ Popup-Blockierung und Redirect-Kontrolle aktiviert');
        } catch (e) {
            console.log('❌ Popup-Blockierung fehlgeschlagen:', e);
        }
    }
    
    // 🌐 DOMAIN-SPOOFING FÜR COOKIE-BANNER
    function setupDomainSpoofing() {
        try {
            // Extrahiere echte URL aus Proxy-URL
            const urlParams = new URLSearchParams(window.location.search);
            const realUrl = urlParams.get('url');
            
            if (realUrl) {
                const realUrlObj = new URL(realUrl);
                
                // Überschreibe window.location für Cookie-Banner
                const fakeLocation = {
                    href: realUrl,
                    hostname: realUrlObj.hostname,
                    host: realUrlObj.host,
                    origin: realUrlObj.origin,
                    protocol: realUrlObj.protocol,
                    pathname: realUrlObj.pathname,
                    search: realUrlObj.search,
                    hash: realUrlObj.hash,
                    port: realUrlObj.port,
                    toString: () => realUrl,
                    assign: (url) => {
                        console.log('🔄 Location.assign abgefangen:', url);
                        const proxyUrl = `http://localhost:3030/proxy?url=${encodeURIComponent(url)}`;
                        window.location.assign(proxyUrl);
                    },
                    replace: (url) => {
                        console.log('🔄 Location.replace abgefangen:', url);
                        const proxyUrl = `http://localhost:3030/proxy?url=${encodeURIComponent(url)}`;
                        window.location.replace(proxyUrl);
                    },
                    reload: () => window.location.reload()
                };
                
                // Überschreibe document.domain
                try {
                    Object.defineProperty(document, 'domain', {
                        value: realUrlObj.hostname,
                        writable: false,
                        configurable: false
                    });
                } catch (e) {
                    console.log('⚠️ Could not override document.domain:', e);
                }
                
                console.log('✅ Domain spoofing setup:', realUrlObj.hostname);
            }
        } catch (e) {
            console.log('❌ Domain spoofing failed:', e);
        }
    }
    
    // 🔄 AJAX-REQUEST-PROXYING FÜR COOKIE-BANNER
    function setupAjaxProxying() {
        try {
            // Überschreibe XMLHttpRequest
            const originalXHR = window.XMLHttpRequest;
            window.XMLHttpRequest = function() {
                const xhr = new originalXHR();
                const originalOpen = xhr.open;
                
                xhr.open = function(method, url, async, user, password) {
                    // Wenn es ein relativer oder same-origin Request ist, proxye ihn
                    if (url.startsWith('/') || url.startsWith('./') || !url.includes('://')) {
                        const urlParams = new URLSearchParams(window.location.search);
                        const realUrl = urlParams.get('url');
                        if (realUrl) {
                            const realUrlObj = new URL(realUrl);
                            const fullUrl = new URL(url, realUrlObj.origin).href;
                            url = `http://localhost:3030/proxy?url=${encodeURIComponent(fullUrl)}`;
                        }
                    }
                    return originalOpen.call(this, method, url, async, user, password);
                };
                
                return xhr;
            };
            
            // Überschreibe fetch
            const originalFetch = window.fetch;
            window.fetch = function(input, init) {
                let url = typeof input === 'string' ? input : input.url;
                
                // Proxye relative URLs
                if (url.startsWith('/') || url.startsWith('./') || !url.includes('://')) {
                    const urlParams = new URLSearchParams(window.location.search);
                    const realUrl = urlParams.get('url');
                    if (realUrl) {
                        const realUrlObj = new URL(realUrl);
                        const fullUrl = new URL(url, realUrlObj.origin).href;
                        url = `http://localhost:3030/proxy?url=${encodeURIComponent(fullUrl)}`;
                    }
                }
                
                return originalFetch.call(this, url, init);
            };
            
            console.log('✅ AJAX proxying setup complete');
        } catch (e) {
            console.log('❌ AJAX proxying failed:', e);
        }
    }
    
    // 🍪 COOKIE-HANDLING-VERBESSERUNG
    function enhanceCookieHandling() {
        try {
            // Überschreibe document.cookie für bessere Kompatibilität
            const originalCookieDescriptor = Object.getOwnPropertyDescriptor(Document.prototype, 'cookie') || 
                                           Object.getOwnPropertyDescriptor(HTMLDocument.prototype, 'cookie');
            
            if (originalCookieDescriptor) {
                Object.defineProperty(document, 'cookie', {
                    get: function() {
                        return originalCookieDescriptor.get.call(this);
                    },
                    set: function(value) {
                        // Modifiziere Cookie-Domain für bessere Kompatibilität
                        const urlParams = new URLSearchParams(window.location.search);
                        const realUrl = urlParams.get('url');
                        if (realUrl) {
                            const realUrlObj = new URL(realUrl);
                            // Füge Domain hinzu wenn nicht vorhanden
                            if (!value.includes('domain=')) {
                                value += `; domain=${realUrlObj.hostname}`;
                            }
                        }
                        return originalCookieDescriptor.set.call(this, value);
                    },
                    configurable: true
                });
            }
            
            console.log('✅ Cookie handling enhanced');
        } catch (e) {
            console.log('❌ Cookie handling enhancement failed:', e);
        }
    }
    
    // 🚀 INITIALISIERUNG (Popup-Blockierung zuerst!)
    blockPopupsAndRedirects();
    setupDomainSpoofing();
    setupAjaxProxying();
    enhanceCookieHandling();
    
    // Warte auf DOM-Ready
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => {
            setTimeout(() => {
                blockPopupsAndRedirects();
                setupDomainSpoofing();
                setupAjaxProxying();
            }, 100);
        });
    }
    
    console.log('✅ Cookie-Banner-Kompatibilität + Popup-Kontrolle vollständig aktiviert!');
    
})();
</script>
"#;
        
        // Füge Script am Anfang des Head hinzu (sehr früh)
        if let Some(head_start) = html.find("<head>") {
            let mut result = html.to_string();
            result.insert_str(head_start + 6, cookie_script);
            result
        } else if let Some(html_start) = html.find("<html>") {
            let mut result = html.to_string();
            result.insert_str(html_start + 6, &format!("<head>{}</head>", cookie_script));
            result
        } else {
            format!("{}{}", cookie_script, html)
        }
    }

    #[allow(dead_code)]
    fn is_major_website(domain: &str) -> bool {
        let major_patterns = [
            "google", "yahoo", "bing", "microsoft", "github", "stackoverflow",
            "wikipedia", "facebook", "twitter", "youtube", "amazon", "apple",
            "netflix", "reddit", "linkedin", "instagram", "tiktok", "discord",
            "dzen", "yandex", "baidu", "qq", "weibo", "vk", "consent",
            "cloudflare", "akamai", "fastly", "jsdelivr", "cdnjs"
        ];
        
        major_patterns.iter().any(|pattern| domain.contains(pattern))
    }
    
    #[allow(dead_code)]
    fn is_social_media_site(domain: &str) -> bool {
        let social_patterns = [
            "facebook", "instagram", "twitter", "x.com", "linkedin", 
            "tiktok", "snapchat", "pinterest", "reddit", "discord",
            "telegram", "whatsapp", "youtube", "vimeo", "twitch"
        ];
        
        social_patterns.iter().any(|pattern| domain.contains(pattern))
    }
}
