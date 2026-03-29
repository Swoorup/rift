use objc2_app_kit::NSNormalWindowLevel;

use crate::common::collections::HashMap;
use crate::sys::window_server::{WindowServerId, WindowServerInfo, window_is_sticky, window_level};

/// Computes whether a window is manageable based on its properties and window server information.
///
/// A window is manageable if:
/// - It is not minimized
/// - Its layer is 0 (if info available)
/// - It is not sticky
/// - Its level is normal (if available)
/// - It is an AX window (role == AXWindow)
/// - If it has the standard subrole, it must also be an AX root (no window server parent).
///   Non-standard subrole windows (e.g. Java/Swing apps) are exempt from the root check
///   since their toolkits often create parent container windows.
pub fn compute_window_manageability(
    window_server_id: Option<WindowServerId>,
    is_minimized: bool,
    is_ax_window: bool,
    is_ax_root: bool,
    is_ax_standard: bool,
    window_server_info: &HashMap<WindowServerId, WindowServerInfo>,
) -> bool {
    if is_minimized {
        return false;
    }

    if let Some(wsid) = window_server_id {
        if let Some(info) = window_server_info.get(&wsid) {
            if info.layer != 0 {
                return false;
            }
        }
        if window_is_sticky(wsid) {
            return false;
        }

        if let Some(level) = window_level(wsid.0) {
            if level != NSNormalWindowLevel {
                return false;
            }
        }
    }
    is_ax_window && (is_ax_root || !is_ax_standard)
}
