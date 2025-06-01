#[cfg(windows)]
mod windows_webview {
    use anyhow::Result;
    use std::sync::{Arc, Mutex};
    use webview2_com::{
        CreateCoreWebView2ControllerCompletedHandler, CreateCoreWebView2EnvironmentCompletedHandler,
        Microsoft::Web::WebView2::Win32::*,
    };
    use windows::{
        core::{PCWSTR, w},
        Win32::{
            Foundation::*,
            System::{Com::*, LibraryLoader::GetModuleHandleW},
            UI::WindowsAndMessaging::*,
        },
    };

    const WINDOW_WIDTH: i32 = 800;
    const WINDOW_HEIGHT: i32 = 600;
    const DEFAULT_URL: &str = "https://www.google.com";

    #[derive(Debug)]
    pub enum WebViewError {
        EmptyInput(String),
        ControllerError(String),
        NavigationError(String),
    }

    // Implementierung von Display
    impl std::fmt::Display for WebViewError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                WebViewError::EmptyInput(msg) => write!(f, "Leere Eingabe: {}", msg),
                WebViewError::ControllerError(msg) => write!(f, "Controller-Fehler: {}", msg),
                WebViewError::NavigationError(msg) => write!(f, "Navigationsfehler: {}", msg),
            }
        }
    }

    // Implementierung von std::error::Error
    impl std::error::Error for WebViewError {}

    pub struct WebViewApp {
        controller: Arc<Mutex<Option<ICoreWebView2Controller>>>,
        hwnd: HWND,
    }

    impl WebViewApp {
        pub fn new() -> Result<Self> {
            unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()? };
            let hwnd = Self::create_window()?;
            Ok(Self {
                controller: Arc::new(Mutex::new(None)),
                hwnd,
            })
        }

        fn create_window() -> Result<HWND> {
            unsafe {
                let instance = GetModuleHandleW(None)?;
                let hwnd = CreateWindowExW(
                    WINDOW_EX_STYLE::default(),
                    w!("STATIC"),
                    w!("Projekt Ora"),
                    WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    WINDOW_WIDTH,
                    WINDOW_HEIGHT,
                    None,
                    None,
                    Some(HINSTANCE(instance.0)),
                    None,
                )?;
                Ok(hwnd)
            }
        }

        fn handle_to_pcwstr(s: &str) -> Vec<u16> {
            s.encode_utf16().chain(std::iter::once(0)).collect()
        }

        fn handle_controller_setup(
            controller: Option<ICoreWebView2Controller>,
            controller_clone: Arc<Mutex<Option<ICoreWebView2Controller>>>,
        ) -> windows::core::Result<()> {
            match controller {
                Some(ctrl) => {
                    println!("WebView2-Controller erfolgreich erstellt");
                    if let Ok(mut controller_lock) = controller_clone.lock() {
                        *controller_lock = Some(ctrl.clone());

                        unsafe {
                            ctrl.SetBounds(RECT {
                                left: 0,
                                top: 0,
                                right: WINDOW_WIDTH,
                                bottom: WINDOW_HEIGHT,
                            })?;

                            let webview = ctrl.CoreWebView2()?;
                            let url_wide = Self::handle_to_pcwstr(DEFAULT_URL);
                            webview.Navigate(PCWSTR::from_raw(url_wide.as_ptr()))?;
                        }
                    }
                }
                None => println!("Kein Controller erstellt"),
            }
            Ok(())
        }

        pub fn initialize(&mut self) -> Result<()> {
            println!("WebView2 wird initialisiert...");
            let controller_clone = self.controller.clone();
            let hwnd = self.hwnd;

            let environment_handler = Box::new(
                move |result: windows::core::Result<()>, environment: Option<ICoreWebView2Environment>| {
                    if let Err(e) = result {
                        println!("Fehler bei der WebView2-Umgebungserstellung: {:?}", e);
                        return Ok(());
                    }

                    if let Some(env) = environment {
                        println!("WebView2-Umgebung erfolgreich erstellt");
                        let controller_clone2 = controller_clone.clone();
                        let handler = Box::new(
                            move |result: windows::core::Result<()>,
                                  controller: Option<ICoreWebView2Controller>| {
                                if let Err(e) = result {
                                    println!("Fehler bei der Controller-Erstellung: {:?}", e);
                                    return Ok(());
                                }
                                Self::handle_controller_setup(controller, controller_clone2)
                            },
                        );

                        unsafe {
                            let handler = CreateCoreWebView2ControllerCompletedHandler::create(handler);
                            env.CreateCoreWebView2Controller(hwnd, &handler)?;
                        }
                    }
                    Ok(())
                },
            );

            unsafe {
                let handler = CreateCoreWebView2EnvironmentCompletedHandler::create(environment_handler);
                CreateCoreWebView2Environment(&handler)?;
            }

            Ok(())
        }

        pub fn navigate_to(&self, url: &str) -> Result<()> {
            if url.is_empty() {
                return Err(anyhow::anyhow!(WebViewError::EmptyInput("URL".to_string())));
            }

            println!("Navigiere zu: {}", url);
            if let Ok(controller_lock) = self.controller.lock() {
                if let Some(controller) = controller_lock.as_ref() {
                    unsafe {
                        let webview = controller.CoreWebView2()
                            .map_err(|e| WebViewError::ControllerError(e.to_string()))?;
                        let url_wide = Self::handle_to_pcwstr(url);
                        webview.Navigate(PCWSTR::from_raw(url_wide.as_ptr()))
                            .map_err(|e| WebViewError::NavigationError(e.to_string()))?;
                    }
                }
            }
            Ok(())
        }
    }

    impl Drop for WebViewApp {
        fn drop(&mut self) {
            if let Ok(controller_lock) = self.controller.lock() {
                if let Some(controller) = controller_lock.as_ref() {
                    unsafe {
                        let _ = controller.Close();
                    }
                }
            }
        }
    }
}

