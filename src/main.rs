use anyhow::Result;
use windows::core::{w, HSTRING};
use windows::Win32::Foundation::{HWND, HINSTANCE, WPARAM, LPARAM, LRESULT, RECT};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, RegisterClassW,
    WS_CHILD, WS_VISIBLE, WS_OVERLAPPEDWINDOW, WS_VSCROLL,
    WINDOW_EX_STYLE, WNDCLASSW, WINDOW_STYLE,
    GetMessageW, TranslateMessage, DispatchMessageW, PostQuitMessage,
    MSG, WM_KEYDOWN, WM_CLOSE, WM_DESTROY, GetWindowTextW, WM_SIZE, WM_COMMAND,
    CS_HREDRAW, CS_VREDRAW, SendMessageW, DestroyWindow, SetWindowPos, SetWindowTextW, SWP_NOZORDER,
    BS_PUSHBUTTON,
};
// Listbox message constants
const LB_RESETCONTENT: u32 = 0x0184;
const LB_ADDSTRING: u32 = 0x0180;
const LB_GETCURSEL: u32 = 0x0188;
// BS_PUSHBUTTON is 0x00000000L - default button style
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_RETURN, GetFocus};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
use windows::Win32::Graphics::Gdi::{InvalidateRect, UpdateWindow};
use std::ptr::null;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

// WebView2 imports
use webview2_com::Microsoft::Web::WebView2::Win32::{
    ICoreWebView2, ICoreWebView2Controller,
    CreateCoreWebView2EnvironmentWithOptions,
};
use webview2_com::{
    CreateCoreWebView2EnvironmentCompletedHandler,
    CreateCoreWebView2ControllerCompletedHandler,
};

mod address_bar;
mod layout;
mod bookmarks;
mod tab_manager;

use layout::BrowserLayout;
use bookmarks::BookmarkManager;
use tab_manager::TabManager;

static mut MAIN_WINDOW: HWND = HWND(0);
static mut ADDRESS_BAR: HWND = HWND(0);
static mut STATUS_AREA: HWND = HWND(0);
static mut BACK_BUTTON: HWND = HWND(0);
static mut FORWARD_BUTTON: HWND = HWND(0);
static mut REFRESH_BUTTON: HWND = HWND(0);
static mut BOOKMARK_ADD_BUTTON: HWND = HWND(0);
static mut BOOKMARKS_LIST: HWND = HWND(0);
static mut NEW_TAB_BUTTON: HWND = HWND(0);
static mut TAB_BUTTONS: Vec<HWND> = Vec::new();
static mut BOOKMARK_BUTTONS: Vec<HWND> = Vec::new(); // Horizontale Favoriten-Buttons
static mut BOOKMARK_DELETE_BUTTONS: Vec<HWND> = Vec::new(); // "X" Buttons zum einzelnen Löschen
static mut BOOKMARKS_CLEAR_BUTTON: HWND = HWND(0);
static mut BOOKMARKS_MANAGE_BUTTON: HWND = HWND(0);
static mut BOOKMARKS_SEPARATOR: HWND = HWND(0);
static mut WEBVIEW_CONTROLLER: Option<ICoreWebView2Controller> = None;
static mut WEBVIEW_CORE: Option<ICoreWebView2> = None;
static mut WEBVIEW_INITIALIZED: bool = false;
static mut BOOKMARK_MANAGER: Option<BookmarkManager> = None;
static mut TAB_MANAGER: Option<TabManager> = None;
static mut CURRENT_URL: String = String::new();

unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_COMMAND => {
            let notification_code = (wparam.0 >> 16) & 0xFFFF;
            let control_hwnd = HWND(lparam.0 as isize);
            
            println!("🎛️ WM_COMMAND received - Control: {:?}, Notification: {}", control_hwnd, notification_code);
            
            // Handle listbox double-click (LBN_DBLCLK = 2)
            if control_hwnd == BOOKMARKS_LIST && notification_code == 2 {
                println!("📚 Bookmark double-clicked");
                
                // Get selected index
                let selected_index = SendMessageW(
                    BOOKMARKS_LIST,
                    LB_GETCURSEL,
                    WPARAM(0),
                    LPARAM(0),
                ).0 as usize;
                
                if let Some(ref bookmark_manager) = BOOKMARK_MANAGER {
                    if let Some(bookmark) = bookmark_manager.get_bookmark_by_index(selected_index) {
                        println!("🔗 Navigating to bookmark: {} -> {}", bookmark.title, bookmark.url);
                        
                        // Navigate to bookmark URL
                        if WEBVIEW_INITIALIZED && WEBVIEW_CORE.is_some() {
                            if let Some(ref webview) = WEBVIEW_CORE {
                                let url_hstring = HSTRING::from(bookmark.url.clone());
                                match webview.Navigate(&url_hstring) {
                                    Ok(_) => {
                                        println!("✅ Successfully navigating to bookmark: {}", bookmark.url);
                                        CURRENT_URL = bookmark.url.clone();
                                    },
                                    Err(e) => {
                                        println!("❌ Bookmark navigation failed: {:?}", e);
                                    }
                                }
                            }
                        }
                    }
                }
                return LRESULT(0);
            }
            
            // Button click handling
            let button_hwnd = control_hwnd;
            
            if button_hwnd == BACK_BUTTON {
                println!("⬅️ Back button clicked");
                if WEBVIEW_INITIALIZED && WEBVIEW_CORE.is_some() {
                    if let Some(ref webview) = WEBVIEW_CORE {
                        if let Err(e) = webview.GoBack() {
                            println!("❌ Go back failed: {:?}", e);
                        } else {
                            println!("✅ Successfully went back");
                        }
                    }
                }
            } else if button_hwnd == FORWARD_BUTTON {
                println!("➡️ Forward button clicked");
                if WEBVIEW_INITIALIZED && WEBVIEW_CORE.is_some() {
                    if let Some(ref webview) = WEBVIEW_CORE {
                        if let Err(e) = webview.GoForward() {
                            println!("❌ Go forward failed: {:?}", e);
                        } else {
                            println!("✅ Successfully went forward");
                        }
                    }
                }
            } else if button_hwnd == REFRESH_BUTTON {
                println!("🔄 Refresh button clicked");
                if WEBVIEW_INITIALIZED && WEBVIEW_CORE.is_some() {
                    if let Some(ref webview) = WEBVIEW_CORE {
                        if let Err(e) = webview.Reload() {
                            println!("❌ Reload failed: {:?}", e);
                        } else {
                            println!("✅ Successfully reloaded");
                        }
                    }
                }
            } else if button_hwnd == BOOKMARK_ADD_BUTTON {
                println!("⭐ Add bookmark button clicked");
                if let Some(ref mut bookmark_manager) = BOOKMARK_MANAGER {
                    let url = CURRENT_URL.clone();
                    if !url.is_empty() {
                        // Extrahiere Titel aus URL (vereinfacht)
                        let title = if let Some(domain_start) = url.find("://") {
                            if let Some(domain_end) = url[domain_start + 3..].find('/') {
                                url[domain_start + 3..domain_start + 3 + domain_end].to_string()
                            } else {
                                url[domain_start + 3..].to_string()
                            }
                        } else {
                            url.clone()
                        };
                        
                        if let Err(e) = bookmark_manager.add_bookmark(title, url) {
                            println!("❌ Failed to add bookmark: {:?}", e);
                        } else {
                            update_bookmarks_list();
                        }
                    } else {
                        println!("⚠️  No current URL to bookmark");
                    }
                }
            } else if button_hwnd == NEW_TAB_BUTTON {
                println!("📂 New tab button clicked");
                if let Some(ref mut tab_manager) = TAB_MANAGER {
                    match tab_manager.create_tab("https://www.google.com".to_string()) {
                        Ok(tab_id) => {
                            println!("✅ Created new tab #{}", tab_id);
                            recreate_tab_buttons();
                            navigate_to_active_tab();
                        },
                        Err(e) => {
                            println!("❌ Failed to create new tab: {:?}", e);
                        }
                    }
                }
            } else if button_hwnd == BOOKMARKS_CLEAR_BUTTON {
                println!("🗑️ Clear bookmarks button clicked");
                if let Some(ref mut bookmark_manager) = BOOKMARK_MANAGER {
                    bookmark_manager.bookmarks.clear();
                    if let Err(e) = bookmark_manager.save_bookmarks() {
                        println!("❌ Failed to save after clearing: {:?}", e);
                    } else {
                        println!("✅ All bookmarks cleared!");
                        update_bookmarks_list();
                    }
                }
            } else if button_hwnd == BOOKMARKS_MANAGE_BUTTON {
                println!("⚙️ Manage bookmarks button clicked");
                show_bookmarks_info();
            } else {
                // Check if it's a bookmark delete button
                println!("🔍 Checking against {} bookmark delete buttons", BOOKMARK_DELETE_BUTTONS.len());
                for (i, &delete_button_hwnd) in BOOKMARK_DELETE_BUTTONS.iter().enumerate() {
                    println!("🔍 Delete button #{}: {:?} vs clicked {:?}", i + 1, delete_button_hwnd, button_hwnd);
                    if button_hwnd == delete_button_hwnd {
                        println!("❌ Delete bookmark #{} clicked", i + 1);
                        if let Some(ref mut bookmark_manager) = BOOKMARK_MANAGER {
                            if i < bookmark_manager.get_bookmarks().len() {
                                let bookmark_to_delete = bookmark_manager.get_bookmarks()[i].clone();
                                println!("🗑️ Deleting bookmark: {} -> {}", bookmark_to_delete.title, bookmark_to_delete.url);
                                
                                // Remove bookmark from manager
                                bookmark_manager.bookmarks.remove(i);
                                
                                // Save updated bookmarks
                                if let Err(e) = bookmark_manager.save_bookmarks() {
                                    println!("❌ Failed to save after deleting: {:?}", e);
                                } else {
                                    println!("✅ Bookmark deleted and saved!");
                                    update_bookmarks_list();
                                }
                            }
                        }
                        return LRESULT(0);
                    }
                }

                // Check if it's a bookmark button
                println!("🔍 Checking against {} bookmark buttons", BOOKMARK_BUTTONS.len());
                for (i, &bookmark_button_hwnd) in BOOKMARK_BUTTONS.iter().enumerate() {
                    println!("🔍 Bookmark button #{}: {:?} vs clicked {:?}", i + 1, bookmark_button_hwnd, button_hwnd);
                    if button_hwnd == bookmark_button_hwnd {
                        println!("📚 Bookmark #{} clicked", i + 1);
                        if let Some(ref bookmark_manager) = BOOKMARK_MANAGER {
                            if let Some(bookmark) = bookmark_manager.get_bookmark_by_index(i) {
                                println!("🔗 Navigating to bookmark: {} -> {}", bookmark.title, bookmark.url);
                                
                                // Navigate to bookmark URL
                                if WEBVIEW_INITIALIZED && WEBVIEW_CORE.is_some() {
                                    if let Some(ref webview) = WEBVIEW_CORE {
                                        let url_hstring = HSTRING::from(bookmark.url.clone());
                                        match webview.Navigate(&url_hstring) {
                                            Ok(_) => {
                                                println!("✅ Successfully navigating to bookmark: {}", bookmark.url);
                                                CURRENT_URL = bookmark.url.clone();
                                            },
                                            Err(e) => {
                                                println!("❌ Bookmark navigation failed: {:?}", e);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        return LRESULT(0);
                    }
                }
                
                // Check if it's a tab button
                for (i, &tab_button_hwnd) in TAB_BUTTONS.iter().enumerate() {
                    if button_hwnd == tab_button_hwnd {
                        println!("📂 Tab #{} clicked", i + 1);
                        if let Some(ref mut tab_manager) = TAB_MANAGER {
                            let tabs = tab_manager.get_tabs();
                            if i < tabs.len() {
                                let tab_id = tabs[i].id;
                                if let Err(e) = tab_manager.switch_to_tab(tab_id) {
                                    println!("❌ Failed to switch to tab: {:?}", e);
                                } else {
                                    update_tab_buttons();
                                    navigate_to_active_tab();
                                }
                            }
                        }
                        break;
                    }
                }
            }
            LRESULT(0)
        }
        WM_SIZE => {
            // Resize WebView2 when window is resized
            if let Some(ref controller) = WEBVIEW_CONTROLLER {
                let layout = BrowserLayout::new(1024, 768);
                let (x, y, width, height) = layout.webview_dimensions();
                let bounds = RECT { 
                    left: x, 
                    top: y, 
                    right: x + width, 
                    bottom: y + height 
                };
                let _ = controller.SetBounds(bounds);
            }
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
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
    println!("🚀 Starting Ora Integrated Web Browser...");
    
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
    
    // Create navigation label
    let (x, y, width, height) = layout.nav_label_dimensions();
    let _nav_label = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("STATIC"),
            w!("🧭 Navigation:"),
            WS_CHILD | WS_VISIBLE,
            x, y, width, height,
            window_setup.handle,
            None,
            window_setup.instance,
            null(),
        )
    };

    // Create back button
    let (x, y, width, height) = layout.back_button_dimensions();
    let back_button_hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("BUTTON"),
            w!("⬅️ Zurück"),
            WS_CHILD | WS_VISIBLE,
            x, y, width, height,
            window_setup.handle,
            None,
            window_setup.instance,
            null(),
        )
    };

    // Create forward button
    let (x, y, width, height) = layout.forward_button_dimensions();
    let forward_button_hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("BUTTON"),
            w!("➡️ Vor"),
            WS_CHILD | WS_VISIBLE,
            x, y, width, height,
            window_setup.handle,
            None,
            window_setup.instance,
            null(),
        )
    };

    // Create refresh button
    let (x, y, width, height) = layout.refresh_button_dimensions();
    let refresh_button_hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("BUTTON"),
            w!("🔄 Aktualisieren"),
            WS_CHILD | WS_VISIBLE,
            x, y, width, height,
            window_setup.handle,
            None,
            window_setup.instance,
            null(),
        )
    };

    unsafe {
        BACK_BUTTON = back_button_hwnd;
        FORWARD_BUTTON = forward_button_hwnd;
        REFRESH_BUTTON = refresh_button_hwnd;
    }

    println!("✅ Navigation buttons created successfully!");

    // Create bookmark add button
    let (x, y, width, height) = layout.bookmark_add_button_dimensions();
    let bookmark_add_button_hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("BUTTON"),
            w!("⭐ Lesezeichen"),
            WS_CHILD | WS_VISIBLE,
            x, y, width, height,
            window_setup.handle,
            None,
            window_setup.instance,
            null(),
        )
    };

    unsafe {
        BOOKMARK_ADD_BUTTON = bookmark_add_button_hwnd;
    }

    println!("✅ Bookmark button created successfully!");

    // Create address bar label
    let (x, y, width, height) = layout.address_label_dimensions();
    let _address_label = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("STATIC"),
            w!("🌐 URL eingeben und Enter drücken:"),
            WS_CHILD | WS_VISIBLE,
            x, y, width, height,
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

    // Create horizontal favorites area header
    let (x, y, width, height) = layout.bookmarks_header_dimensions();
    let _bookmarks_header = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("STATIC"),
            w!("⭐ Horizontale Favoritenleiste:"),
            WS_CHILD | WS_VISIBLE,
            x, y, width, height,
            window_setup.handle,
            None,
            window_setup.instance,
            null(),
        )
    };

    // Create bookmarks management buttons (horizontal layout)
    let (x, y, width, height) = layout.clear_bookmarks_button_dimensions();
    let bookmarks_clear_button_hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("BUTTON"),
            w!("🗑️ Löschen"),
            WS_CHILD | WS_VISIBLE,
            x, y, width, height,
            window_setup.handle,
            None,
            window_setup.instance,
            null(),
        )
    };

    let (x, y, width, height) = layout.info_bookmarks_button_dimensions();
    let bookmarks_manage_button_hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("BUTTON"),
            w!("ℹ️ Info"),
            WS_CHILD | WS_VISIBLE,
            x, y, width, height,
            window_setup.handle,
            None,
            window_setup.instance,
            null(),
        )
    };

    unsafe {
        BOOKMARKS_CLEAR_BUTTON = bookmarks_clear_button_hwnd;
        BOOKMARKS_MANAGE_BUTTON = bookmarks_manage_button_hwnd;
    }

    println!("✅ Horizontal favorites area created successfully!");

    // Initialize managers
    unsafe {
        BOOKMARK_MANAGER = Some(BookmarkManager::new());
        TAB_MANAGER = Some(TabManager::new());
    }
    println!("✅ Bookmark manager initialized!");
    println!("✅ Tab manager initialized!");
    
    // Create initial tab
    unsafe {
        if let Some(ref mut tab_manager) = TAB_MANAGER {
            if let Ok(_) = tab_manager.create_tab("https://www.google.com".to_string()) {
                println!("✅ Initial tab created!");
            }
        }
    }
    
    // Create tab bar UI
    create_tab_bar_ui(&window_setup, &layout);
    
    // Load existing bookmarks into the list
    update_bookmarks_list();

    // Create status area (will be replaced by WebView2)
    let (x, y, width, height) = layout.webview_dimensions();
    
    let status_hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("STATIC"),
            w!("🔄 Initialisiere WebView2..."),
            WS_CHILD | WS_VISIBLE,
            x, y, width, height,
            window_setup.handle,
            None,
            window_setup.instance,
            null(),
        )
    };
    
    unsafe { STATUS_AREA = status_hwnd; }
    
    // Initialize WebView2
    println!("📦 Initializing WebView2...");
    if initialize_webview2(window_setup.handle, x, y, width, height).is_ok() {
        println!("✅ WebView2 initialization started successfully!");
        update_status("✅ WebView2 wird geladen...\r\n\r\n🔄 Browser-Engine startet\r\n⏳ Bitte warten...");
    } else {
        println!("❌ WebView2 initialization failed!");
        update_status("❌ WebView2-Initialisierung fehlgeschlagen!\r\n\r\n📋 Möglicherweise ist WebView2 Runtime nicht installiert.\r\n💡 Installieren Sie WebView2 Runtime von Microsoft.");
    }

    println!("🚀 Browser ready! WebView2 loading...");

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
                        continue;
                    }
                    
                    // Process URL
                    println!("🌐 Processing URL: '{}'", url);
                    
                    let final_url = if !url.starts_with("http://") && !url.starts_with("https://") {
                        format!("https://{}", url)
                    } else {
                        url.clone()
                    };
                    
                    println!("🔗 Navigating to: '{}'", final_url);
                    
                    // Navigate WebView2
                    unsafe {
                        if WEBVIEW_INITIALIZED && WEBVIEW_CORE.is_some() {
                            if let Some(ref webview) = WEBVIEW_CORE {
                                let url_hstring = HSTRING::from(final_url.clone());
                                match webview.Navigate(&url_hstring) {
                                    Ok(_) => {
                                        println!("✅ Successfully navigating to: {}", final_url);
                                        CURRENT_URL = final_url.clone(); // Track current URL
                                        
                                        // Update active tab URL
                                        if let Some(ref mut tab_manager) = TAB_MANAGER {
                                            tab_manager.update_active_tab_url(final_url);
                                            update_tab_buttons();
                                        }
                                    },
                                    Err(e) => {
                                        println!("❌ Navigation failed: {:?}", e);
                                    }
                                }
                            }
                        } else {
                            println!("⚠️  WebView2 not ready yet - try again in a moment");
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

    println!("👋 Browser exiting cleanly...");
    Ok(())
}

fn initialize_webview2(parent_hwnd: HWND, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
    println!("📦 Creating WebView2 environment...");

    let env_handler = CreateCoreWebView2EnvironmentCompletedHandler::create(Box::new(move |result, env| {
        if result.is_err() || env.is_none() {
            println!("❌ Environment creation failed: {:?}", result);
            return Ok(());
        }

        let environment = env.unwrap();
        println!("✅ WebView2 environment created!");

        let ctrl_handler = CreateCoreWebView2ControllerCompletedHandler::create(Box::new(move |result, ctrl| {
            if result.is_err() || ctrl.is_none() {
                println!("❌ Controller creation failed: {:?}", result);
                return Ok(());
            }

            let controller = ctrl.unwrap();
            println!("✅ WebView2 controller created!");

            // Set bounds
            let bounds = RECT { left: x, top: y, right: x + width, bottom: y + height };
            if let Err(e) = unsafe { controller.SetBounds(bounds) } {
                println!("❌ Failed to set bounds: {:?}", e);
                return Ok(());
            }

            // Get core WebView
            match unsafe { controller.CoreWebView2() } {
                Ok(webview) => {
                    println!("✅ WebView2 core obtained!");
                    
                    unsafe {
                        WEBVIEW_CONTROLLER = Some(controller);
                        WEBVIEW_CORE = Some(webview);
                        WEBVIEW_INITIALIZED = true;
                        
                        // Hide status area and show WebView
                        if STATUS_AREA.0 != 0 {
                            windows::Win32::UI::WindowsAndMessaging::ShowWindow(STATUS_AREA, windows::Win32::UI::WindowsAndMessaging::SW_HIDE);
                        }
                    }
                    
                    // Navigate to default page
                    if let Some(ref webview) = unsafe { &WEBVIEW_CORE } {
                        let url = HSTRING::from("https://www.google.com");
                        unsafe {
                            if let Err(e) = webview.Navigate(&url) {
                                println!("❌ Failed to navigate to default page: {:?}", e);
                            } else {
                                println!("✅ Navigated to Google successfully!");
                                CURRENT_URL = "https://www.google.com".to_string(); // Track initial URL
                            }
                        }
                    }
                },
                Err(e) => {
                    println!("❌ Failed to get core WebView: {:?}", e);
                }
            }

            Ok(())
        }));

        if let Err(e) = unsafe { environment.CreateCoreWebView2Controller(parent_hwnd, &ctrl_handler) } {
            println!("❌ Failed to create controller: {:?}", e);
        }

        Ok(())
    }));

    unsafe {
        CreateCoreWebView2EnvironmentWithOptions(None, None, None, &env_handler)?;
    }

    Ok(())
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

fn update_bookmarks_list() {
    update_horizontal_bookmarks();
}

fn update_horizontal_bookmarks() {
    unsafe {
        if let Some(window_setup) = get_window_setup() {
            let layout = BrowserLayout::new(1024, 768);
            recreate_bookmark_buttons(&window_setup, &layout);
        }
    }
}

fn recreate_bookmark_buttons(window_setup: &WindowSetup, layout: &BrowserLayout) {
    unsafe {
        // Destroy existing bookmark buttons
        for &bookmark_button_hwnd in &BOOKMARK_BUTTONS {
            if bookmark_button_hwnd.0 != 0 {
                DestroyWindow(bookmark_button_hwnd);
            }
        }
        BOOKMARK_BUTTONS.clear();
        
        // Destroy existing delete buttons
        for &delete_button_hwnd in &BOOKMARK_DELETE_BUTTONS {
            if delete_button_hwnd.0 != 0 {
                DestroyWindow(delete_button_hwnd);
            }
        }
        BOOKMARK_DELETE_BUTTONS.clear();
        
        if let Some(ref bookmark_manager) = BOOKMARK_MANAGER {
            let bookmarks = bookmark_manager.get_bookmarks();
            let bookmarks_per_row = 6; // 6 Favoriten pro Reihe
            
            for (i, bookmark) in bookmarks.iter().enumerate() {
                let (x, y, width, height) = layout.bookmark_button_dimensions(i, bookmarks_per_row);
                
                // Kürze den Titel falls zu lang (schmaler wegen X Button)
                let display_title = if bookmark.title.len() > 12 {
                    format!("{}...", &bookmark.title[..9])
                } else {
                    bookmark.title.clone()
                };
                
                let text_wide: Vec<u16> = display_title.encode_utf16().chain(Some(0)).collect();
                
                let bookmark_button_hwnd = CreateWindowExW(
                    WINDOW_EX_STYLE::default(),
                    w!("BUTTON"),
                    windows::core::PCWSTR(text_wide.as_ptr()),
                    WS_CHILD | WS_VISIBLE,
                    x, y, width, height,
                    window_setup.handle,
                    None,
                    window_setup.instance,
                    null(),
                );
                
                println!("📚 Created bookmark button #{}: '{}' at ({}, {}) with HWND: {:?}", 
                         i + 1, display_title, x, y, bookmark_button_hwnd);
                
                BOOKMARK_BUTTONS.push(bookmark_button_hwnd);
                
                // Create delete button
                let (del_x, del_y, del_width, del_height) = layout.bookmark_delete_button_dimensions(i, bookmarks_per_row);
                
                let delete_button_hwnd = CreateWindowExW(
                    WINDOW_EX_STYLE::default(),
                    w!("BUTTON"),
                    w!("❌"),
                    WS_CHILD | WS_VISIBLE,
                    del_x, del_y, del_width, del_height,
                    window_setup.handle,
                    None,
                    window_setup.instance,
                    null(),
                );
                
                println!("❌ Created delete button #{}: at ({}, {}) with HWND: {:?}", 
                         i + 1, del_x, del_y, delete_button_hwnd);
                
                BOOKMARK_DELETE_BUTTONS.push(delete_button_hwnd);
            }
        }
        
        println!("📚 Horizontal bookmarks updated ({} bookmark buttons, {} delete buttons)", 
                 BOOKMARK_BUTTONS.len(), BOOKMARK_DELETE_BUTTONS.len());
    }
}

fn create_tab_bar_ui(window_setup: &WindowSetup, layout: &BrowserLayout) {
    unsafe {
        // Create new tab button
        let tab_count = if let Some(ref tab_manager) = TAB_MANAGER {
            tab_manager.get_tab_count()
        } else {
            0
        };
        
        let (x, y, width, height) = layout.new_tab_button_dimensions(tab_count);
        let new_tab_button_hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("BUTTON"),
            w!("+"),
            WS_CHILD | WS_VISIBLE,
            x, y, width, height,
            window_setup.handle,
            None,
            window_setup.instance,
            null(),
        );
        
        NEW_TAB_BUTTON = new_tab_button_hwnd;
        println!("✅ New tab button created!");
        
        // Create initial tab buttons
        recreate_tab_buttons_impl(window_setup, layout);
    }
}

fn recreate_tab_buttons() {
    unsafe {
        if let Some(window_setup) = get_window_setup() {
            let layout = BrowserLayout::new(1024, 768);
            recreate_tab_buttons_impl(&window_setup, &layout);
        }
    }
}

fn recreate_tab_buttons_impl(window_setup: &WindowSetup, layout: &BrowserLayout) {
    unsafe {
        // Destroy existing tab buttons
        for &tab_button_hwnd in &TAB_BUTTONS {
            if tab_button_hwnd.0 != 0 {
                DestroyWindow(tab_button_hwnd);
            }
        }
        TAB_BUTTONS.clear();
        
        if let Some(ref tab_manager) = TAB_MANAGER {
            let tabs = tab_manager.get_tabs();
            
            for (i, tab) in tabs.iter().enumerate() {
                let (x, y, width, height) = layout.tab_button_dimensions(i, tabs.len());
                let button_text = if tab.is_active {
                    format!("● {}", tab.get_display_title())
                } else {
                    tab.get_display_title()
                };
                
                let text_wide: Vec<u16> = button_text.encode_utf16().chain(Some(0)).collect();
                
                let tab_button_hwnd = CreateWindowExW(
                    WINDOW_EX_STYLE::default(),
                    w!("BUTTON"),
                    windows::core::PCWSTR(text_wide.as_ptr()),
                    WS_CHILD | WS_VISIBLE,
                    x, y, width, height,
                    window_setup.handle,
                    None,
                    window_setup.instance,
                    null(),
                );
                
                TAB_BUTTONS.push(tab_button_hwnd);
            }
            
            // Update new tab button position
            let (x, y, width, height) = layout.new_tab_button_dimensions(tabs.len());
            SetWindowPos(
                NEW_TAB_BUTTON,
                None,
                x, y, width, height,
                SWP_NOZORDER,
            );
        }
        
        println!("📂 Tab buttons recreated");
    }
}

fn update_tab_buttons() {
    unsafe {
        if let Some(ref tab_manager) = TAB_MANAGER {
            let tabs = tab_manager.get_tabs();
            
            for (i, tab) in tabs.iter().enumerate() {
                if i < TAB_BUTTONS.len() {
                    let button_text = if tab.is_active {
                        format!("● {}", tab.get_display_title())
                    } else {
                        tab.get_display_title()
                    };
                    
                    let text_wide: Vec<u16> = button_text.encode_utf16().chain(Some(0)).collect();
                    
                    SetWindowTextW(
                        TAB_BUTTONS[i],
                        windows::core::PCWSTR(text_wide.as_ptr())
                    );
                }
            }
        }
        
        println!("📂 Tab buttons updated");
    }
}

fn navigate_to_active_tab() {
    unsafe {
        if let Some(ref tab_manager) = TAB_MANAGER {
            if let Some(active_tab) = tab_manager.get_active_tab() {
                println!("🔄 Navigating to active tab: {}", active_tab.url);
                
                if WEBVIEW_INITIALIZED && WEBVIEW_CORE.is_some() {
                    if let Some(ref webview) = WEBVIEW_CORE {
                        let url_hstring = HSTRING::from(active_tab.url.clone());
                        match webview.Navigate(&url_hstring) {
                            Ok(_) => {
                                println!("✅ Successfully navigated to tab URL: {}", active_tab.url);
                                CURRENT_URL = active_tab.url.clone();
                            },
                            Err(e) => {
                                println!("❌ Tab navigation failed: {:?}", e);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn get_window_setup() -> Option<WindowSetup> {
    unsafe {
        if MAIN_WINDOW.0 != 0 {
            if let Ok(instance) = windows::Win32::System::LibraryLoader::GetModuleHandleW(None) {
                return Some(WindowSetup { 
                    handle: MAIN_WINDOW, 
                    instance: HINSTANCE(instance.0)
                });
            }
        }
    }
    None
}

fn show_bookmarks_info() {
    unsafe {
        if let Some(ref bookmark_manager) = BOOKMARK_MANAGER {
            let count = bookmark_manager.get_bookmarks().len();
            println!("ℹ️ Favoritenleiste Info:");
            println!("   📊 Anzahl Lesezeichen: {}", count);
            println!("   📂 Datei: bookmarks.json");
            println!("   📋 Layout: Horizontal (6 pro Reihe)");
            println!("   🎛️ Buttons erstellt: {}", BOOKMARK_BUTTONS.len());
            
            if count > 0 {
                println!("   📝 Lesezeichen:");
                for (i, bookmark) in bookmark_manager.get_bookmarks().iter().enumerate() {
                    println!("      {}. {} -> {}", i + 1, bookmark.title, bookmark.url);
                }
            } else {
                println!("   📭 Keine Lesezeichen vorhanden");
            }
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
            let class_name = w!("OraIntegratedBrowserClass");
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
                w!("🌐 Ora Web Browser - Integriert"),
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