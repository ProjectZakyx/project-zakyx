mod webview;

use anyhow::Result;
use webview::WebViewApp;

fn main() -> Result<()> {
    let mut app = WebViewApp::new()?;
    app.initialize()?;
    app.set_window_title("Projekt Ora")?;
    app.navigate_to("https://www.google.com")?;
    app.run_message_loop()?;
    Ok(())
}
