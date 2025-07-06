// 🖥️ FULLSCREEN SYSTEM - Fullscreen Manager
use anyhow::Result;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::WindowsAndMessaging::*;

pub struct FullscreenManager {
    is_fullscreen: bool,
    original_style: u32,
    original_rect: (i32, i32, i32, i32),
}

impl FullscreenManager {
    pub fn new() -> Self {
        FullscreenManager {
            is_fullscreen: false,
            original_style: 0,
            original_rect: (0, 0, 0, 0),
        }
    }

    pub fn toggle_fullscreen(&mut self, hwnd: HWND) -> Result<bool> {
        unsafe {
            if self.is_fullscreen {
                // Fullscreen verlassen
                let _ = SetWindowLongW(hwnd, GWL_STYLE, self.original_style as i32);
                let _ = SetWindowPos(
                    hwnd,
                    None,
                    self.original_rect.0,
                    self.original_rect.1,
                    self.original_rect.2 - self.original_rect.0,
                    self.original_rect.3 - self.original_rect.1,
                    SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED,
                );

                self.is_fullscreen = false;
                println!("🖥️ Fullscreen disabled");
            } else {
                // Fullscreen aktivieren
                self.original_style = GetWindowLongW(hwnd, GWL_STYLE) as u32;

                let mut rect = windows::Win32::Foundation::RECT::default();
                let _ = GetWindowRect(hwnd, &mut rect);
                self.original_rect = (rect.left, rect.top, rect.right, rect.bottom);

                let _ = SetWindowLongW(
                    hwnd,
                    GWL_STYLE,
                    (WS_OVERLAPPEDWINDOW.0 & !(WS_CAPTION.0 | WS_THICKFRAME.0)) as i32,
                );

                let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY);
                let mut monitor_info = MONITORINFO {
                    cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                    ..Default::default()
                };

                if GetMonitorInfoW(monitor, &mut monitor_info).as_bool() {
                    let _ = SetWindowPos(
                        hwnd,
                        None,
                        monitor_info.rcMonitor.left,
                        monitor_info.rcMonitor.top,
                        monitor_info.rcMonitor.right - monitor_info.rcMonitor.left,
                        monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top,
                        SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED,
                    );
                }

                self.is_fullscreen = true;
                println!("🖥️ Fullscreen enabled");
            }
        }

        Ok(self.is_fullscreen)
    }

    pub fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }
} 