// 🌐 ZAKYX BROWSER - TAURI v2 EDITION
// Modern Cross-Platform Web Browser built with Rust + Tauri
// Copyright © 2024 ZAKYX Browser Team

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tracing::{info, error, debug};

// 📦 INTERNAL MODULES
mod internal_webview2_navigation;
mod proxy_server;
mod proxy;
mod browser;
mod ui;
mod ethical_safeguards;
mod browser_state;
mod tauri_commands;
mod url_utils;
mod plugin;
mod config;
mod metrics;
mod error;

// 📥 IMPORTS
use browser_state::BrowserState;
use proxy_server::ProxyServer;
use tauri_commands::*;
use config::ZAKYXConfig;
use metrics::{init_metrics, get_metrics};
use error::ZAKYXBrowserError;

// 🚀 MAIN FUNCTION - TAURI v2
fn main() {
    // 📝 STRUCTURED LOGGING INITIALISIERUNG
    init_logging();
    
    // 📊 METRICS SYSTEM INITIALISIEREN
    let _metrics = init_metrics();
    info!("📊 Metrics system initialized");
    
    info!("🚀 Starting ZAKYX Browser with Tauri v2...");
    
    // Verbesserte Fehlerbehandlung für kritische Initialisierung
    let result = std::panic::catch_unwind(|| {
        tauri::Builder::default()
            .plugin(tauri_plugin_shell::init())
            .setup(|app| {
                // 🔧 KONFIGURATION LADEN (hier im Setup um Lifetime-Probleme zu vermeiden)
                let config = ZAKYXConfig::load();
                info!("🔧 Configuration loaded: version {}", config.version);
                
                // Sichere Initialisierung mit Fehlerbehandlung
                match setup_browser_state(app, &config) {
                    Ok(_) => {
                        info!("✅ ZAKYX Browser window created successfully!");
                        info!("🌐 Ready for cross-platform browsing!");
                        Ok(())
                    },
                    Err(e) => {
                        error!("❌ Critical failure during browser setup: {}", e);
                        // Kritische Fehler führen zum sofortigen Beenden
                        std::process::exit(1);
                    }
                }
            })
                         .invoke_handler(tauri::generate_handler![
                 create_new_tab,
                 close_tab,
                 internal_webview_navigate,
                 navigate_to,
                 get_tabs,
                 add_bookmark,
                 get_bookmarks,
                 remove_bookmark,
                 sync_bookmarks,
                 get_settings,
                 update_settings,
                 open_external_url,
                 get_history,
                 update_tab_title,
                 navigate_internally,
                 check_internal_navigation,
                 get_webview_config,
                 get_proxy_url,
                 navigate_and_get_content,
                 get_all_plugins,
                 get_loaded_plugins,
                 enable_plugin,
                 disable_plugin,
                 load_plugin,
                 unload_plugin,
             ])
             .on_window_event(|window, event| {
                 if let tauri::WindowEvent::CloseRequested { .. } = event {
                     let app_handle = window.app_handle();
                     if let Some(state) = app_handle.try_state::<BrowserState>() {
                         state.save_all_state();
                     }
                     // 📊 METRICS SUMMARY VOR BEENDEN
                     if let Some(metrics) = get_metrics() {
                         metrics.log_summary();
                     }
                 }
             })
             .run(tauri::generate_context!())
    });
    
    match result {
        Ok(run_result) => {
            if let Err(e) = run_result {
                error!("❌ Tauri application error: {}", e);
                std::process::exit(1);
            }
        },
        Err(_) => {
            error!("❌ Critical panic occurred during startup");
            std::process::exit(1);
        }
    }
}

fn setup_browser_state(app: &mut tauri::App, config: &ZAKYXConfig) -> Result<(), ZAKYXBrowserError> {
    debug!("🔧 Setting up browser state...");
    
    // 📊 METRICS: Startup-Timer starten
    let _setup_timer = get_metrics().map(|m| m.start_timer("browser_setup"));
    
    // 1. Proxy Server starten (KRITISCH)
    start_proxy_server(config).map_err(|e| {
        error!("❌ Critical: Proxy server failed to start: {}", e);
        ZAKYXBrowserError::proxy_error(&format!("Proxy server startup failed: {}", e), None)
    })?;
    
    // 2. Browser State initialisieren (KRITISCH)
    let state = match std::panic::catch_unwind(|| BrowserState::new()) {
        Ok(state) => state,
        Err(_) => {
            error!("❌ Critical: Browser state initialization panicked");
            return Err(ZAKYXBrowserError::ui_error("browser_state", "Browser state initialization failed due to panic", false));
        }
    };
    app.manage(state);
    
    // 3. Window konfigurieren (KRITISCH)
    setup_main_window(app).map_err(|e| {
        error!("❌ Critical: Main window setup failed: {}", e);
        ZAKYXBrowserError::ui_error("main_window", &format!("Main window setup failed: {}", e), true)
    })?;
    
    // 4. Event Handler registrieren (NICHT-KRITISCH)
    setup_event_handlers(app);
    
    // 📊 METRICS: Setup abgeschlossen
    if let Some(metrics) = get_metrics() {
        metrics.count("browser_setup_completed", None);
    }
    
    info!("✅ Browser state setup completed successfully");
    Ok(())
}

