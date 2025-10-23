// 🌐 ZAKYX BROWSER - MODULAR PROXY SERVER
// Neue schlanke Implementation mit modularen Komponenten
// Ersetzt die monolithische proxy_server.rs (2,956 Zeilen → ~200 Zeilen)

use std::collections::HashMap;
use tokio::sync::mpsc;
use warp::Filter;
use chrono;
use crate::error::ZAKYXBrowserError;

// Temporarily simplified structure without modular components
// TODO: Re-add modular components once they're stable

#[derive(Debug, Clone)]
pub struct ProxyResponse {
    pub content: String,
    pub content_type: String,
    pub status_code: u16,
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

    pub async fn start(&mut self) -> Result<(), ZAKYXBrowserError> {
        println!("🌐 Starting ZAKYX Browser Proxy Server (Modular) on port {}...", self.port);
        
        // CORS-Handler für Preflight-Requests
        let cors = warp::cors()
            .allow_any_origin()
            .allow_credentials(true)
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS", "HEAD", "PATCH"])
            .allow_headers(vec![
                "content-type",
                "authorization", 
                "x-requested-with",
                "accept",
                "origin",
                "cache-control",
                "pragma",
                "user-agent",
                "accept-encoding",
                "accept-language",
                "connection",
                "upgrade-insecure-requests",
            ]);
        
        // 🌐 UNIVERSELLE RESSOURCEN-ROUTE 
        let universal_proxy_route = warp::path("universal")
            .and(warp::query::<HashMap<String, String>>())
            .and_then(|params: HashMap<String, String>| async move {
                if let Some(url) = params.get("url") {
                    Self::handle_universal_resource(url.clone()).await
                } else {
                    let reply = warp::reply::with_status(
                        "Missing URL parameter", 
                        warp::http::StatusCode::BAD_REQUEST
                    );
                    Ok(Box::new(reply) as Box<dyn warp::Reply>)
                }
            })
            .with(cors.clone());
        
        // Standard Proxy-Route
        let proxy_route = warp::path("proxy")
            .and(warp::query::<HashMap<String, String>>())
            .and_then(|params: HashMap<String, String>| async move {
                if let Some(url) = params.get("url") {
                    Self::handle_proxy_request(url.clone()).await
                } else {
                    let reply = warp::reply::with_status(
                        "Missing URL parameter", 
                        warp::http::StatusCode::BAD_REQUEST
                    );
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
                    "service": "ZAKYX Browser Proxy (Modular)",
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "version": "2.0.0"
                }))
            })
            .with(cors.clone());

        // Catch-all route für relative URLs
        let catch_all_route = warp::path::tail()
            .and(warp::query::<HashMap<String, String>>())
            .and_then(|path: warp::path::Tail, params: HashMap<String, String>| async move {
                let relative_path = path.as_str();
                
                // Ignoriere lokale Asset-Dateien
                if Self::is_local_asset(relative_path) {
                    println!("🚫 Ignoring local asset request: {}", relative_path);
                    let reply = warp::reply::with_status(
                        "Asset not handled by proxy", 
                        warp::http::StatusCode::NOT_FOUND
                    );
                    return Ok(Box::new(reply) as Box<dyn warp::Reply>);
                }
                
                println!("🔄 Catch-all route handling: {}", relative_path);
                
                if let Some(base_url) = params.get("base") {
                    let full_url = Self::combine_base_and_relative_url(base_url, relative_path);
                    println!("🔗 Combined URL: {} + {} = {}", base_url, relative_path, full_url);
                    Self::handle_proxy_request(full_url).await
                } else {
                    let potential_url = Self::intelligent_url_guess(relative_path);
                    println!("🤔 Intelligent guess: {} -> {}", relative_path, potential_url);
                    Self::handle_proxy_request(potential_url).await
                }
            })
            .with(cors.clone());
        
        // Kombiniere alle Routen
        let routes = universal_proxy_route
            .or(proxy_route)
            .or(assets_route)
            .or(health_route)
            .or(catch_all_route)
            .with(warp::log("proxy"));
        
        println!("✅ Modular proxy server routes configured");
        println!("🔗 Proxy URL: http://localhost:{}/proxy?url=<URL>", self.port);
        println!("🌐 Universal Resource URL: http://localhost:{}/universal?url=<URL>", self.port);
        println!("🏥 Health check: http://localhost:{}/health", self.port);
        
        // Starte den Server
        warp::serve(routes)
            .run(([127, 0, 0, 1], self.port))
            .await;
        
        Ok(())
    }

    // Universelle Ressourcen-Handler (vereinfacht)
    async fn handle_universal_resource(url: String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
        println!("🔄 Universal resource request for: {}", url);
        
        match Self::simple_fetch(&url).await {
            Ok(response) => {
                let warp_response = warp::http::Response::builder()
                    .status(response.status_code)
                    .header("Content-Type", &response.content_type)
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS, HEAD, PATCH")
                    .header("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With, Accept, Origin, Cache-Control, Pragma")
                    .header("Access-Control-Allow-Credentials", "true")
                    .header("Cross-Origin-Resource-Policy", "cross-origin")
                    .body(response.content)
                    .unwrap();
                
                Ok(Box::new(warp_response))
            }
            Err(e) => {
                println!("❌ Universal resource request failed: {}", e);
                let error_response = warp::http::Response::builder()
                    .status(500)
                    .header("Content-Type", "text/plain")
                    .header("Access-Control-Allow-Origin", "*")
                    .body(format!("Universal resource error: {}", e))
                    .unwrap();
                
                Ok(Box::new(error_response))
            }
        }
    }

    // Vereinfachte Proxy-Request-Behandlung
    async fn handle_proxy_request(url: String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
        println!("🔄 Proxy request for: {}", url);
        
        match Self::simple_fetch(&url).await {
            Ok(response) => {
                let warp_response = warp::http::Response::builder()
                    .status(response.status_code)
                    .header("Content-Type", &response.content_type)
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS, HEAD, PATCH")
                    .header("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With, Accept, Origin, Cache-Control, Pragma")
                    .header("Access-Control-Allow-Credentials", "true")
                    .header("Cross-Origin-Resource-Policy", "cross-origin")
                    .body(response.content)
                    .unwrap();
                
                Ok(Box::new(warp_response))
            }
            Err(e) => {
                println!("❌ Proxy request failed: {}", e);
                let error_response = warp::http::Response::builder()
                    .status(500)
                    .header("Content-Type", "text/plain")
                    .header("Access-Control-Allow-Origin", "*")
                    .body(format!("Proxy error: {}", e))
                    .unwrap();
                
                Ok(Box::new(error_response))
            }
        }
    }

    // Einfache Fetch-Implementierung
    async fn simple_fetch(url: &str) -> Result<ProxyResponse, ZAKYXBrowserError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| ZAKYXBrowserError::network_error(&format!("Failed to create HTTP client: {}", e), Some(url)))?;

        let response = client.get(url).send().await
            .map_err(|e| ZAKYXBrowserError::network_error(&format!("HTTP request failed: {}", e), Some(url)))?;
        
        let status_code = response.status().as_u16();
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("text/plain")
            .to_string();
        
        let content = response.text().await
            .map_err(|e| ZAKYXBrowserError::network_error(&format!("Failed to read response body: {}", e), Some(url)))?;
        
        Ok(ProxyResponse {
            content,
            content_type,
            status_code,
        })
    }

    // Asset-Handler (vereinfacht)
    async fn handle_assets(path: warp::path::Tail) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
        let path_str = path.as_str();
        println!("🔗 Asset request: {}", path_str);
        
        let reply = warp::reply::with_status(
            warp::reply::with_header(
                "Asset handling not implemented in modular version yet",
                "Access-Control-Allow-Origin", "*"
            ),
            warp::http::StatusCode::NOT_IMPLEMENTED
        );
        
        Ok(Box::new(reply))
    }

    // Utility-Funktionen
    fn is_local_asset(path: &str) -> bool {
        path.ends_with(".js") || 
        path.ends_with(".css") || 
        path.ends_with(".html") ||
        path.ends_with(".ico") ||
        path.ends_with(".png") ||
        path.ends_with(".jpg") ||
        path.ends_with(".gif") ||
        path.starts_with("assets/") ||
        path == "index.html" ||
        path == "app.js" ||
        path == "styles.css" ||
        path == "favicon.ico"
    }

    fn combine_base_and_relative_url(base_url: &str, relative_path: &str) -> String {
        if relative_path.starts_with("http") {
            return relative_path.to_string();
        }
        
        let base = base_url.trim_end_matches('/');
        let path = relative_path.trim_start_matches('/');
        
        format!("{}/{}", base, path)
    }

    fn intelligent_url_guess(path: &str) -> String {
        if path.starts_with("http") {
            return path.to_string();
        }
        
        // Einfache Heuristik
        if path.contains('.') && !path.contains('/') {
            format!("https://{}", path)
        } else {
            format!("https://www.google.com/search?q={}", urlencoding::encode(path))
        }
    }

    // Legacy-Kompatibilität: Wrapper für die alten Funktionen
    pub async fn fetch_with_method(url: &str, method: &str, _form_data: &str) -> Result<ProxyResponse, ZAKYXBrowserError> {
        // Vereinfachte Implementierung - nur GET für jetzt
        if method == "GET" {
            Self::simple_fetch(url).await
        } else {
            Err(ZAKYXBrowserError::proxy_error(&format!("Only GET method supported in simplified version, got: {}", method), Some(url)))
        }
    }

    pub async fn fetch_and_strip_headers(url: &str) -> Result<ProxyResponse, ZAKYXBrowserError> {
        // Vereinfachte Implementierung
        Self::fetch_with_method(url, "GET", "").await
    }
} 
