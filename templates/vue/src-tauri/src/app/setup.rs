use tauri::{App, Manager};

#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;

use tauri_plugin_window_state::{StateFlags, WindowExt};
use window_vibrancy::{apply_mica, apply_vibrancy, NSVisualEffectMaterial};

pub fn init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(window) = app.get_webview_window("main") {
        // restore the window state
        window
            .restore_state(StateFlags::all())
            .expect("Failed to restore window state");

        // customize the window
        #[cfg(target_os = "macos")]
        {
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("unsupported platform! 'apply_vibrancy' is only supported on macos");
            use cocoa::appkit::{NSColor, NSWindow};
            use cocoa::base::{id, nil};
            let ns_window = window.ns_window().unwrap() as id;
            unsafe {
                let bg_color = NSColor::colorWithRed_green_blue_alpha_(
                    nil,
                    50.0 / 255.0,
                    158.0 / 255.0,
                    163.5 / 255.0,
                    1.0,
                );
                ns_window.setBackgroundColor_(bg_color);
            }
        }
        #[cfg(target_os = "windows")]
        {
            apply_mica(&window, None)
                .expect("unsupported platform! 'apply_mica' is only supported on windows 11");
        }
    }
    Ok(())
}
