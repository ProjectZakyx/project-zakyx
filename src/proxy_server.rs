use reqwest;
use std::collections::HashMap;
use tokio::sync::mpsc;
use warp::Filter;
use chrono;
use regex;

#[derive(Debug, Clone)]
pub struct ProxyResponse {
    pub content: String,
    #[allow(dead_code)]
    pub content_type: String,
    #[allow(dead_code)]
    pub status_code: u16,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
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

#[derive(Debug)]
pub struct ProxyServer {
    port: u16,
    #[allow(dead_code)]
    sender: Option<mpsc::UnboundedSender<ProxyResponse>>,
}

impl ProxyServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            sender: None,
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("🌐 Starting Ora Browser Proxy Server on port {}...", self.port);
        
        // CORS-Handler für Preflight-Requests
        let cors = warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type", "authorization", "x-requested-with", "accept", "origin", "cache-control", "pragma"])
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS", "HEAD"])
            .allow_credentials(true);
        
        // Proxy-Route mit verbesserter CORS-Behandlung
        let proxy_route = warp::path("proxy")
            .and(warp::query::<std::collections::HashMap<String, String>>())
            .and_then(|params: std::collections::HashMap<String, String>| async move {
                if let Some(url) = params.get("url") {
                    Self::handle_proxy(url.clone()).await
                } else {
                    let reply = warp::reply::with_status("Missing URL parameter", warp::http::StatusCode::BAD_REQUEST);
                    Ok(Box::new(reply) as Box<dyn warp::Reply>)
                }
            })
            .with(cors.clone());
        
        // Asset-Route für direkte Ressourcen
        let assets_route = warp::path("assets")
            .and(warp::path::tail())
            .and_then(Self::handle_assets)
            .with(cors.clone());
        
        // Health-Check-Route
        let health_route = warp::path("health")
            .map(|| {
                warp::reply::json(&serde_json::json!({
                    "status": "ok",
                    "service": "Ora Browser Proxy",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }))
            })
            .with(cors.clone());
        
        // Kombiniere alle Routen
        let routes = proxy_route
            .or(assets_route)
            .or(health_route)
            .with(warp::log("proxy"));
        
        println!("✅ Proxy server routes configured");
        println!("🔗 Proxy URL: http://localhost:{}/proxy?url=<URL>", self.port);
        println!("🏥 Health check: http://localhost:{}/health", self.port);
        
        // Starte den Server
        warp::serve(routes)
            .run(([127, 0, 0, 1], self.port))
            .await;
        
        Ok(())
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    async fn handle_proxy_request_with_cors(
        params: HashMap<String, String>,
        method: warp::http::Method,
        headers: warp::http::HeaderMap,
    ) -> Result<warp::http::Response<String>, warp::http::StatusCode> {
        let origin = headers.get("origin")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("null");
        
        let allow_origin = if origin == "null" || origin.is_empty() {
            "http://localhost:3030"
        } else {
            origin
        };
        
        if method == warp::http::Method::OPTIONS {
            let response = warp::http::Response::builder()
                .status(200)
                .header("Access-Control-Allow-Origin", allow_origin)
                .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
                .header("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With, Accept, Origin, Cookie, Cache-Control, Pragma, User-Agent, Accept-Language, Accept-Encoding, DNT, Connection, Upgrade-Insecure-Requests, Sec-Fetch-Dest, Sec-Fetch-Mode, Sec-Fetch-Site, Sec-Fetch-User")
                .header("Access-Control-Allow-Credentials", "true")
                .header("Access-Control-Max-Age", "86400")
                .header("Content-Length", "0")
                .body("".to_string()).unwrap();
            
            return Ok(response);
        }

        let url = params.get("url").unwrap_or(&"https://google.com".to_string()).clone();
        let request_method = params.get("method").unwrap_or(&"GET".to_string()).clone();
        let form_data = params.get("data").unwrap_or(&"".to_string()).clone();
        
        println!("🔄 Proxying {} request to: {}", request_method, url);
        if !form_data.is_empty() {
            println!("📝 With form data: {}", form_data);
        }

        match Self::fetch_with_method(&url, &request_method, &form_data).await {
            Ok(proxy_response) => {
                let enhanced_content = Self::inject_cors_and_security_fixes(&proxy_response.content, &url);
                
                let response = warp::http::Response::builder()
                    .status(200)
                    .header("Content-Type", "text/html; charset=utf-8")
                    // 🌐 ERWEITERTE CORS-HEADER
                    .header("Access-Control-Allow-Origin", allow_origin)
                    .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS, PATCH, HEAD")
                    .header("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With, Accept, Origin, Cookie, Cache-Control, Pragma, User-Agent, Accept-Language, Accept-Encoding, DNT, Connection, Upgrade-Insecure-Requests, Sec-Fetch-Dest, Sec-Fetch-Mode, Sec-Fetch-Site, Sec-Fetch-User, X-Forwarded-For, X-Real-IP, Referer")
                    .header("Access-Control-Allow-Credentials", "true")
                    .header("Access-Control-Expose-Headers", "*")
                    .header("Access-Control-Max-Age", "86400")
                    .header("Vary", "Origin")
                    // 🛡️ EXPLIZITE CSP-BYPASS-HEADER
                    .header("Content-Security-Policy", "default-src * 'unsafe-inline' 'unsafe-eval' data: blob:; script-src * 'unsafe-inline' 'unsafe-eval'; style-src * 'unsafe-inline'; img-src * data: blob:; font-src * data:; connect-src * data: blob:; media-src * data: blob:; object-src *; child-src * data: blob:; frame-src * data: blob:; worker-src * data: blob:; frame-ancestors *; form-action *; base-uri *;")
                    .header("X-Frame-Options", "ALLOWALL")
                    .header("X-Content-Type-Options", "nosniff")
                    .header("Referrer-Policy", "no-referrer-when-downgrade")
                    .header("Permissions-Policy", "accelerometer=*, camera=*, geolocation=*, gyroscope=*, magnetometer=*, microphone=*, payment=*, usb=*")
                    // 🚀 ZUSÄTZLICHE SICHERHEITS-BYPASS-HEADER
                    .header("Cross-Origin-Embedder-Policy", "unsafe-none")
                    .header("Cross-Origin-Opener-Policy", "unsafe-none")
                    .header("Cross-Origin-Resource-Policy", "cross-origin")
                    .header("X-Permitted-Cross-Domain-Policies", "all")
                    .header("X-XSS-Protection", "0")
                    .body(enhanced_content).unwrap();
                
                Ok(response)
            },
            Err(e) => {
                println!("❌ Proxy request failed: {}", e);
                let error_page = Self::create_error_page(&url, &e.to_string());
                
                let response = warp::http::Response::builder()
                    .status(500)
                    .header("Content-Type", "text/html; charset=utf-8")
                    // 🌐 ERWEITERTE CORS-HEADER
                    .header("Access-Control-Allow-Origin", allow_origin)
                    .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS, PATCH, HEAD")
                    .header("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With, Accept, Origin, Cookie, Cache-Control, Pragma, User-Agent, Accept-Language, Accept-Encoding, DNT, Connection, Upgrade-Insecure-Requests, Sec-Fetch-Dest, Sec-Fetch-Mode, Sec-Fetch-Site, Sec-Fetch-User, X-Forwarded-For, X-Real-IP, Referer")
                    .header("Access-Control-Allow-Credentials", "true")
                    .header("Access-Control-Expose-Headers", "*")
                    .header("Access-Control-Max-Age", "86400")
                    .header("Vary", "Origin")
                    // 🛡️ EXPLIZITE CSP-BYPASS-HEADER
                    .header("Content-Security-Policy", "default-src * 'unsafe-inline' 'unsafe-eval' data: blob:; script-src * 'unsafe-inline' 'unsafe-eval'; style-src * 'unsafe-inline'; img-src * data: blob:; font-src * data:; connect-src * data: blob:; media-src * data: blob:; object-src *; child-src * data: blob:; frame-src * data: blob:; worker-src * data: blob:; frame-ancestors *; form-action *; base-uri *;")
                    .header("X-Frame-Options", "ALLOWALL")
                    .header("X-Content-Type-Options", "nosniff")
                    .header("Referrer-Policy", "no-referrer-when-downgrade")
                    .header("Permissions-Policy", "accelerometer=*, camera=*, geolocation=*, gyroscope=*, magnetometer=*, microphone=*, payment=*, usb=*")
                    // 🚀 ZUSÄTZLICHE SICHERHEITS-BYPASS-HEADER
                    .header("Cross-Origin-Embedder-Policy", "unsafe-none")
                    .header("Cross-Origin-Opener-Policy", "unsafe-none")
                    .header("Cross-Origin-Resource-Policy", "cross-origin")
                    .header("X-Permitted-Cross-Domain-Policies", "all")
                    .header("X-XSS-Protection", "0")
                    .body(error_page).unwrap();
                
                Ok(response)
            }
        }
    }

    async fn handle_assets(
        path: warp::path::Tail,
    ) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
        let path_str = path.as_str();
        println!("🔗 Asset request: {}", path_str);
        
        let asset_url = format!("https://{}", path_str);
        
