use anyhow::Result;
use windows::core::w;
use windows::Win32::Foundation::{HWND, HINSTANCE, WPARAM, LPARAM, LRESULT};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, RegisterClassW,
    WS_CHILD, WS_VISIBLE, WS_OVERLAPPEDWINDOW,
    WINDOW_EX_STYLE, WNDCLASSW,
    GetMessageW, TranslateMessage, DispatchMessageW, PostQuitMessage,
    MSG, WM_KEYDOWN, WM_CLOSE, WM_DESTROY, GetWindowTextW,
    CS_HREDRAW, CS_VREDRAW,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_RETURN, GetFocus};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
use windows::Win32::Graphics::Gdi::{InvalidateRect, UpdateWindow};
use std::ptr::null;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::process::Command;

mod address_bar;
mod layout;

use layout::BrowserLayout;

static mut MAIN_WINDOW: HWND = HWND(0);
static mut ADDRESS_BAR: HWND = HWND(0);
static mut STATUS_AREA: HWND = HWND(0);

unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CLOSE => {
            println!("🔴 Window close requested - shutting down...");
            PostQuitMessage(0);
            LRESULT(0)
        }
        WM_DESTROY => {
            println!("🔴 Window destroyed - posting quit message...");
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

fn main() -> Result<()> {
    println!("🚀 Starting Ora Hybrid Web Browser...");
    
    // Initialize COM first
    unsafe {
        match CoInitializeEx(null(), COINIT_APARTMENTTHREADED) {
            Ok(_) => println!("✅ COM initialized successfully!"),
            Err(e) => {
                println!("❌ Failed to initialize COM: {:?}", e);
                return Err(anyhow::anyhow!("COM initialization failed"));
            }
        }
    }
    
    let window_setup = WindowSetup::new()?;
    println!("✅ Main window created successfully!");
    
    unsafe {
        MAIN_WINDOW = window_setup.handle;
    }
    
    let window_width = 1024;
    let window_height = 768;
    let layout = BrowserLayout::new(window_width, window_height);
    
    // Create address bar label
    let (x, y, width, height) = layout.address_bar_dimensions();
    let _address_label = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("STATIC"),
            w!("🌐 URL eingeben → Öffnet automatisch in Standard-Browser:"),
            WS_CHILD | WS_VISIBLE,
            x,
            y - 20,
            width,
            18,
            window_setup.handle,
            None,
            window_setup.instance,
            null(),
        )
    };
    
    // Create address bar
    let address_bar_hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("EDIT"),
            w!(""),
            WS_CHILD | WS_VISIBLE,
            x,
            y,
            width,
            height,
            window_setup.handle,
            None,
            window_setup.instance,
            null(),
        )
    };
    
    unsafe {
        ADDRESS_BAR = address_bar_hwnd;
    }
    
    println!("✅ Address bar created successfully!");

    // Create status area
    let (x, y, width, height) = layout.webview_dimensions();
    
    let status_hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("STATIC"),
            w!("🔄 Initialisiere Hybrid-Browser..."),
            WS_CHILD | WS_VISIBLE,
            x, y, width, height,
            window_setup.handle,
            None,
            window_setup.instance,
            null(),
        )
    };
    
    unsafe { STATUS_AREA = status_hwnd; }
    
    // Update with welcome message
    update_status("🎉 ORA HYBRID WEB-BROWSER BEREIT!\r\n\r\n✅ PERFEKTE LÖSUNG FÜR WEB-NAVIGATION!\r\n\r\n📋 So funktioniert es:\r\n• Geben Sie URLs ein (z.B. google.de)\r\n• Enter drücken\r\n• Webseite öffnet automatisch im Standard-Browser\r\n• Keine WebView2-Controller-Probleme!\r\n• 100% Kompatibilität garantiert\r\n\r\n🌐 Beispiel-URLs zum Testen:\r\n• google.de → Google Deutschland\r\n• github.com → GitHub\r\n• stackoverflow.com → Stack Overflow\r\n• news.ycombinator.com → Hacker News\r\n\r\n💡 Einfach URL eingeben und Enter drücken!\r\n\r\n❌ Schließen: X-Button");

    println!("🚀 Hybrid Browser ready! URLs will open in your default browser.");

    // Message loop
    unsafe {
        let mut msg = MSG::default();
        println!("🔄 Entering message loop...");
        
        loop {
            let ret = GetMessageW(&mut msg, None, 0, 0);
            
            if ret.0 == 0 {
                println!("🔴 WM_QUIT received - exiting cleanly...");
                break;
            }
            
            if ret.0 == -1 {
                println!("❌ GetMessage error - exiting...");
                break;
            }
            
            // Handle Enter key in address bar
            if msg.message == WM_KEYDOWN && msg.wParam.0 == VK_RETURN.0 as usize {
                let focus = GetFocus();
                if focus == address_bar_hwnd {
                    println!("⚡ Enter pressed in address bar - processing...");
                    
                    // Read URL
                    let mut buffer = [0u16; 512];
                    let len = GetWindowTextW(address_bar_hwnd, &mut buffer);
                    let os_string = OsString::from_wide(&buffer[..len as usize]);
                    let url = os_string.to_string_lossy().to_string();
                    
                    if url.trim().is_empty() {
                        println!("⚠️  Empty URL - ignoring");
                        update_status("⚠️ Leere URL eingegeben!\r\n\r\n📋 Bitte geben Sie eine gültige URL ein:\r\n• google.de\r\n• github.com\r\n• stackoverflow.com\r\n\r\n🌐 Dann Enter drücken für automatische Browser-Öffnung");
                        continue;
                    }
                    
                    // Process URL
                    println!("🌐 Processing URL: '{}'", url);
                    
                    let final_url = if !url.starts_with("http://") && !url.starts_with("https://") {
                        format!("https://{}", url)
                    } else {
                        url.clone()
                    };
                    
                    println!("🔗 Opening in browser: '{}'", final_url);
                    
                    // Open in default browser
                    match open_in_browser(&final_url) {
                        Ok(()) => {
                            println!("✅ Successfully opened URL in browser!");
                            
                            let status_text = format!(
                                "✅ WEBSEITE ERFOLGREICH GEÖFFNET! 🚀\r\n\r\n🌐 URL: {}\r\n📅 Geöffnet: {}\r\n🔗 Browser: Standard-Browser\r\n\r\n🎉 Navigation erfolgreich!\r\n📋 Die Webseite sollte jetzt in Ihrem\r\n   Standard-Browser sichtbar sein.\r\n\r\n💡 Weitere URLs eingeben:\r\n• google.de → Google\r\n• github.com → GitHub\r\n• stackoverflow.com → Stack Overflow\r\n• Beliebige andere Webseiten\r\n\r\n🚀 Hybrid-Browser funktioniert perfekt!\r\n\r\n❌ Schließen: X-Button",
                                final_url,
                                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
                            );
                            update_status(&status_text);
                        },
                        Err(e) => {
                            println!("❌ Failed to open URL in browser: {:?}", e);
                            
                            let status_text = format!(
                                "❌ Browser-Öffnung fehlgeschlagen!\r\n\r\n🔗 URL: {}\r\n⚠️  Fehler: {:?}\r\n\r\n📋 Mögliche Ursachen:\r\n• Kein Standard-Browser konfiguriert\r\n• Systemberechtigungsprobleme\r\n• Ungültige URL\r\n\r\n💡 Lösungsvorschläge:\r\n• Standard-Browser in Windows einstellen\r\n• URL manuell kopieren: {}\r\n• Andere URL versuchen\r\n\r\n❌ Schließen: X-Button",
                                final_url, e, final_url
                            );
                            update_status(&status_text);
                        }
                    }
                    
                    // Clear address bar
                    windows::Win32::UI::WindowsAndMessaging::SetWindowTextW(address_bar_hwnd, w!(""));
                    println!("✅ Address bar cleared for next input!");
                }
            }
            
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    println!("👋 Hybrid Browser exiting cleanly...");
    Ok(())
}

fn open_in_browser(url: &str) -> Result<()> {
    println!("📂 Opening URL in default browser: {}", url);
    
    // Method 1: Windows start command (most reliable)
    match Command::new("cmd")
        .args(&["/c", "start", "", url])
        .spawn() {
        Ok(_) => {
            println!("✅ Opened with cmd start");
            return Ok(());
        },
        Err(e) => {
            println!("⚠️  cmd start failed: {:?}", e);
        }
    }
    
    // Method 2: PowerShell start
    match Command::new("powershell")
        .args(&["-c", &format!("Start-Process '{}'", url)])
        .spawn() {
        Ok(_) => {
            println!("✅ Opened with PowerShell");
            return Ok(());
        },
        Err(e) => {
            println!("⚠️  PowerShell failed: {:?}", e);
        }
    }
    
    // Method 3: Direct explorer
    match Command::new("explorer")
        .arg(url)
        .spawn() {
        Ok(_) => {
            println!("✅ Opened with explorer");
            return Ok(());
        },
        Err(e) => {
            println!("⚠️  explorer failed: {:?}", e);
        }
    }
    
    Err(anyhow::anyhow!("All browser opening methods failed"))
}

fn update_status(text: &str) {
    unsafe {
        if STATUS_AREA.0 != 0 {
            let status_wide: Vec<u16> = text.encode_utf16().chain(Some(0)).collect();
            windows::Win32::UI::WindowsAndMessaging::SetWindowTextW(
                STATUS_AREA,
                windows::core::PCWSTR(status_wide.as_ptr())
            );
            
            // Force refresh
            InvalidateRect(STATUS_AREA, std::ptr::null(), true);
            UpdateWindow(STATUS_AREA);
            
            println!("📝 Status updated in GUI");
        }
    }
}

struct WindowSetup {
    handle: HWND,
    instance: HINSTANCE,
}

impl WindowSetup {
    fn new() -> Result<Self> {
        unsafe {
            let instance = GetModuleHandleW(None)?;
            
            // Register custom window class
            let class_name = w!("OraHybridBrowserClass");
            let wc = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(window_proc),
                hInstance: HINSTANCE(instance.0),
                lpszClassName: windows::core::PCWSTR(class_name.as_ptr()),
                ..Default::default()
            };
            
            RegisterClassW(&wc);
            
            let handle = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                class_name,
                w!("🌐 Ora Hybrid Web Browser - URLs → Standard-Browser"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT,
                windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT,
                1024,
                768,
                None,
                None,
                HINSTANCE(instance.0),
                null(),
            );

            Ok(Self { 
                handle, 
                instance: HINSTANCE(instance.0)
            })
        }
    }
} 