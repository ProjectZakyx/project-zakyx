mod webview;

use anyhow::Result;
use webview::WebViewApp;

#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, MSG, TranslateMessage};

fn main() -> Result<()> {
    let mut app = WebViewApp::new()?;
    app.initialize()?;
    app.navigate_to("https://www.google.com")?;

    // Platform-specific event loop
    #[cfg(windows)]
    {
        let mut message = MSG::default();
        unsafe {
            while GetMessageW(&mut message, None, 0, 0).into() {
                let _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        app.run();
    }

    #[cfg(target_os = "macos")]
    {
        // macOS event loop would go here
        println!("macOS support is not fully implemented yet");
    }

    Ok(())
}