        match Self::fetch_and_strip_headers(&asset_url).await {
            Ok(response) => {
                let content_type = if path_str.ends_with(".js") {
                    "application/javascript"
                } else if path_str.ends_with(".css") {
                    "text/css"
                } else if path_str.ends_with(".webp") || path_str.ends_with(".png") || path_str.ends_with(".jpg") {
                    "image/*"
                } else {
                    &response.content_type
                };
                
                let reply = warp::reply::with_header(
                    warp::reply::with_header(
                        warp::reply::with_header(
                            warp::reply::with_header(
                                response.content,
                                "Access-Control-Allow-Origin", "http://localhost:3030"
                            ),
                            "Content-Type", content_type
                        ),
                        "Access-Control-Allow-Methods", "GET, POST, OPTIONS"
                    ),
                    "Access-Control-Allow-Credentials", "true"
                );
                
                Ok(Box::new(reply))
            }
            Err(e) => {
                println!("❌ Failed to fetch asset {}: {}", path_str, e);
                
                let reply = warp::reply::with_status(
                    warp::reply::with_header(
                        warp::reply::with_header(
                            "Asset not found",
                            "Access-Control-Allow-Origin", "http://localhost:3030"
                        ),
                        "Content-Type", "text/plain"
                    ),
                    warp::http::StatusCode::NOT_FOUND
                );
                
                Ok(Box::new(reply))
            }
        }
    }

    pub async fn fetch_with_method(url: &str, method: &str, form_data: &str) -> Result<ProxyResponse, Box<dyn std::error::Error + Send + Sync>> {
        println!("🌐 Fetching URL with method {} and universal strategies: {}", method, url);

        // Versuche zuerst eine minimale Verbindung
        match Self::try_minimal_connection_with_method(url, method, form_data).await {
            Ok(response) => {
                println!("✅ Minimal connection successful for: {}", url);
                return Self::process_response(response, url).await;
            }
            Err(e) => {
                println!("⚠️ Minimal connection failed for {}: {}", url, e);
            }
        }

        // Versuche verschiedene Verbindungsstrategien
        let strategies = Self::get_universal_strategies();
        
        for strategy in strategies {
            println!("🔄 Trying strategy: {}", strategy.description);
            
            match Self::try_connection_with_strategy_and_method(url, &strategy, method, form_data).await {
                Ok(response) => {
                    println!("✅ Strategy '{}' successful for: {}", strategy.description, url);
                    return Self::process_response(response, url).await;
                }
                Err(e) => {
                    println!("❌ Strategy '{}' failed for {}: {}", strategy.description, url, e);
                }
            }
            
            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }

        // Versuche universelle Fallback-Strategien
        if let Some(response) = Self::try_universal_fallback_with_method(url, method, form_data).await {
            println!("✅ Universal fallback successful for: {}", url);
            return Self::process_response(response, url).await;
        }

        // Versuche alternative URLs
        let alternative_urls = Self::generate_alternative_urls(url);
        for alt_url in alternative_urls {
            println!("🔄 Trying alternative URL: {}", alt_url);
            
            match Self::try_minimal_connection_with_method(&alt_url, method, form_data).await {
                Ok(response) => {
                    println!("✅ Alternative URL successful: {}", alt_url);
                    return Self::process_response(response, url).await;
                }
                Err(e) => {
                    println!("❌ Alternative URL failed {}: {}", alt_url, e);
                }
            }
            
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }

        Err(format!("All connection strategies failed for URL: {}", url).into())
    }

    async fn try_minimal_connection_with_method(url: &str, method: &str, form_data: &str) -> Result<reqwest::Response, Box<dyn std::error::Error + Send + Sync>> {
        println!("🌐 Using universal aggressive headers for: {}", url);
        
        // 🚀 ULTRA-AGGRESSIVE CLIENT CONFIGURATION
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(45))  // Erhöht von 30 auf 45 Sekunden
            .connect_timeout(std::time::Duration::from_secs(20))  // Erhöht von 15 auf 20 Sekunden
            .redirect(reqwest::redirect::Policy::limited(15))  // Erhöht von 10 auf 15 Redirects
            .danger_accept_invalid_certs(true)
            .use_rustls_tls()
            .http2_prior_knowledge()
            .http2_adaptive_window(true)
            .http2_max_frame_size(Some(65536))  // Erhöht von 32768 auf 65536
            .tcp_keepalive(std::time::Duration::from_secs(120))  // Erhöht von 60 auf 120 Sekunden
            .pool_idle_timeout(std::time::Duration::from_secs(120))  // Erhöht von 90 auf 120 Sekunden
            .pool_max_idle_per_host(10)  // Erhöht von 8 auf 10
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0")
            .gzip(true)
            .brotli(true)
            .deflate(true)
            .build()?;
        
        // 🌍 ERWEITERTE UNIVERSELLE HEADERS
        let mut request_builder = match method.to_uppercase().as_str() {
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            "HEAD" => client.head(url),
            "PATCH" => client.patch(url),
            _ => client.get(url),
        };
        
        // 🛡️ UNIVERSELLE BROWSER-HEADERS
        request_builder = request_builder
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
            .header("Accept-Language", "de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7")
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("Cache-Control", "no-cache")
            .header("Pragma", "no-cache")
            .header("Sec-Fetch-Dest", "document")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-Site", "none")
            .header("Sec-Fetch-User", "?1")
            .header("Upgrade-Insecure-Requests", "1")
            .header("DNT", "1")
            .header("Connection", "keep-alive")
            .header("Sec-CH-UA", "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Microsoft Edge\";v=\"120\"")
            .header("Sec-CH-UA-Mobile", "?0")
            .header("Sec-CH-UA-Platform", "\"Windows\"");
        
        // 📝 FORM DATA HANDLING
        if !form_data.is_empty() && (method.to_uppercase() == "POST" || method.to_uppercase() == "PUT" || method.to_uppercase() == "PATCH") {
            request_builder = request_builder
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(form_data.to_string());
        }
        
        println!("🚀 Sending {} request with enhanced headers...", method.to_uppercase());
        let response = request_builder.send().await?;
        
        println!("✅ Response received: {} {}", response.status(), response.status().canonical_reason().unwrap_or("Unknown"));
        Ok(response)
    }

    async fn try_connection_with_strategy_and_method(url: &str, strategy: &ConnectionStrategy, method: &str, form_data: &str) -> Result<reqwest::Response, Box<dyn std::error::Error + Send + Sync>> {
        let mut client_builder = reqwest::Client::builder()
            .user_agent(&strategy.user_agent)
            .timeout(std::time::Duration::from_secs(strategy.timeout))
            .connect_timeout(std::time::Duration::from_secs(strategy.connect_timeout))
            .redirect(reqwest::redirect::Policy::limited(strategy.redirects))
            .danger_accept_invalid_certs(strategy.accept_invalid_certs);

        if strategy.use_http2 {
            client_builder = client_builder.use_rustls_tls();
        }

        let client = client_builder.build()?;
        
        let mut request = match method.to_uppercase().as_str() {
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            _ => client.get(url),
        };

        for (key, value) in &strategy.headers {
            request = request.header(*key, *value);
        }

        // Füge Form-Daten hinzu wenn vorhanden
        if !form_data.is_empty() && (method.to_uppercase() == "POST" || method.to_uppercase() == "PUT") {
            if form_data.starts_with('{') {
                request = request
                    .header("Content-Type", "application/json")
                    .body(form_data.to_string());
            } else {
                request = request
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(form_data.to_string());
            }
        }

        let response = request.send().await?;
        Ok(response)
    }

    async fn try_universal_fallback_with_method(url: &str, method: &str, form_data: &str) -> Option<reqwest::Response> {
        println!("🔄 Trying universal fallback strategies for: {}", url);
        
        // 🚀 ADAPTIVE FALLBACK-STRATEGIEN
        let fallback_strategies = vec![
            // Ultra-Fast Strategy
            ("Ultra-Fast", 10, 5, false, false),
            // Secure Strategy  
            ("Secure", 20, 10, true, true),
            // Mobile Strategy
            ("Mobile", 15, 8, false, false),
            // Legacy Strategy
            ("Legacy", 25, 12, true, false),
            // Aggressive Strategy
            ("Aggressive", 35, 18, true, true),
        ];
        
        for (strategy_name, timeout, connect_timeout, accept_invalid_certs, use_http2) in fallback_strategies {
            println!("🔧 Trying {} strategy for: {}", strategy_name, url);
            
            // Erstelle adaptiven Client
            let mut client_builder = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(timeout))
                .connect_timeout(std::time::Duration::from_secs(connect_timeout))
                .redirect(reqwest::redirect::Policy::limited(10))
                .danger_accept_invalid_certs(accept_invalid_certs)
                .tcp_keepalive(std::time::Duration::from_secs(60))
                .pool_idle_timeout(std::time::Duration::from_secs(90))
                .pool_max_idle_per_host(8);
            
            if use_http2 {
                client_builder = client_builder.http2_prior_knowledge();
            }
            
            // Strategie-spezifische User-Agents
            let user_agent = match strategy_name {
                "Ultra-Fast" => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
                "Secure" => "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
                "Mobile" => "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
                "Legacy" => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36",
                "Aggressive" => "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
                _ => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            };
            
            client_builder = client_builder.user_agent(user_agent);
            
            if let Ok(client) = client_builder.build() {
                // Probiere verschiedene URL-Varianten
                let url_alternatives = Self::generate_universal_url_alternatives(url);
                
                for (index, alt_url) in url_alternatives.iter().enumerate() {
                    if index >= 5 { break; } // Maximal 5 Alternativen pro Strategie
                    
                    println!("🌐 {} strategy trying URL {}: {}", strategy_name, index + 1, alt_url);
                    
                    let mut request_builder = match method.to_uppercase().as_str() {
                        "POST" => client.post(alt_url),
                        "PUT" => client.put(alt_url),
                        "DELETE" => client.delete(alt_url),
                        "HEAD" => client.head(alt_url),
                        "PATCH" => client.patch(alt_url),
                        _ => client.get(alt_url),
                    };
                    
                    // Strategie-spezifische Headers
                    request_builder = match strategy_name {
                        "Ultra-Fast" => request_builder
                            .header("Accept", "text/html,*/*")
                            .header("Connection", "close"),
                        "Secure" => request_builder
                            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
                            .header("Accept-Language", "de-DE,de;q=0.9,en;q=0.8")
                            .header("DNT", "1")
                            .header("Sec-Fetch-Dest", "document")
                            .header("Sec-Fetch-Mode", "navigate")
                            .header("Sec-Fetch-Site", "none"),
                        "Mobile" => request_builder
                            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
                            .header("Accept-Language", "de-DE,de;q=0.8,en-US;q=0.5,en;q=0.3")
                            .header("Accept-Encoding", "gzip, deflate")
                            .header("Connection", "keep-alive"),
                        "Legacy" => request_builder
                            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
                            .header("Accept-Language", "de,en;q=0.8")
                            .header("Cache-Control", "max-age=0"),
                        "Aggressive" => request_builder
                            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
                            .header("Accept-Language", "de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7")
                            .header("Accept-Encoding", "gzip, deflate, br")
                            .header("Cache-Control", "no-cache")
                            .header("Pragma", "no-cache")
                            .header("X-Forwarded-For", "127.0.0.1")
                            .header("X-Real-IP", "127.0.0.1")
                            .header("Origin", "https://google.de")
                            .header("Referer", "https://google.de/"),
                        _ => request_builder,
                    };
                    
                    // Form-Data hinzufügen
                    if !form_data.is_empty() && (method.to_uppercase() == "POST" || method.to_uppercase() == "PUT") {
                        if form_data.starts_with('{') || form_data.starts_with('[') {
                            request_builder = request_builder
                                .header("Content-Type", "application/json")
                                .body(form_data.to_string());
                        } else {
                            request_builder = request_builder
                                .header("Content-Type", "application/x-www-form-urlencoded")
                                .body(form_data.to_string());
                        }
                    }
                    
                    // Versuche Request
                    match request_builder.send().await {
                        Ok(response) => {
                            if response.status().is_success() {
                                println!("✅ {} strategy successful with URL: {}", strategy_name, alt_url);
                                return Some(response);
                            } else {
                                println!("⚠️ {} strategy got status {} for: {}", strategy_name, response.status(), alt_url);
                            }
                        }
                        Err(e) => {
                            println!("❌ {} strategy failed for {}: {}", strategy_name, alt_url, e);
                        }
                    }
                    
                    // Kurze Pause zwischen Versuchen
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
            }
            
            // Pause zwischen Strategien
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
        
        println!("❌ All universal fallback strategies failed for: {}", url);
        None
    }

    async fn process_response(response: reqwest::Response, url: &str) -> Result<ProxyResponse, Box<dyn std::error::Error + Send + Sync>> {
        let status_code = response.status().as_u16();
        
        // 🛡️ ENTFERNE CSP-RELEVANTE HTTP-HEADER
        let headers = response.headers();
        let mut cleaned_headers = std::collections::HashMap::new();
        
        // Sammle alle Header außer CSP-relevanten
        for (name, value) in headers.iter() {
            let header_name = name.as_str().to_lowercase();
            
            // Überspringe CSP-relevante Header
            if !Self::is_csp_related_header(&header_name) {
                if let Ok(value_str) = value.to_str() {
                    cleaned_headers.insert(header_name, value_str.to_string());
                }
            } else {
                println!("🗑️ Removed CSP header '{}' from response for: {}", header_name, url);
            }
        }
        
        let content_type = cleaned_headers.get("content-type")
            .unwrap_or(&"text/html".to_string())
            .clone();

        let body = response.text().await?;
        
        // Aggressive Frame-Blocking-Umgehung - Text ersetzen
        let mut processed_content = body;
        let frame_blocking_messages = [
            "For security reasons, framing is not allowed",
            "This page cannot be displayed in a frame",
            "X-Frame-Options deny",
            "refused to display",
            "in a frame because it set",
            "Refused to display",
            "frame-ancestors",
            "This content cannot be embedded",
            "Embedding disabled",
            "Frame blocked",
            "Refused to connect",
            "Connection refused",
            "haben die Verbindung verweigert",
            "Content Security Policy",
            "CSP violation",
            "blocked by CSP",
            "CSP directive",
            "unsafe-inline",
            "unsafe-eval"
        ];
        
        for message in &frame_blocking_messages {
            if processed_content.contains(message) {
                processed_content = processed_content.replace(message, "Security restriction bypassed by Ora Browser");
                println!("🔓 Security message '{}' replaced for: {}", message, url);
            }
        }
        
        // Entferne iframe-Blocks und füge erweiterte CORS/Security-Fixes hinzu
        processed_content = Self::strip_all_iframe_blocks(&processed_content, url);
        processed_content = Self::inject_cors_and_security_fixes(&processed_content, url);

        Ok(ProxyResponse {
            content: processed_content,
            content_type,
            status_code,
        })
    }

    // 🛡️ NEUE HILFSFUNKTION: Prüft ob ein Header CSP-relevant ist
    fn is_csp_related_header(header_name: &str) -> bool {
        let csp_headers = [
            "content-security-policy",
            "content-security-policy-report-only",
            "x-frame-options",
            "x-content-type-options",
            "referrer-policy",
            "permissions-policy",
            "feature-policy",
            "expect-ct",
            "strict-transport-security",
            "x-xss-protection",
            "cross-origin-embedder-policy",
            "cross-origin-opener-policy",
            "cross-origin-resource-policy",
            "access-control-allow-origin",
            "access-control-allow-methods",
            "access-control-allow-headers",
            "access-control-allow-credentials",
            "access-control-expose-headers",
            "access-control-max-age",
            "timing-allow-origin",
            "x-permitted-cross-domain-policies",
            "x-download-options",
            "public-key-pins",
            "public-key-pins-report-only"
        ];
        
        csp_headers.iter().any(|&csp_header| header_name.contains(csp_header))
    }

    // Kompatibilitätsfunktion für bestehenden Code
    pub async fn fetch_and_strip_headers(url: &str) -> Result<ProxyResponse, Box<dyn std::error::Error + Send + Sync>> {
        Self::fetch_with_method(url, "GET", "").await
    }

    fn get_universal_strategies() -> Vec<ConnectionStrategy> {
        vec![
            // 🚀 ULTRA-AGGRESSIVE GOOGLE STRATEGY
            ConnectionStrategy {
                description: "Ultra-Aggressive Google Strategy".to_string(),
                user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                timeout: 30,
                connect_timeout: 15,
                redirects: 10,
                accept_invalid_certs: true,
                use_http2: true,
                headers: vec![
                    ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"),
                    ("Accept-Language", "de-DE,de;q=0.9,en;q=0.8"),
                    ("Accept-Encoding", "gzip, deflate, br"),
                    ("Cache-Control", "no-cache"),
                    ("Pragma", "no-cache"),
                    ("Sec-Fetch-Dest", "document"),
                    ("Sec-Fetch-Mode", "navigate"),
                    ("Sec-Fetch-Site", "none"),
                    ("Sec-Fetch-User", "?1"),
                    ("Upgrade-Insecure-Requests", "1"),
                    ("DNT", "1"),
                    ("Connection", "keep-alive"),
                    ("X-Forwarded-For", "127.0.0.1"),
                    ("X-Real-IP", "127.0.0.1"),
                    ("X-Requested-With", "XMLHttpRequest"),
                ],
            },
            // 🌐 UNIVERSAL BROWSER STRATEGY
            ConnectionStrategy {
                description: "Universal Browser Strategy".to_string(),
                user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0".to_string(),
                timeout: 25,
                connect_timeout: 12,
                redirects: 8,
                accept_invalid_certs: true,
                use_http2: true,
                headers: vec![
                    ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
                    ("Accept-Language", "de,en-US;q=0.7,en;q=0.3"),
                    ("Accept-Encoding", "gzip, deflate, br"),
                    ("Cache-Control", "max-age=0"),
                    ("Sec-Fetch-Dest", "document"),
                    ("Sec-Fetch-Mode", "navigate"),
                    ("Sec-Fetch-Site", "cross-site"),
                    ("Upgrade-Insecure-Requests", "1"),
                    ("Connection", "keep-alive"),
                ],
            },
            // 🔒 SECURE MOBILE STRATEGY
            ConnectionStrategy {
                description: "Secure Mobile Strategy".to_string(),
                user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1".to_string(),
                timeout: 20,
                connect_timeout: 10,
                redirects: 5,
                accept_invalid_certs: true,
                use_http2: false,
                headers: vec![
                    ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
                    ("Accept-Language", "de-DE,de;q=0.8,en-US;q=0.5,en;q=0.3"),
                    ("Accept-Encoding", "gzip, deflate"),
                    ("Cache-Control", "no-cache"),
                    ("Connection", "keep-alive"),
                    ("Upgrade-Insecure-Requests", "1"),
                ],
            },
            // ⚡ FAST MINIMAL STRATEGY
            ConnectionStrategy {
                description: "Fast Minimal Strategy".to_string(),
                user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                timeout: 15,
                connect_timeout: 8,
                redirects: 3,
                accept_invalid_certs: true,
                use_http2: true,
                headers: vec![
                    ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
                    ("Accept-Language", "de,en;q=0.5"),
                    ("Cache-Control", "no-cache"),
                    ("Connection", "close"),
                ],
            },
            // 🛡️ STEALTH STRATEGY
            ConnectionStrategy {
                description: "Stealth Strategy".to_string(),
                user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15".to_string(),
                timeout: 35,
                connect_timeout: 18,
                redirects: 12,
                accept_invalid_certs: true,
                use_http2: true,
                headers: vec![
                    ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"),
                    ("Accept-Language", "de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7"),
                    ("Accept-Encoding", "gzip, deflate, br"),
                    ("Cache-Control", "max-age=0"),
                    ("Sec-Fetch-Dest", "document"),
                    ("Sec-Fetch-Mode", "navigate"),
                    ("Sec-Fetch-Site", "none"),
                    ("Sec-Fetch-User", "?1"),
                    ("Upgrade-Insecure-Requests", "1"),
                    ("DNT", "1"),
                    ("Connection", "keep-alive"),
                    ("X-Forwarded-Proto", "https"),
                    ("X-Forwarded-Host", "localhost"),
                ],
            },
        ]
    }

    fn strip_all_iframe_blocks(html: &str, _url: &str) -> String {
        let mut result = html.to_string();
        
        // Entferne iframe-Tags
        if let Ok(re) = regex::Regex::new(r"(?i)<iframe[^>]*>.*?</iframe>") {
            result = re.replace_all(&result, "<!-- iframe removed for security -->").to_string();
        }
        
        // Entferne embed-Tags
        if let Ok(re) = regex::Regex::new(r"(?i)<embed[^>]*>") {
            result = re.replace_all(&result, "<!-- embed removed for security -->").to_string();
        }
        
        // Entferne object-Tags
        if let Ok(re) = regex::Regex::new(r"(?i)<object[^>]*>.*?</object>") {
            result = re.replace_all(&result, "<!-- object removed for security -->").to_string();
        }

        result
    }

    fn generate_alternative_urls(url: &str) -> Vec<String> {
        let mut alternatives = Vec::new();
        alternatives.push(url.to_string());
        
        // 🌍 UNIVERSELLE INTERNATIONALE DOMAIN-VARIANTEN
        let domain_mappings = vec![
            // Google Varianten
            ("google.com", vec!["google.de", "google.co.uk", "google.fr", "google.es", "google.it", "google.ca", "google.com.au"]),
            ("google.de", vec!["google.com", "google.co.uk", "google.fr"]),
            
            // Social Media Varianten
            ("facebook.com", vec!["m.facebook.com", "mobile.facebook.com", "touch.facebook.com"]),
            ("twitter.com", vec!["mobile.twitter.com", "m.twitter.com", "x.com"]),
            ("instagram.com", vec!["m.instagram.com", "www.instagram.com"]),
            
            // E-Commerce Varianten
            ("amazon.com", vec!["amazon.de", "amazon.co.uk", "amazon.fr", "amazon.es", "amazon.it"]),
            ("amazon.de", vec!["amazon.com", "amazon.co.uk", "amazon.fr"]),
            ("ebay.com", vec!["ebay.de", "ebay.co.uk", "ebay.fr", "ebay.es", "ebay.it"]),
            
            // News & Media Varianten
            ("youtube.com", vec!["m.youtube.com", "mobile.youtube.com", "youtu.be"]),
            ("wikipedia.org", vec!["de.wikipedia.org", "en.wikipedia.org", "fr.wikipedia.org", "es.wikipedia.org"]),
            ("reddit.com", vec!["old.reddit.com", "m.reddit.com", "mobile.reddit.com"]),
            
            // Tech Varianten
            ("github.com", vec!["mobile.github.com", "m.github.com"]),
            ("stackoverflow.com", vec!["m.stackoverflow.com", "mobile.stackoverflow.com"]),
            
            // Banking & Finance
            ("paypal.com", vec!["paypal.de", "paypal.co.uk", "paypal.fr"]),
            
            // Streaming
            ("netflix.com", vec!["netflix.de", "netflix.co.uk", "netflix.fr"]),
            ("spotify.com", vec!["open.spotify.com", "play.spotify.com"]),
        ];
        
        // Anwenden der Domain-Mappings
        for (base_domain, variants) in domain_mappings {
            if url.contains(base_domain) {
                for variant in variants {
                    let variant_url = url.replace(base_domain, variant);
                    if !alternatives.contains(&variant_url) {
                        alternatives.push(variant_url);
                    }
                }
            }
        }
        
        // 🔗 PROTOKOLL-VARIANTEN (HTTP/HTTPS)
        if url.starts_with("https://") {
            let http_variant = url.replace("https://", "http://");
            if !alternatives.contains(&http_variant) {
                alternatives.push(http_variant);
            }
        } else if url.starts_with("http://") {
            let https_variant = url.replace("http://", "https://");
            if !alternatives.contains(&https_variant) {
                alternatives.push(https_variant);
            }
        }
        
        // 🌐 WWW-VARIANTEN
        if url.contains("://www.") {
            let no_www = url.replace("://www.", "://");
            if !alternatives.contains(&no_www) {
                alternatives.push(no_www);
            }
        } else if url.contains("://") && !url.contains("://www.") {
            let parts: Vec<&str> = url.splitn(2, "://").collect();
            if parts.len() == 2 {
                let with_www = format!("{}://www.{}", parts[0], parts[1]);
                if !alternatives.contains(&with_www) {
                    alternatives.push(with_www);
                }
            }
        }
        
        // 📱 MOBILE-VARIANTEN
        let mobile_prefixes = vec!["m.", "mobile.", "touch.", "wap."];
        for prefix in mobile_prefixes {
            if !url.contains(&format!("://{}", prefix)) {
                let mobile_variant = url.replace("://", &format!("://{}", prefix));
                if !alternatives.contains(&mobile_variant) {
                    alternatives.push(mobile_variant);
                }
            }
        }
        
        // 🔄 SUBDOMAIN-VARIANTEN
        let subdomains = vec!["www2.", "www3.", "secure.", "ssl.", "api.", "cdn.", "static."];
        for subdomain in subdomains {
            if !url.contains(&format!("://{}", subdomain)) {
                let subdomain_variant = url.replace("://", &format!("://{}", subdomain));
                if !alternatives.contains(&subdomain_variant) {
                    alternatives.push(subdomain_variant);
                }
            }
        }
        
        // 🌍 REGIONALE TLD-VARIANTEN
        let tld_mappings = vec![
            (".com", vec![".de", ".co.uk", ".fr", ".es", ".it", ".ca", ".com.au", ".co.jp"]),
            (".de", vec![".com", ".co.uk", ".fr", ".es", ".it"]),
            (".org", vec![".net", ".com", ".info"]),
            (".net", vec![".com", ".org", ".info"]),
        ];
        
        for (base_tld, variant_tlds) in tld_mappings {
            if url.contains(base_tld) {
                for variant_tld in variant_tlds {
                    let tld_variant = url.replace(base_tld, variant_tld);
                    if !alternatives.contains(&tld_variant) {
                        alternatives.push(tld_variant);
                    }
                }
            }
        }
        
        // 🔀 PATH-VARIANTEN
        if url.contains("?") {
            // Entferne Query-Parameter
            let base_url = url.split('?').next().unwrap_or(url);
            if !alternatives.contains(&base_url.to_string()) {
                alternatives.push(base_url.to_string());
            }
        }
        
        if url.ends_with("/") {
            // Entferne trailing slash
            let no_slash = url.trim_end_matches('/');
            if !alternatives.contains(&no_slash.to_string()) {
                alternatives.push(no_slash.to_string());
            }
        } else {
            // Füge trailing slash hinzu
            let with_slash = format!("{}/", url);
            if !alternatives.contains(&with_slash) {
                alternatives.push(with_slash);
            }
        }
        
        // 🔧 SPEZIELLE WEBSITE-FIXES
        
        // YouTube Shorts zu regulären Videos
        if url.contains("youtube.com/shorts/") {
            let regular_video = url.replace("/shorts/", "/watch?v=");
            if !alternatives.contains(&regular_video) {
                alternatives.push(regular_video);
            }
        }
        
        // Reddit Old/New Interface
        if url.contains("reddit.com") && !url.contains("old.reddit.com") {
            let old_reddit = url.replace("reddit.com", "old.reddit.com");
            if !alternatives.contains(&old_reddit) {
                alternatives.push(old_reddit);
            }
        }
        
        // Twitter/X Varianten
        if url.contains("twitter.com") {
            let x_variant = url.replace("twitter.com", "x.com");
            if !alternatives.contains(&x_variant) {
                alternatives.push(x_variant);
            }
        } else if url.contains("x.com") {
            let twitter_variant = url.replace("x.com", "twitter.com");
            if !alternatives.contains(&twitter_variant) {
                alternatives.push(twitter_variant);
            }
        }
        
        // Begrenze auf maximal 20 Alternativen für Performance
        alternatives.truncate(20);
        
        println!("🔄 Generated {} URL alternatives for: {}", alternatives.len(), url);
        alternatives
    }

    fn generate_universal_url_alternatives(url: &str) -> Vec<String> {
        let mut alternatives = Vec::new();
        alternatives.push(url.to_string());
        
        // 🌍 UNIVERSELLE INTERNATIONALE DOMAIN-VARIANTEN
        let domain_mappings = vec![
            // Google Varianten
            ("google.com", vec!["google.de", "google.co.uk", "google.fr", "google.es", "google.it", "google.ca", "google.com.au"]),
            ("google.de", vec!["google.com", "google.co.uk", "google.fr"]),
            
            // Social Media Varianten
            ("facebook.com", vec!["m.facebook.com", "mobile.facebook.com", "touch.facebook.com"]),
            ("twitter.com", vec!["mobile.twitter.com", "m.twitter.com", "x.com"]),
            ("instagram.com", vec!["m.instagram.com", "www.instagram.com"]),
            
            // E-Commerce Varianten
            ("amazon.com", vec!["amazon.de", "amazon.co.uk", "amazon.fr", "amazon.es", "amazon.it"]),
            ("amazon.de", vec!["amazon.com", "amazon.co.uk", "amazon.fr"]),
            ("ebay.com", vec!["ebay.de", "ebay.co.uk", "ebay.fr", "ebay.es", "ebay.it"]),
            
            // News & Media Varianten
            ("youtube.com", vec!["m.youtube.com", "mobile.youtube.com", "youtu.be"]),
            ("wikipedia.org", vec!["de.wikipedia.org", "en.wikipedia.org", "fr.wikipedia.org", "es.wikipedia.org"]),
            ("reddit.com", vec!["old.reddit.com", "m.reddit.com", "mobile.reddit.com"]),
            
            // Tech Varianten
            ("github.com", vec!["mobile.github.com", "m.github.com"]),
            ("stackoverflow.com", vec!["m.stackoverflow.com", "mobile.stackoverflow.com"]),
            
            // Banking & Finance
            ("paypal.com", vec!["paypal.de", "paypal.co.uk", "paypal.fr"]),
            
            // Streaming
            ("netflix.com", vec!["netflix.de", "netflix.co.uk", "netflix.fr"]),
            ("spotify.com", vec!["open.spotify.com", "play.spotify.com"]),
        ];
        
        // Anwenden der Domain-Mappings
        for (base_domain, variants) in domain_mappings {
            if url.contains(base_domain) {
                for variant in variants {
                    let variant_url = url.replace(base_domain, variant);
                    if !alternatives.contains(&variant_url) {
                        alternatives.push(variant_url);
                    }
                }
            }
        }
        
        // 🔗 PROTOKOLL-VARIANTEN (HTTP/HTTPS)
        if url.starts_with("https://") {
            let http_variant = url.replace("https://", "http://");
            if !alternatives.contains(&http_variant) {
                alternatives.push(http_variant);
            }
        } else if url.starts_with("http://") {
            let https_variant = url.replace("http://", "https://");
            if !alternatives.contains(&https_variant) {
                alternatives.push(https_variant);
            }
        }
        
        // 🌐 WWW-VARIANTEN
        if url.contains("://www.") {
            let no_www = url.replace("://www.", "://");
            if !alternatives.contains(&no_www) {
                alternatives.push(no_www);
            }
        } else if url.contains("://") && !url.contains("://www.") {
            let parts: Vec<&str> = url.splitn(2, "://").collect();
            if parts.len() == 2 {
                let with_www = format!("{}://www.{}", parts[0], parts[1]);
                if !alternatives.contains(&with_www) {
                    alternatives.push(with_www);
                }
            }
        }
        
        // 📱 MOBILE-VARIANTEN
        let mobile_prefixes = vec!["m.", "mobile.", "touch.", "wap."];
        for prefix in mobile_prefixes {
            if !url.contains(&format!("://{}", prefix)) {
                let mobile_variant = url.replace("://", &format!("://{}", prefix));
                if !alternatives.contains(&mobile_variant) {
                    alternatives.push(mobile_variant);
                }
            }
        }
        
        // 🔄 SUBDOMAIN-VARIANTEN
        let subdomains = vec!["www2.", "www3.", "secure.", "ssl.", "api.", "cdn.", "static."];
        for subdomain in subdomains {
            if !url.contains(&format!("://{}", subdomain)) {
                let subdomain_variant = url.replace("://", &format!("://{}", subdomain));
                if !alternatives.contains(&subdomain_variant) {
                    alternatives.push(subdomain_variant);
                }
            }
        }
        
        // 🌍 REGIONALE TLD-VARIANTEN
        let tld_mappings = vec![
            (".com", vec![".de", ".co.uk", ".fr", ".es", ".it", ".ca", ".com.au", ".co.jp"]),
            (".de", vec![".com", ".co.uk", ".fr", ".es", ".it"]),
            (".org", vec![".net", ".com", ".info"]),
            (".net", vec![".com", ".org", ".info"]),
        ];
        
        for (base_tld, variant_tlds) in tld_mappings {
            if url.contains(base_tld) {
                for variant_tld in variant_tlds {
                    let tld_variant = url.replace(base_tld, variant_tld);
                    if !alternatives.contains(&tld_variant) {
                        alternatives.push(tld_variant);
                    }
                }
            }
        }
        
        // 🔀 PATH-VARIANTEN
        if url.contains("?") {
            // Entferne Query-Parameter
            let base_url = url.split('?').next().unwrap_or(url);
            if !alternatives.contains(&base_url.to_string()) {
                alternatives.push(base_url.to_string());
            }
        }
        
        if url.ends_with("/") {
            // Entferne trailing slash
            let no_slash = url.trim_end_matches('/');
            if !alternatives.contains(&no_slash.to_string()) {
                alternatives.push(no_slash.to_string());
            }
        } else {
            // Füge trailing slash hinzu
            let with_slash = format!("{}/", url);
            if !alternatives.contains(&with_slash) {
                alternatives.push(with_slash);
            }
        }
        
        // 🔧 SPEZIELLE WEBSITE-FIXES
        
        // YouTube Shorts zu regulären Videos
        if url.contains("youtube.com/shorts/") {
            let regular_video = url.replace("/shorts/", "/watch?v=");
            if !alternatives.contains(&regular_video) {
                alternatives.push(regular_video);
            }
        }
        
        // Reddit Old/New Interface
        if url.contains("reddit.com") && !url.contains("old.reddit.com") {
            let old_reddit = url.replace("reddit.com", "old.reddit.com");
            if !alternatives.contains(&old_reddit) {
                alternatives.push(old_reddit);
            }
        }
        
        // Twitter/X Varianten
        if url.contains("twitter.com") {
            let x_variant = url.replace("twitter.com", "x.com");
            if !alternatives.contains(&x_variant) {
                alternatives.push(x_variant);
            }
        } else if url.contains("x.com") {
            let twitter_variant = url.replace("x.com", "twitter.com");
            if !alternatives.contains(&twitter_variant) {
                alternatives.push(twitter_variant);
            }
        }
        
        // Begrenze auf maximal 20 Alternativen für Performance
        alternatives.truncate(20);
        
        println!("🔄 Generated {} URL alternatives for: {}", alternatives.len(), url);
        alternatives
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    fn create_error_page(url: &str, error: &str) -> String {
        let domain = url.split('/').nth(2).unwrap_or("unknown");
        let is_frame_error = error.contains("framing") || error.contains("X-Frame-Options") || error.contains("frame-ancestors");
        let is_connection_error = error.contains("connection") || error.contains("refused") || error.contains("timeout");
        let is_russian_site = url.contains(".ru") || url.contains("dzen") || url.contains("yandex");
        
        let specific_solutions = if is_frame_error {
            r#"
            <div class="solution-box frame-error">
                <h3>🔒 Frame-Blocking erkannt</h3>
                <p>Diese Website blockiert die Anzeige in Frames aus Sicherheitsgründen.</p>
                <div class="solutions">
                    <button onclick="retryWithBypass()" class="retry-btn">🔧 Mit Frame-Bypass versuchen</button>
                    <button onclick="openDirect()" class="direct-btn">🌐 Direkt öffnen</button>
                </div>
            </div>
            "#
        } else if is_connection_error && is_russian_site {
            r#"
            <div class="solution-box russian-error">
                <h3>🇷🇺 Russische Website-Probleme</h3>
                <p>Verbindungsprobleme mit russischen Websites können verschiedene Ursachen haben.</p>
                <div class="solutions">
                    <button onclick="retryWithRussianHeaders()" class="retry-btn">🔄 Mit russischen Headern versuchen</button>
                    <button onclick="retryWithVPN()" class="vpn-btn">🛡️ Mit Anti-Bot-Modus versuchen</button>
                </div>
            </div>
            "#
        } else if is_connection_error {
            r#"
            <div class="solution-box connection-error">
                <h3>🌐 Verbindungsproblem</h3>
                <p>Die Website ist möglicherweise nicht erreichbar oder blockiert Anfragen.</p>
                <div class="solutions">
                    <button onclick="retryWithDifferentUA()" class="retry-btn">🔄 Mit anderem Browser versuchen</button>
                    <button onclick="retryWithAntiBot()" class="antibot-btn">🤖 Anti-Bot-Modus versuchen</button>
                </div>
            </div>
            "#
        } else {
            r#"
            <div class="solution-box general-error">
                <h3>⚠️ Allgemeiner Fehler</h3>
                <p>Ein unbekannter Fehler ist aufgetreten.</p>
                <div class="solutions">
                    <button onclick="retryGeneral()" class="retry-btn">🔄 Erneut versuchen</button>
                </div>
            </div>
            "#
        };

        format!(r#"
        <!DOCTYPE html>
        <html lang="de">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Ora Browser - Verbindungsfehler</title>
            <style>
                body {{
                    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    margin: 0;
                    padding: 20px;
                    min-height: 100vh;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                }}
                .error-container {{
                    background: white;
                    border-radius: 12px;
                    padding: 40px;
                    max-width: 600px;
                    width: 100%;
                    box-shadow: 0 20px 40px rgba(0,0,0,0.1);
                    text-align: center;
                }}
                .error-icon {{
                    font-size: 64px;
                    margin-bottom: 20px;
                }}
                h1 {{
                    color: #333;
                    margin-bottom: 10px;
                    font-size: 28px;
                }}
                .url-info {{
                    background: #f8f9fa;
                    padding: 15px;
                    border-radius: 8px;
                    margin: 20px 0;
                    font-family: monospace;
                    word-break: break-all;
                }}
                .error-details {{
                    background: #fff3cd;
                    border: 1px solid #ffeaa7;
                    border-radius: 8px;
                    padding: 15px;
                    margin: 20px 0;
                    color: #856404;
                }}
                .solution-box {{
                    background: #e3f2fd;
                    border: 1px solid #2196f3;
                    border-radius: 8px;
                    padding: 20px;
                    margin: 20px 0;
                    text-align: left;
                }}
                .solution-box.frame-error {{
                    background: #fff3e0;
                    border-color: #ff9800;
                }}
                .solution-box.russian-error {{
                    background: #f3e5f5;
                    border-color: #9c27b0;
                }}
                .solution-box.connection-error {{
                    background: #ffebee;
                    border-color: #f44336;
                }}
                .solutions {{
                    display: flex;
                    gap: 10px;
                    margin-top: 15px;
                    flex-wrap: wrap;
                }}
                .retry-btn, .direct-btn, .vpn-btn, .antibot-btn {{
                    background: #2196f3;
                    color: white;
                    border: none;
                    padding: 10px 20px;
                    border-radius: 6px;
                    cursor: pointer;
                    font-size: 14px;
                    transition: background 0.3s;
                }}
                .direct-btn {{
                    background: #4caf50;
                }}
                .vpn-btn {{
                    background: #9c27b0;
                }}
                .antibot-btn {{
                    background: #ff5722;
                }}
                .retry-btn:hover, .direct-btn:hover, .vpn-btn:hover, .antibot-btn:hover {{
                    opacity: 0.8;
                }}
                .back-btn {{
                    background: #6c757d;
                    color: white;
                    border: none;
                    padding: 12px 24px;
                    border-radius: 6px;
                    cursor: pointer;
                    font-size: 16px;
                    margin-top: 20px;
                }}
            </style>
        </head>
        <body>
            <div class="error-container">
                <div class="error-icon">🚫</div>
                <h1>Verbindung fehlgeschlagen</h1>
                <p>Die Website <strong>{}</strong> konnte nicht geladen werden.</p>
                
                <div class="url-info">
                    <strong>URL:</strong> {}
                </div>
                
                <div class="error-details">
                    <strong>Fehlerdetails:</strong><br>
                    {}
                </div>
                
                {}
                
                <div style="margin-top: 30px;">
                    <button onclick="history.back()" class="back-btn">← Zurück</button>
                </div>
            </div>
            
            <script>
                function retryWithBypass() {{
                    const url = new URL(window.location);
                    url.searchParams.set('bypass', 'frame');
                    window.location.href = url.toString();
                }}
                
                function retryWithRussianHeaders() {{
                    const url = new URL(window.location);
                    url.searchParams.set('mode', 'russian');
                    window.location.href = url.toString();
                }}
                
                function retryWithVPN() {{
                    const url = new URL(window.location);
                    url.searchParams.set('mode', 'antibot');
                    window.location.href = url.toString();
                }}
                
                function retryWithDifferentUA() {{
                    const url = new URL(window.location);
                    url.searchParams.set('ua', 'mobile');
                    window.location.href = url.toString();
                }}
                
                function retryWithAntiBot() {{
                    const url = new URL(window.location);
                    url.searchParams.set('mode', 'antibot');
                    window.location.href = url.toString();
                }}
                
                function retryGeneral() {{
                    window.location.reload();
                }}
                
                function openDirect() {{
                    window.open('{}', '_blank');
                }}
            </script>
        </body>
        </html>
        "#, domain, url, error, specific_solutions, url)
    }

    fn inject_cors_and_security_fixes(html: &str, _url: &str) -> String {
        let mut enhanced_html = html.to_string();
        
        // 🔧 ERWEITERTE CSP-ENTFERNUNG UND CORS-FIXES
        
        // Entferne alle Content-Security-Policy Headers und Meta-Tags (erweitert)
        enhanced_html = regex::Regex::new(r#"(?i)<meta[^>]*http-equiv\s*=\s*["']?content-security-policy["']?[^>]*>"#)
            .unwrap()
            .replace_all(&enhanced_html, "")
            .to_string();
        
        // Entferne auch CSP-Nonce-Attribute
        enhanced_html = regex::Regex::new(r#"(?i)\s*nonce\s*=\s*["'][^"']*["']"#)
            .unwrap()
            .replace_all(&enhanced_html, "")
            .to_string();
        
        // Entferne X-Frame-Options Meta-Tags
        enhanced_html = regex::Regex::new(r#"(?i)<meta[^>]*http-equiv\s*=\s*["']?x-frame-options["']?[^>]*>"#)
            .unwrap()
            .replace_all(&enhanced_html, "")
            .to_string();
        
        // Entferne Referrer-Policy Beschränkungen
        enhanced_html = regex::Regex::new(r#"(?i)<meta[^>]*name\s*=\s*["']?referrer["']?[^>]*>"#)
            .unwrap()
            .replace_all(&enhanced_html, "")
            .to_string();
        
        // Entferne Permissions-Policy Meta-Tags
        enhanced_html = regex::Regex::new(r#"(?i)<meta[^>]*http-equiv\s*=\s*["']?permissions-policy["']?[^>]*>"#)
            .unwrap()
            .replace_all(&enhanced_html, "")
            .to_string();
        
        // Entferne Feature-Policy Meta-Tags
        enhanced_html = regex::Regex::new(r#"(?i)<meta[^>]*http-equiv\s*=\s*["']?feature-policy["']?[^>]*>"#)
            .unwrap()
            .replace_all(&enhanced_html, "")
            .to_string();
        
        // 🚀 ULTIMATIVE UNIVERSELLE CORS UND SECURITY HEADERS INJECTION
        let security_injection = r#"
<script>
// 🌐 ULTIMATIVE UNIVERSELLE CORS UND SECURITY FIXES - ORA BROWSER
(function() {
    'use strict';
    
    console.log('🔧 Ora Browser: Applying ULTIMATE universal security fixes...');
    
    // 🛡️ TOTALER CSP-BYPASS: Überschreibe alle CSP-relevanten Funktionen
    
    // Deaktiviere CSP-Enforcement komplett
    if (window.SecurityPolicyViolationEvent) {
        const originalAddEventListener = EventTarget.prototype.addEventListener;
        EventTarget.prototype.addEventListener = function(type, listener, options) {
            if (type === 'securitypolicyviolation') {
                console.log('🚫 CSP violation event blocked');
                return;
            }
            return originalAddEventListener.call(this, type, listener, options);
        };
    }
    
    // Überschreibe eval() für CSP-Umgehung
    const originalEval = window.eval;
    window.eval = function(code) {
        try {
            return originalEval.call(this, code);
        } catch (e) {
            if (e.message && e.message.includes('Content Security Policy')) {
                console.log('🔓 CSP eval restriction bypassed');
                return Function(code)();
            }
            throw e;
        }
    };
    
    // Überschreibe Function constructor
    const originalFunction = window.Function;
    window.Function = function(...args) {
        try {
            return originalFunction.apply(this, args);
        } catch (e) {
            if (e.message && e.message.includes('Content Security Policy')) {
                console.log('🔓 CSP Function restriction bypassed');
                // Fallback-Implementierung
                const code = args[args.length - 1];
                return originalEval.call(window, `(function(${args.slice(0, -1).join(', ')}) { ${code} })`);
            }
            throw e;
        }
    };
    
    // 🌐 ULTIMATIVE FETCH API OVERRIDE
    const originalFetch = window.fetch;
    window.fetch = function(url, options = {}) {
        // Entferne alle CORS-Beschränkungen
        options.mode = 'no-cors';
        options.credentials = 'include';
        options.headers = options.headers || {};
        
        // Füge ultimative universelle Headers hinzu
        if (typeof options.headers === 'object') {
            Object.assign(options.headers, {
                'Access-Control-Allow-Origin': '*',
                'Access-Control-Allow-Methods': '*',
                'Access-Control-Allow-Headers': '*',
                'Access-Control-Allow-Credentials': 'true',
                'X-Requested-With': 'XMLHttpRequest',
                'Cache-Control': 'no-cache',
                'Pragma': 'no-cache',
                'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 OraBrowser/1.0'
            });
        }
        
        return originalFetch.call(this, url, options).catch(error => {
            console.log('🔄 Fetch failed, trying ultimate fallback:', error);
            // Ultimativer Fallback - versuche alle Modi
            const fallbackModes = ['cors', 'no-cors', 'same-origin'];
            return fallbackModes.reduce((promise, mode) => {
                return promise.catch(() => {
                    const fallbackOptions = { ...options, mode };
                    return originalFetch.call(this, url, fallbackOptions);
                });
            }, Promise.reject(error));
        });
    };
    
    // 🌐 XMLHTTPREQUEST OVERRIDE
    const originalXHR = window.XMLHttpRequest;
    window.XMLHttpRequest = function() {
        const xhr = new originalXHR();
        const originalOpen = xhr.open;
        const originalSend = xhr.send;
        
        xhr.open = function(method, url, async, user, password) {
            console.log('🌐 XHR Override:', method, url);
            return originalOpen.call(this, method, url, async, user, password);
        };
        
        xhr.send = function(data) {
            // Setze universelle Headers
            try {
                xhr.setRequestHeader('Access-Control-Allow-Origin', '*');
                xhr.setRequestHeader('Access-Control-Allow-Methods', '*');
                xhr.setRequestHeader('Access-Control-Allow-Headers', '*');
                xhr.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
            } catch (e) {
                console.log('🔄 XHR header setting failed, continuing anyway');
            }
            return originalSend.call(this, data);
        };
        
        return xhr;
    };
    
    // 🖱️ ULTIMATIVE CLICK-EVENT-FIXES
    function enableAllInteractions() {
        console.log('🖱️ Ora Browser: Enabling ALL interactions...');
        
        // Entferne alle pointer-events: none Styles AGGRESSIV
        const style = document.createElement('style');
        style.textContent = `
            *, *::before, *::after { 
                pointer-events: auto !important; 
                user-select: auto !important;
                -webkit-user-select: auto !important;
                -moz-user-select: auto !important;
                -ms-user-select: auto !important;
            }
            button, input, select, textarea, a, [onclick], [role="button"], [data-ved], [jsaction] { 
                pointer-events: auto !important; 
                cursor: pointer !important; 
                opacity: 1 !important;
                visibility: visible !important;
                display: block !important;
                position: relative !important;
                z-index: auto !important;
            }
            [disabled], [readonly] {
                pointer-events: auto !important;
                cursor: pointer !important;
                opacity: 1 !important;
            }
            .disabled, .readonly {
                pointer-events: auto !important;
                cursor: pointer !important;
                opacity: 1 !important;
            }
        `;
        document.head.appendChild(style);
        
        // Force-Enable ALLE interaktiven Elemente
        const allElements = document.querySelectorAll('*');
        allElements.forEach(element => {
            // Entferne alle Disabled-Attribute
            element.removeAttribute('disabled');
            element.removeAttribute('readonly');
            element.removeAttribute('inert');
            
            // Setze interaktive Styles
            if (element.tagName && ['BUTTON', 'INPUT', 'SELECT', 'TEXTAREA', 'A'].includes(element.tagName)) {
                element.style.pointerEvents = 'auto';
                element.style.cursor = 'pointer';
                element.style.opacity = '1';
                element.style.visibility = 'visible';
                element.disabled = false;
            }
            
            // Aktiviere alle Elemente mit Event-Attributen
            if (element.hasAttribute('onclick') || 
                element.hasAttribute('onmousedown') || 
                element.hasAttribute('onmouseup') ||
                element.hasAttribute('data-ved') ||
                element.hasAttribute('jsaction') ||
                element.getAttribute('role') === 'button') {
                element.style.pointerEvents = 'auto';
                element.style.cursor = 'pointer';
                element.style.opacity = '1';
                element.style.visibility = 'visible';
            }
        });
        
        console.log(`✅ Processed ${allElements.length} elements for interaction`);
    }
    
    // 🍪 ULTIMATIVE COOKIE-POPUP-FIXES
    function fixCookiePopups() {
        console.log('🍪 Ora Browser: Fixing ALL cookie popups...');
        
        // Erweiterte Google-spezifische Cookie-Button-IDs (AKZEPTIEREN)
        const googleSelectors = [
            '#L2AGLb', '#VnjCcb', '#W0wltc', // Google Deutschland
            'button[aria-label*="Accept"]', 'button[aria-label*="Akzeptieren"]',
            'button[aria-label*="Accept all"]', 'button[aria-label*="Alle akzeptieren"]',
            'button[aria-label*="I agree"]', 'button[aria-label*="Ich stimme zu"]',
            '[data-ved] button:first-child', 'form[action*="consent"] button:first-child',
            '[jsname] button:first-child', '[data-testid*="accept"] button'
        ];
        
        googleSelectors.forEach(selector => {
            const elements = document.querySelectorAll(selector);
            elements.forEach(btn => {
                btn.style.pointerEvents = 'auto';
                btn.disabled = false;
                btn.style.cursor = 'pointer';
                btn.style.opacity = '1';
                btn.style.visibility = 'visible';
                btn.removeAttribute('disabled');
                btn.removeAttribute('readonly');
                console.log('✅ Google accept button enabled:', selector);
                
                // Auto-click für Akzeptieren-Buttons (nach 1 Sekunde)
                if (btn.textContent && 
                    (btn.textContent.toLowerCase().includes('akzeptieren') || 
                     btn.textContent.toLowerCase().includes('accept') ||
                     btn.textContent.toLowerCase().includes('agree') ||
                     btn.getAttribute('aria-label')?.toLowerCase().includes('akzeptieren') ||
                     btn.getAttribute('aria-label')?.toLowerCase().includes('accept'))) {
                    setTimeout(() => {
                        console.log('🍪 Auto-clicking accept button:', selector);
                        btn.click();
                    }, 1000);
                }
            });
        });
        
        // ULTIMATIVE universelle Cookie-Banner-Selektoren
        const universalSelectors = [
            '[id*="cookie"]', '[class*="cookie"]', '[data-testid*="cookie"]',
            '[id*="consent"]', '[class*="consent"]', '[data-testid*="consent"]',
            '[id*="accept"]', '[class*="accept"]', '[data-testid*="accept"]',
            '[id*="agree"]', '[class*="agree"]', '[data-testid*="agree"]',
            '[aria-label*="Accept"]', '[aria-label*="Akzeptieren"]',
            '[aria-label*="Agree"]', '[aria-label*="Zustimmen"]',
            'button[type="submit"]', 'input[type="submit"]',
            '[role="button"]', '[jsaction]', '[data-ved]'
        ];
        
        universalSelectors.forEach(selector => {
            const elements = document.querySelectorAll(selector);
            elements.forEach(element => {
                element.style.pointerEvents = 'auto';
                element.disabled = false;
                element.style.cursor = 'pointer';
                element.style.opacity = '1';
                element.style.visibility = 'visible';
                element.removeAttribute('disabled');
                element.removeAttribute('readonly');
            });
        });
        
        // Verstecke Ablehnungs-Popups
        const rejectSelectors = [
            '[id*="reject"]', '[class*="reject"]',
            '[id*="decline"]', '[class*="decline"]',
            '[aria-label*="Reject"]', '[aria-label*="Ablehnen"]'
        ];
        
        rejectSelectors.forEach(selector => {
            const elements = document.querySelectorAll(selector);
            elements.forEach(element => {
                // Mache Ablehnungs-Buttons weniger prominent
                element.style.opacity = '0.5';
                element.style.fontSize = '12px';
            });
        });
    }
    
    // 🔗 ULTIMATIVE LINK-FIXES
    function fixAllLinks() {
        console.log('🔗 Ora Browser: Fixing ALL links...');
        
        const allLinks = document.querySelectorAll('a, [href], [data-href], [onclick*="location"], [onclick*="window.open"]');
        allLinks.forEach(link => {
            link.style.pointerEvents = 'auto';
            link.style.cursor = 'pointer';
            link.style.textDecoration = 'underline';
            link.style.color = '#0066cc';
            
            // Entferne target="_blank" Beschränkungen
            if (link.hasAttribute('target')) {
                link.removeAttribute('target');
            }
            
            // Entferne rel="noopener" Beschränkungen
            if (link.hasAttribute('rel')) {
                link.removeAttribute('rel');
            }
        });
        
        console.log(`✅ Fixed ${allLinks.length} links`);
    }
    
    // 📑 ULTIMATIVE TAB-FIXES
    function fixAllTabs() {
        console.log('📑 Ora Browser: Fixing ALL tabs...');
        
        // Entferne window.open Beschränkungen
        const originalWindowOpen = window.open;
        window.open = function(url, name, features) {
            console.log('🔗 Window.open intercepted:', url);
            return originalWindowOpen.call(this, url, name || '_blank', features);
        };
        
        // Entferne Popup-Blocker
        Object.defineProperty(window, 'opener', {
            get: function() { return window; },
            set: function() { return true; }
        });
        
        // Erlaube alle Tab-Operationen
        const originalClose = window.close;
        window.close = function() {
            console.log('🔗 Window.close intercepted');
            return originalClose.call(this);
        };
    }
    
    // 🔄 ULTIMATIVER MUTATION-OBSERVER FÜR DYNAMISCHE INHALTE
    const observer = new MutationObserver(function(mutations) {
        mutations.forEach(function(mutation) {
            if (mutation.type === 'childList') {
                mutation.addedNodes.forEach(function(node) {
                    if (node.nodeType === 1) { // Element node
                        // Aktiviere ALLE neuen Elemente
                        const allNewElements = node.querySelectorAll ? 
                            node.querySelectorAll('*') : [];
                        allNewElements.forEach(element => {
                            element.style.pointerEvents = 'auto';
                            element.disabled = false;
                            element.style.cursor = element.tagName && ['BUTTON', 'A', 'INPUT'].includes(element.tagName) ? 'pointer' : 'auto';
                            element.removeAttribute('disabled');
                            element.removeAttribute('readonly');
                        });
                        
                        // Prüfe auf neue Cookie-Popups
                        if (node.id && (node.id.includes('cookie') || node.id.includes('consent') || node.id.includes('banner'))) {
                            setTimeout(fixCookiePopups, 100);
                        }
                        
                        // Prüfe auf neue Links
                        if (node.tagName === 'A' || node.querySelector('a')) {
                            setTimeout(fixAllLinks, 100);
                        }
                    }
                });
            }
            
            // Reagiere auf Attribut-Änderungen
            if (mutation.type === 'attributes') {
                const element = mutation.target;
                if (mutation.attributeName === 'disabled' || 
                    mutation.attributeName === 'readonly' ||
                    mutation.attributeName === 'style') {
                    element.style.pointerEvents = 'auto';
                    element.disabled = false;
                    element.removeAttribute('disabled');
                    element.removeAttribute('readonly');
                }
            }
        });
    });
    
    // 🚀 GLOBALE HELPER-FUNKTIONEN
    window.oraBrowserFixAll = function() {
        enableAllInteractions();
        fixCookiePopups();
        fixAllLinks();
        fixAllTabs();
        console.log('🚀 Ora Browser: All fixes reapplied manually');
    };
    
    window.oraBrowserForceClick = function(selector) {
        const elements = document.querySelectorAll(selector);
        elements.forEach(el => {
            el.style.pointerEvents = 'auto';
            el.disabled = false;
            el.click();
            console.log('🖱️ Force-clicked:', selector);
        });
    };
    
    // Starte alle Fixes
    function initializeAllFixes() {
        enableAllInteractions();
        fixCookiePopups();
        fixAllLinks();
        fixAllTabs();
        
        // Starte Observer
        if (document.body) {
            observer.observe(document.body, { 
                childList: true, 
                subtree: true, 
                attributes: true,
                attributeFilter: ['disabled', 'readonly', 'style', 'class']
            });
        }
    }
    
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initializeAllFixes);
    } else {
        initializeAllFixes();
    }
    
    // Wiederhole Fixes alle 1 Sekunde für ultimative Kompatibilität
    setInterval(function() {
        enableAllInteractions();
        fixCookiePopups();
        fixAllLinks();
    }, 1000);
    
    console.log('✅ Ora Browser: ULTIMATE universal security fixes applied - ALL restrictions bypassed');
})();
</script>
"#;
        
        // Injiziere die Security-Fixes in den HTML-Head
        if enhanced_html.contains("<head>") {
            enhanced_html = enhanced_html.replace("<head>", &format!("<head>{}", security_injection));
        } else if enhanced_html.contains("<html>") {
            enhanced_html = enhanced_html.replace("<html>", &format!("<html>{}", security_injection));
        } else {
            enhanced_html = format!("{}{}", security_injection, enhanced_html);
        }
        
        enhanced_html
    }

    async fn handle_proxy(
        url: String,
    ) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
        println!("🔄 Proxy request for: {}", url);
        
        // 🚀 VERWENDE ERWEITERTE FETCH-STRATEGIEN
        match Self::fetch_with_method(&url, "GET", "").await {
            Ok(response) => {
                println!("✅ Proxy response: {} bytes", response.content.len());
                
                // 🔧 ERWEITERTE CONTENT-VERARBEITUNG
                let enhanced_content = Self::inject_cors_and_security_fixes(&response.content, &url);
                
                // 🌐 ERSTELLE RESPONSE MIT UNIVERSELLEN HEADERS
                let response = warp::http::Response::builder()
                    .status(200)
                    .header("Content-Type", "text/html; charset=utf-8")
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS, HEAD, PATCH")
                    .header("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With, Accept, Origin, Cache-Control, Pragma, User-Agent, Accept-Language, Accept-Encoding, DNT, Connection, Upgrade-Insecure-Requests, Sec-Fetch-Dest, Sec-Fetch-Mode, Sec-Fetch-Site, Sec-Fetch-User, X-Forwarded-For, X-Real-IP, Referer")
                    .header("Access-Control-Allow-Credentials", "true")
                    .header("Access-Control-Expose-Headers", "*")
                    .header("Access-Control-Max-Age", "86400")
                    .header("Vary", "Origin")
                    // 🛡️ EXPLIZITE CSP-BYPASS-HEADER
                    .header("Content-Security-Policy", "default-src * 'unsafe-inline' 'unsafe-eval' data: blob:; script-src * 'unsafe-inline' 'unsafe-eval'; style-src * 'unsafe-inline'; img-src * data: blob:; font-src * data:; connect-src * data: blob:; media-src * data: blob:; object-src *; child-src * data: blob:; frame-src * data: blob:; worker-src * data: blob:; frame-ancestors *; form-action *; base-uri *;")
                    .header("X-Frame-Options", "ALLOWALL")
                    .header("X-Content-Type-Options", "nosniff")
                    .header("Referrer-Policy", "no-referrer-when-downgrade")
                    .header("Permissions-Policy", "accelerometer=*, camera=*, geolocation=*, gyroscope=*, magnetometer=*, microphone=*, payment=*, usb=*")
                    // 🚀 ZUSÄTZLICHE SICHERHEITS-BYPASS-HEADER
                    .header("Cross-Origin-Embedder-Policy", "unsafe-none")
                    .header("Cross-Origin-Opener-Policy", "unsafe-none")
                    .header("Cross-Origin-Resource-Policy", "cross-origin")
                    .header("X-Permitted-Cross-Domain-Policies", "all")
                    .header("X-XSS-Protection", "0")
                    .body(enhanced_content)
                    .unwrap();
                
                Ok(Box::new(response))
            }
            Err(e) => {
                println!("❌ Proxy error: {}", e);
                
                // 🔄 ERWEITERTE FEHLERBEHANDLUNG MIT FALLBACK-STRATEGIEN
                println!("🔄 Trying fallback strategies for: {}", url);
                
                // Versuche alternative URLs
                let alternative_urls = Self::generate_universal_url_alternatives(&url);
                for alt_url in alternative_urls.iter().take(3) {
                    println!("🔄 Trying alternative URL: {}", alt_url);
                    if let Ok(response) = Self::fetch_with_method(alt_url, "GET", "").await {
                        println!("✅ Alternative URL succeeded: {}", alt_url);
                        let enhanced_content = Self::inject_cors_and_security_fixes(&response.content, alt_url);
                        
                        let response = warp::http::Response::builder()
                            .status(200)
                            .header("Content-Type", "text/html; charset=utf-8")
                            .header("Access-Control-Allow-Origin", "*")
                            .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS, HEAD")
                            .header("X-Frame-Options", "ALLOWALL")
                            .body(enhanced_content)
                            .unwrap();
                        
                        return Ok(Box::new(response));
                    }
                }
                
                // 📄 ERSTELLE ERWEITERTE FEHLERSEITE
                let error_page = Self::create_enhanced_error_page(&url, &e.to_string());
                let response = warp::http::Response::builder()
                    .status(502)
                    .header("Content-Type", "text/html; charset=utf-8")
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS, HEAD")
                    .body(error_page)
                    .unwrap();
                
                Ok(Box::new(response))
            }
        }
    }

    // 📄 ERWEITERTE FEHLERSEITE
    fn create_enhanced_error_page(url: &str, error: &str) -> String {
        format!(r#"
<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ora Browser - Verbindungsfehler</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 0;
            padding: 40px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }}
        .error-container {{
            background: rgba(255,255,255,0.1);
            padding: 40px;
            border-radius: 20px;
            backdrop-filter: blur(10px);
            box-shadow: 0 8px 32px rgba(0,0,0,0.3);
            text-align: center;
            max-width: 600px;
            width: 100%;
        }}
        .error-icon {{
            font-size: 64px;
            margin-bottom: 20px;
        }}
        .error-title {{
            font-size: 28px;
            margin-bottom: 16px;
            font-weight: 600;
        }}
        .error-message {{
            font-size: 16px;
            margin-bottom: 30px;
            opacity: 0.9;
            line-height: 1.5;
        }}
        .error-details {{
            background: rgba(0,0,0,0.2);
            padding: 20px;
            border-radius: 10px;
            margin: 20px 0;
            font-family: monospace;
            font-size: 14px;
            text-align: left;
            word-break: break-all;
        }}
        .retry-options {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 16px;
            margin-top: 30px;
        }}
        .retry-btn {{
            background: rgba(255,255,255,0.2);
            border: 2px solid rgba(255,255,255,0.3);
            color: white;
            padding: 16px 24px;
            border-radius: 10px;
            cursor: pointer;
            transition: all 0.3s ease;
            text-decoration: none;
            display: block;
            font-weight: 500;
        }}
        .retry-btn:hover {{
            background: rgba(255,255,255,0.3);
            border-color: rgba(255,255,255,0.5);
            transform: translateY(-2px);
        }}
        .url-display {{
            background: rgba(255,255,255,0.1);
            padding: 12px;
            border-radius: 8px;
            margin: 20px 0;
            font-family: monospace;
            word-break: break-all;
        }}
    </style>
</head>
<body>
    <div class="error-container">
        <div class="error-icon">🌐❌</div>
        <h1 class="error-title">Verbindungsfehler</h1>
        <p class="error-message">
            Die Website konnte nicht geladen werden. Ora Browser hat verschiedene Strategien versucht, 
            aber die Verbindung ist fehlgeschlagen.
        </p>
        
        <div class="url-display">
            <strong>URL:</strong> {}
        </div>
        
        <div class="error-details">
            <strong>Fehlerdetails:</strong><br>
            {}
        </div>
        
        <div class="retry-options">
            <a href="javascript:location.reload()" class="retry-btn">
                🔄 Erneut versuchen
            </a>
            <a href="javascript:window.open('{}', '_blank')" class="retry-btn">
                🌐 In neuem Fenster öffnen
            </a>
            <a href="javascript:window.open('{}', '_blank')" class="retry-btn">
                🔓 HTTP-Version versuchen
            </a>
            <a href="javascript:window.open('{}', '_blank')" class="retry-btn">
                📱 Mobile Version versuchen
            </a>
        </div>
        
        <p style="margin-top: 30px; opacity: 0.7; font-size: 14px;">
            🛡️ Ora Browser Proxy - Erweiterte Verbindungsstrategien
        </p>
    </div>
</body>
</html>
"#, 
            url, 
            error,
            url,
            url.replace("https://", "http://"),
            url.replace("www.", "m.")
        )
    }
} 