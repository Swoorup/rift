use objc2_app_kit::NSNormalWindowLevel;

use crate::common::collections::HashMap;
use crate::sys::app::WindowInfo;
use crate::sys::window_server::{WindowServerId, WindowServerInfo, window_is_sticky, window_level};

/// Computes whether a window is manageable based on its properties and window server information.
///
/// A window is manageable if:
/// - It is not minimized
/// - Its layer/level is normal (if info available), except for known IBKR Desktop AX dialogs
/// - It is not sticky
/// - It is an AX window (role == AXWindow)
/// - It is an AX root (no visible window server parent)
pub fn compute_window_info_manageability(
    info: &WindowInfo,
    window_server_info: &HashMap<WindowServerId, WindowServerInfo>,
) -> bool {
    compute_window_manageability(
        info.sys_id,
        info.is_minimized,
        info.is_ax_window,
        info.is_root,
        is_ibkr_desktop_dialog(info),
        window_server_info,
    )
}

fn compute_window_manageability(
    window_server_id: Option<WindowServerId>,
    is_minimized: bool,
    is_ax_window: bool,
    is_ax_root: bool,
    allow_non_normal_layer: bool,
    window_server_info: &HashMap<WindowServerId, WindowServerInfo>,
) -> bool {
    if is_minimized {
        return false;
    }

    if let Some(wsid) = window_server_id {
        if let Some(info) = window_server_info.get(&wsid) {
            if info.layer != 0 && !allow_non_normal_layer {
                return false;
            }
        }
        if window_is_sticky(wsid) {
            return false;
        }

        if let Some(level) = window_level(wsid.0) {
            if level != NSNormalWindowLevel && !allow_non_normal_layer {
                return false;
            }
        }
    }
    is_ax_window && is_ax_root
}

fn is_ibkr_desktop_dialog(info: &WindowInfo) -> bool {
    if info.ax_role.as_deref() != Some("AXWindow")
        || info.ax_subrole.as_deref() != Some("AXDialog")
    {
        return false;
    }

    let path_matches = info.path.as_ref().and_then(|path| path.to_str()).is_some_and(|path| {
        let path = path.to_ascii_lowercase();
        path.contains("ibkr%20desktop") || path.contains("ibkr desktop")
    });

    let bundle_matches = info.bundle_id.as_deref().is_some_and(|bundle_id| {
        bundle_id.starts_with("com.install4j.") || bundle_id == "com.azul.zulu.java"
    });

    let max_size = info.max_size.unwrap_or(info.frame.size);
    let min_size = info.min_size.unwrap_or(info.frame.size);
    let looks_like_primary_window = max_size.width >= 1200.0
        || max_size.height >= 900.0
        || min_size.width >= 1000.0
        || info.frame.size.width >= 1200.0
        || info.frame.size.height >= 900.0;

    path_matches && bundle_matches && looks_like_primary_window
}
