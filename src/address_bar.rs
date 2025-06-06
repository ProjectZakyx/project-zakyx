use std::sync::Arc;
use tokio::sync::Mutex;
use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2;
use anyhow::Result;
use windows::core::HSTRING;

pub struct AddressBar {
    webview: Arc<Mutex<ICoreWebView2>>,
}

impl AddressBar {
    pub fn new(webview: Arc<Mutex<ICoreWebView2>>) -> Self {
        Self { webview }
    }

    pub async fn navigate_to(&mut self, url: &str) -> Result<()> {
        let url = if !url.starts_with("http://") && !url.starts_with("https://") {
            format!("https://{}", url)
        } else {
            url.to_string()
        };

        let webview = self.webview.lock().await;
        unsafe {
            let url_hstring = HSTRING::from(url);
            webview.Navigate(&url_hstring)?;
        }
        Ok(())
    }
}