use dioxus::events::FormData;
use dioxus::prelude::*;
use dioxus_desktop::tao::dpi::LogicalSize;
use dioxus_desktop::{Config, WindowBuilder};

pub use projekt_ora_core::browser::Browser as CoreBrowser;
use projekt_ora_core::BrowserEvent;
pub use projekt_ora_core::BrowserSettings;

use std::sync::Arc;

pub struct Browser {
    settings: BrowserSettings,
    current_url: String,
}

#[derive(Clone, Debug)]
struct BrowserState {
    current_url: String,
    history: Vec<String>,
    current_index: usize,
}

impl BrowserState {
    fn new(initial_url: &str) -> Self {
        Self {
            current_url: initial_url.to_string(),
            history: vec![initial_url.to_string()],
            current_index: 0,
        }
    }

    fn can_go_back(&self) -> bool {
        self.current_index > 0
    }

    fn can_go_forward(&self) -> bool {
        self.current_index < self.history.len() - 1
    }

    fn navigate(&mut self, url: String) {
        // Wenn wir nicht am Ende der Historie sind, schneiden wir sie ab
        self.history.truncate(self.current_index + 1);
        self.history.push(url.clone());
        self.current_index = self.history.len() - 1;
        self.current_url = url;
    }

    fn go_back(&mut self) -> Option<String> {
        if self.can_go_back() {
            self.current_index -= 1;
            self.current_url = self.history[self.current_index].clone();
            Some(self.current_url.clone())
        } else {
            None
        }
    }

    fn go_forward(&mut self) -> Option<String> {
        if self.can_go_forward() {
            self.current_index += 1;
            self.current_url = self.history[self.current_index].clone();
            Some(self.current_url.clone())
        } else {
            None
        }
    }
}

fn app(cx: Scope) -> Element {
    let url = use_state(&cx, || "https://www.example.com".to_string());
    
    let on_url_submit = move |_| {
        let current = url.get().clone();
        url.set(current);
    };

    render! {
        div { class: "container",
            h1 { "Projekt-Ora Browser" }
            div { class: "toolbar",
                input {
                    class: "url-input",
                    "type": "text",
                    value: "{url}",
                    oninput: move |evt| url.set(evt.value.clone())
                }
                button {
                    class: "nav-button",
                    onclick: on_url_submit,
                    "Go"
                }
            }
            div { class: "content",
                iframe {
                    src: "{url}",
                    style: "width: 100%; height: calc(100vh - 120px); border: none;"
                }
            }
            style { "
                .container {{
                    padding: 1rem;
                    display: flex;
                    flex-direction: column;
                    height: 100vh;
                }}
                .toolbar {{
                    display: flex;
                    gap: 0.5rem;
                    margin-bottom: 1rem;
                    align-items: center;
                }}
                .url-input {{
                    flex: 1;
                    padding: 0.5rem;
                    border: 1px solid #ccc;
                    border-radius: 4px;
                    font-size: 14px;
                }}
                .nav-button {{
                    padding: 0.5rem 1rem;
                    border: 1px solid #ccc;
                    border-radius: 4px;
                    background: white;
                    cursor: pointer;
                    font-size: 14px;
                }}
                .nav-button:hover {{
                    background: #f0f0f0;
                }}
                .content {{
                    flex: 1;
                    overflow: hidden;
                }}
            " }
        }
    }
}

impl CoreBrowser for Browser {
    fn new(settings: BrowserSettings) -> anyhow::Result<Arc<Browser>> {
        println!("Creating new browser instance");
        let browser = Arc::new(Browser {
            settings,
            current_url: String::from("https://www.example.com"),
        });
        println!("Browser instance created");
        Ok(browser)
    }

    fn run(&self) -> anyhow::Result<()> {
        println!("Starting browser application");
        let config = Config::default()
            .with_window(WindowBuilder::default()
                .with_title("Projekt-Ora Browser")
                .with_resizable(true)
                .with_inner_size(LogicalSize::new(1024.0, 768.0)));

        println!("Launching Dioxus application");
        dioxus_desktop::launch_cfg(app, config);
        println!("Dioxus application launched");
        Ok(())
    }

    fn navigate(&self, url: &str) -> anyhow::Result<()> {
        println!("Navigating to: {}", url);
        Ok(())
    }

    fn reload(&self) -> anyhow::Result<()> {
        // Reload wird über Dioxus-Komponenten gehandelt
        Ok(())
    }

    fn stop(&self) -> anyhow::Result<()> {
        // Stop wird über Dioxus-Komponenten gehandelt
        Ok(())
    }

    fn close(&self) -> anyhow::Result<()> {
        // Close wird über Dioxus-Window gehandelt
        Ok(())
    }

    fn settings(&self) -> BrowserSettings {
        self.settings.clone()
    }

    fn on_event(&self, _event: BrowserEvent) {
        // Events werden über Dioxus-State und Hooks gehandelt
    }
}