#[cfg(target_os = "linux")]
mod linux_webview {
    use super::*;
    use gtk::prelude::*;
    use webkit2gtk::{WebView, WebViewExt};

    pub struct WebViewApp {
        window: gtk::Window,
        webview: WebView,
    }

    impl WebViewApp {
        pub fn new() -> Result<Self> {
            gtk::init().map_err(|_| anyhow::anyhow!("Failed to initialize GTK"))?;
            
            let window = gtk::Window::new(gtk::WindowType::Toplevel);
            window.set_title("Projekt Ora");
            window.set_default_size(800, 600);
            
            let webview = WebView::new();
            window.add(&webview);
            
            window.connect_delete_event(|_, _| {
                gtk::main_quit();
                Inhibit(false)
            });

            Ok(Self { window, webview })
        }

        pub fn initialize(&mut self) -> Result<()> {
            self.window.show_all();
            Ok(())
        }

        pub fn navigate_to(&self, url: &str) -> Result<()> {
            self.webview.load_uri(url);
            Ok(())
        }

        pub fn run(&self) {
            gtk::main();
        }
    }
}

#[cfg(target_os = "macos")]
mod macos_webview {
    use super::*;
    
    pub struct WebViewApp {
        // macOS-specific implementation would go here
        // Using webkit2gtk for now as a placeholder
    }

    impl WebViewApp {
        pub fn new() -> Result<Self> {
            // macOS-specific initialization
            Ok(Self {})
        }

        pub fn initialize(&mut self) -> Result<()> {
            Ok(())
        }

        pub fn navigate_to(&self, url: &str) -> Result<()> {
            Ok(())
        }
    }
}

// Re-export the platform-specific implementation
#[cfg(windows)]
pub use windows_webview::*;

#[cfg(target_os = "linux")]
pub use linux_webview::*;

#[cfg(target_os = "macos")]
pub use macos_webview::*;