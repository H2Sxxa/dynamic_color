//! Optional Tauri command integration.
//!
//! Enable the `tauri` feature to compile and register
//! [`system_accent_color`] as an application command.

use ::tauri::command as tauri_command;

use crate::Error;

/// Returns the current system accent color as an opaque `#rrggbb` string.
///
/// Register this command in the application:
///
/// ```rust,ignore
/// tauri::Builder::default()
///     .invoke_handler(tauri::generate_handler![
///         dynamic_color::tauri::system_accent_color
///     ]);
/// ```
///
/// # Errors
///
/// Returns the same platform query errors as [`crate::accent_color`].
#[tauri_command]
pub fn system_accent_color() -> Result<String, Error> {
    crate::accent_color().map(|color| color.to_hex())
}
