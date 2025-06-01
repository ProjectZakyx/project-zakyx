mod webview;

use anyhow::Result;
use webview::WebViewApp;
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, MSG, TranslateMessage};

fn main() -> Result<()> {
    let mut app = WebViewApp::new()?;
    app.initialize()?;
    app.navigate_to("https://www.google.com")?;

    let mut message = MSG::default();
    unsafe {
        while GetMessageW(&mut message, None, 0, 0).into() {
            let _ = TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }

    Ok(())
}