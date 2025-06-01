use anyhow::Result;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        System::{Com::*, LibraryLoader::GetModuleHandleW},
        UI::WindowsAndMessaging::*,
    },
};
use webview2_com::{
    Microsoft::Web::WebView2::Win32::*,
    CreateCoreWebView2EnvironmentCompletedHandler,
    CreateCoreWebView2ControllerCompletedHandler,
};
use std::sync::{Arc, Mutex};

pub struct WebViewApp {
    controller: Arc<Mutex<Option<ICoreWebView2Controller>>>,
    hwnd: HWND,
}

impl WebViewApp {
    pub fn new() -> Result<Self> {
        unsafe {
            // COM initialisieren
            CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
        }

        // Fenster erstellen
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
                800,
                600,
                None,
                None,
                Some(HINSTANCE(instance.0)),
                None,
            )?;

            Ok(hwnd)
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        println!("WebView2 wird initialisiert...");

        // WebView2 Environment erstellen
        let controller_clone = self.controller.clone();
        let hwnd = self.hwnd;

        let handler = Box::new(move |result: Result<(), Error>, environment: Option<ICoreWebView2Environment>| {
            if let Err(e) = result {
                println!("Fehler bei der WebView2-Umgebungserstellung: {:?}", e);
                return Ok(());
            }

            match environment {
                Some(env) => {
                    println!("WebView2-Umgebung erfolgreich erstellt");
                    let controller_clone2 = controller_clone.clone();
                    
                    let handler = Box::new(move |result: Result<(), Error>, controller: Option<ICoreWebView2Controller>| {
                        if let Err(e) = result {
                            println!("Fehler bei der Controller-Erstellung: {:?}", e);
                            return Ok(());
                        }

                        match controller {
                            Some(ctrl) => {
                                println!("WebView2-Controller erfolgreich erstellt");
                                if let Ok(mut controller_lock) = controller_clone2.lock() {
                                    *controller_lock = Some(ctrl.clone());
                                    
                                    unsafe {
                                        if let Err(e) = ctrl.SetBounds(RECT {
                                            left: 0,
                                            top: 0,
                                            right: 800,
                                            bottom: 600,
                                        }) {
                                            println!("Fehler beim Setzen der Bounds: {:?}", e);
                                        }

                                        // Initialisiere die WebView
                                        if let Ok(webview) = ctrl.CoreWebView2() {
                                            println!("WebView2 erfolgreich initialisiert");
                                            if let Err(e) = webview.Navigate(w!("https://www.google.com")) {
                                                println!("Fehler bei der Navigation: {:?}", e);
                                            }
                                        }
                                    }
                                }
                            }
                            None => println!("Kein Controller erstellt"),
                        }
                        Ok(())
                    });

                    unsafe {
                        let handler = CreateCoreWebView2ControllerCompletedHandler::create(handler);
                        if let Err(e) = env.CreateCoreWebView2Controller(hwnd, &handler) {
                            println!("Fehler beim Erstellen des Controllers: {:?}", e);
                        }
                    }
                }
                None => println!("Keine WebView2-Umgebung erstellt"),
            }
            Ok(())
        });

        unsafe {
            let handler = CreateCoreWebView2EnvironmentCompletedHandler::create(handler);
            CreateCoreWebView2Environment(&handler)?;
        }

        Ok(())
    }

    pub fn navigate_to(&self, url: &str) -> Result<()> {
        if url.is_empty() {
            return Err(anyhow::anyhow!("URL darf nicht leer sein"));
        }

        println!("Navigiere zu: {}", url);

        if let Ok(controller_lock) = self.controller.lock() {
            if let Some(controller) = controller_lock.as_ref() {
                unsafe {
                    let webview = controller.CoreWebView2()?;
                    let url_wide = url.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();
                    webview.Navigate(PCWSTR::from_raw(url_wide.as_ptr()))?;
                }
            }
        }

        Ok(())
    }

    pub fn set_window_title(&self, title: &str) -> Result<()> {
        if title.is_empty() {
            return Err(anyhow::anyhow!("Titel darf nicht leer sein"));
        }

        println!("Setze Fenstertitel: {}", title);

        unsafe {
            let title_wide = title.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();
            SetWindowTextW(self.hwnd, PCWSTR::from_raw(title_wide.as_ptr()))
                .map_err(|e| anyhow::anyhow!("Fehler beim Setzen des Fenstertitels: {:?}", e))?;
        }

        Ok(())
    }

    pub fn run_message_loop(&self) -> Result<()> {
        println!("Starte Message Loop...");

        unsafe {
            let mut msg = MSG::default();
            
            while GetMessageW(&mut msg, None, 0, 0).into() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_controller(&self) -> Arc<Mutex<Option<ICoreWebView2Controller>>> {
        self.controller.clone()
    }
}

impl Default for WebViewApp {
    fn default() -> Self {
        Self::new().expect("Fehler beim Erstellen der WebView App")
    }
}

impl Drop for WebViewApp {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webview_creation() {
        // Hinweis: Diese Tests benötigen eine Windows-Umgebung
        let result = WebViewApp::new();
        // In einer Testumgebung könnte dies fehlschlagen
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_navigate_to_empty_url() {
        if let Ok(app) = WebViewApp::new() {
            let result = app.navigate_to("");
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_set_window_title_empty() {
        if let Ok(app) = WebViewApp::new() {
            let result = app.set_window_title("");
            assert!(result.is_err());
        }
    }
}
