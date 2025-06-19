// 🌐 ORA BROWSER - TAURI v2 EDITION
// Modern Cross-Platform Web Browser built with Rust + Tauri
// Copyright © 2024 Ora Browser Team

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

// 📦 INTERNAL MODULES
mod internal_webview2_navigation;
mod proxy_server;
mod smart_proxy;
mod browser_features;
mod ethical_safeguards;
mod browser_state;
mod tauri_commands;
mod url_utils;
mod plugin_manager;

// 📥 IMPORTS
use browser_state::BrowserState;
use proxy_server::ProxyServer;
use tauri_commands::*;

// 🚀 MAIN FUNCTION - TAURI v2
fn main() {
    println!("🚀 Starting Ora Browser with Tauri v2...");
    
    // Verbesserte Fehlerbehandlung für kritische Initialisierung
    let result = std::panic::catch_unwind(|| {
        tauri::Builder::default()
            .plugin(tauri_plugin_shell::init())
            .setup(|app| {
                // Sichere Initialisierung mit Fehlerbehandlung
                match setup_browser_state(app) {
                    Ok(_) => {
                        println!("✅ Ora Browser window created successfully!");
                        println!("🌐 Ready for cross-platform browsing!");
                        Ok(())
                    },
                    Err(e) => {
                        eprintln!("❌ Failed to setup browser state: {}", e);
                        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))
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
                 }
             })
             .run(tauri::generate_context!())
    });
    
    match result {
        Ok(run_result) => {
            if let Err(e) = run_result {
                eprintln!("❌ Tauri application error: {}", e);
                std::process::exit(1);
            }
        },
        Err(_) => {
            eprintln!("❌ Critical panic occurred during startup");
            std::process::exit(1);
        }
    }
}

fn setup_browser_state(app: &mut tauri::App) -> Result<(), String> {
    println!("🔧 Setting up browser state...");
    
    // 1. Proxy Server starten
    start_proxy_server()?;
    
    // 2. Browser State initialisieren
    let state = BrowserState::new();
    app.manage(state);
    
    // 3. Window konfigurieren
    setup_main_window(app)?;
    
    // 4. Event Handler registrieren
    setup_event_handlers(app);
    
    println!("✅ Browser state setup completed");
    Ok(())
}

fn start_proxy_server() -> Result<(), String> {
    println!("🌐 Starting proxy server on port 3030...");
    
    let proxy_handle = std::thread::Builder::new()
        .name("proxy-server".to_string())
        .stack_size(4 * 1024 * 1024) // 4MB Stack
        .spawn(|| {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .max_blocking_threads(2)
                .enable_all()
                .build();
                
            match rt {
                Ok(runtime) => {
                    runtime.block_on(async {
                        let mut proxy_server = ProxyServer::new(3030);
                        match proxy_server.start().await {
                            Ok(_) => println!("✅ Proxy server started successfully on port 3030"),
                            Err(e) => {
                                eprintln!("❌ Failed to start proxy server: {}", e);
                                // Versuche alternativen Port
                                println!("🔄 Trying alternative port 3031...");
                                let mut alt_proxy = ProxyServer::new(3031);
                                if let Err(e2) = alt_proxy.start().await {
                                    eprintln!("❌ Alternative port also failed: {}", e2);
                                }
                            }
                        }
                    });
                }
                Err(e) => {
                    eprintln!("❌ Failed to create Tokio runtime: {}", e);
                }
            }
        });
    
    match proxy_handle {
        Ok(_) => {
            println!("✅ Proxy server thread spawned successfully");
            // Längere Pause um dem Proxy-Server Zeit zum Starten zu geben
            std::thread::sleep(std::time::Duration::from_millis(2000));
            
            // Teste ob der Proxy-Server tatsächlich läuft
            let test_result = std::thread::spawn(|| {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    match reqwest::get("http://localhost:3030/health").await {
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
                    Ok(())
                },
                _ => {
                    println!("⚠️ Proxy server may not be running properly, but continuing...");
                    Ok(())
                }
            }
        },
        Err(e) => {
            eprintln!("⚠️ Failed to spawn proxy server thread: {}", e);
            println!("🔄 Browser will continue without proxy server");
            // Nicht kritisch - Browser kann ohne Proxy laufen
            Ok(())
        }
    }
}

fn setup_main_window(app: &tauri::App) -> Result<(), String> {
    let window = app.get_webview_window("main")
        .ok_or("Failed to get main window")?;
    
    window.set_title("Ora Browser")
        .map_err(|e| format!("Failed to set window title: {}", e))?;
    
    Ok(())
}

fn setup_event_handlers(_app: &tauri::App) {
    // Event Handler werden in der setup-Funktion über .on_window_event registriert
    // Diese Funktion ist für zukünftige Event-Handler reserviert
} 