fn start_proxy_server(config: &ZAKYXConfig) -> Result<(), ZAKYXBrowserError> {
    info!("🌐 Starting proxy server on port {}...", config.proxy.primary_port);
    
    // 📊 METRICS: Proxy startup messen
    let _proxy_timer = get_metrics().map(|m| m.start_timer("proxy_startup"));
    
    let proxy_config = config.proxy.clone();
    let proxy_handle = std::thread::Builder::new()
        .name("proxy-server".to_string())
        .stack_size(proxy_config.stack_size_mb * 1024 * 1024) // Konfigurierbare Stack-Größe
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .max_blocking_threads(2)
                .enable_all()
                .build();
                
            match rt {
                Ok(runtime) => {
                    runtime.block_on(async {
                        let mut proxy_server = ProxyServer::new(proxy_config.primary_port);
                        match proxy_server.start().await {
                            Ok(_) => {
                                println!("✅ Proxy server started successfully on port {}", proxy_config.primary_port);
                                // 📊 METRICS: Proxy erfolgreich gestartet
                                if let Some(metrics) = get_metrics() {
                                    metrics.count("proxy_server_started", None);
                                }
                            },
                            Err(e) => {
                                eprintln!("❌ Failed to start proxy server: {}", e);
                                // Versuche alternativen Port
                                println!("🔄 Trying alternative port {}...", proxy_config.fallback_port);
                                let mut alt_proxy = ProxyServer::new(proxy_config.fallback_port);
                                if let Err(e2) = alt_proxy.start().await {
                                    eprintln!("❌ Alternative port also failed: {}", e2);
                                    if let Some(metrics) = get_metrics() {
                                        metrics.count("proxy_server_failed", None);
                                    }
                                } else {
                                    if let Some(metrics) = get_metrics() {
                                        metrics.count("proxy_server_fallback_started", None);
                                    }
                                }
                            }
                        }
                    });
                }
                Err(e) => {
                    eprintln!("❌ Failed to create Tokio runtime: {}", e);
                    if let Some(metrics) = get_metrics() {
                        metrics.count("proxy_runtime_failed", None);
                    }
                }
            }
        });
    
    match proxy_handle {
        Ok(_) => {
            println!("✅ Proxy server thread spawned successfully");
            // Konfigurierbare Wartezeit
            let wait_time = std::time::Duration::from_millis(config.proxy.connect_timeout_secs * 100);
            std::thread::sleep(wait_time);
            
            // Teste ob der Proxy-Server tatsächlich läuft
            let test_port = config.proxy.primary_port;
            let test_result = std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    match reqwest::get(&format!("http://localhost:{}/health", test_port)).await {
                        Ok(response) => {
                            if response.status().is_success() {
                                println!("✅ Proxy server health check passed");
                                true
                            } else {
                                println!("⚠️ Proxy server health check failed: {}", response.status());
                                false
                            }
                        },
                        Err(e) => {
                            println!("⚠️ Proxy server not reachable: {}", e);
                            false
                        }
                    }
                })
            }).join();
            
            match test_result {
                Ok(true) => {
                    println!("✅ Proxy server confirmed running");
                    if let Some(metrics) = get_metrics() {
                        metrics.count("proxy_health_check_passed", None);
                    }
                    Ok(())
                },
                _ => {
                    println!("⚠️ Proxy server may not be running properly, but continuing...");
                    if let Some(metrics) = get_metrics() {
                        metrics.count("proxy_health_check_failed", None);
                    }
                    Ok(())
                }
            }
        },
        Err(e) => {
            eprintln!("⚠️ Failed to spawn proxy server thread: {}", e);
            println!("🔄 Browser will continue without proxy server");
            if let Some(metrics) = get_metrics() {
                metrics.count("proxy_spawn_failed", None);
            }
            // Nicht kritisch - Browser kann ohne Proxy laufen
            Ok(())
        }
    }
}

fn setup_main_window(app: &tauri::App) -> Result<(), ZAKYXBrowserError> {
    let window = app.get_webview_window("main")
        .ok_or_else(|| ZAKYXBrowserError::ui_error("main_window", "Failed to get main window handle", false))?;
    
                    window.set_title("ZAKYX Browser")
        .map_err(|e| ZAKYXBrowserError::ui_error("main_window", &format!("Failed to set window title: {}", e), true))?;
    
    Ok(())
}

fn setup_event_handlers(_app: &tauri::App) {
    // Event Handler werden in der setup-Funktion über .on_window_event registriert
    // Diese Funktion ist für zukünftige Event-Handler reserviert
}

// 📝 STRUCTURED LOGGING INITIALISIERUNG
fn init_logging() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    
    // Erstelle Log-Verzeichnis
    let log_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("zakyx-browser")
        .join("logs");
    
    std::fs::create_dir_all(&log_dir).unwrap_or_else(|e| {
        eprintln!("⚠️ Failed to create log directory: {}", e);
    });
    
    // File Appender für Logs
    let file_appender = tracing_appender::rolling::daily(&log_dir, "zakyx-browser.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    // Konfiguriere Tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "zakyx_browser=debug,info".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .with_ansi(true)
                .compact(),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .json(),
        )
        .init();
    
    info!("📝 Structured logging initialized");
    info!("📂 Log directory: {:?}", log_dir);
} 
