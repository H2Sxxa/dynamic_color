//! Lightweight access to the operating system's dynamic accent color.
//!
//! Desktop platforms return a single seed color. Android can additionally
//! return the exact 65 system colors used by dynamic color. Enable the
//! `material` feature to map those colors to a Material 3 light or dark scheme
//! without an external Material Color Utilities dependency. Enable the `seed`
//! feature to generate a full palette from any accent seed on every platform.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod color;
mod error;
#[cfg(feature = "material")]
pub mod material;
mod platform;

#[cfg(feature = "tauri")]
pub mod tauri;

#[cfg(all(target_os = "linux", feature = "gtk"))]
pub use platform::gtk;

#[cfg(target_os = "android")]
pub use platform::android;

pub use color::AccentColor;
pub use error::Error;
#[cfg(feature = "material")]
pub use material::{ColorScheme, CorePalette, TONES, TonalPalette, Tone};

/// Returns the current operating system accent color.
///
/// Windows uses `UISettings`, macOS uses `NSColor.controlAccentColor`, Linux
/// first queries the XDG Desktop Portal and then falls back to GNOME's
/// accent-color setting, and Android uses `system_accent1_500`. The optional
/// `android-activity` feature adds compatibility with hosts that initialize JNI
/// through `android-activity`. With the `material` feature, Android also
/// exposes `android::core_palette` for the exact 65 system colors.
///
/// # Errors
///
/// Returns [`Error::UnsupportedPlatform`] on unsupported targets and
/// [`Error::Query`] when the platform backend cannot provide a color.
pub fn accent_color() -> Result<AccentColor, Error> {
    platform::accent_color()
}

/// Generates a Material 3 core palette from the current accent color seed.
///
/// This function is available with the `seed` feature. On Android, use
/// `android::core_palette()` instead when the exact system palette is required.
///
/// # Errors
///
/// Returns the same platform errors as [`accent_color`].
#[cfg(feature = "seed")]
pub fn accent_color_core_palette() -> Result<material::CorePalette, Error> {
    accent_color().map(material::CorePalette::from_seed)
}

/// Generates a Material 3 light scheme from the current accent color seed.
///
/// This function is available with the `seed` feature. On Android, use
/// `android::light_color_scheme()` instead when the exact system palette is
/// required.
///
/// # Errors
///
/// Returns the same platform errors as [`accent_color`].
#[cfg(feature = "seed")]
pub fn accent_color_light_scheme() -> Result<material::ColorScheme, Error> {
    accent_color_core_palette().map(|palette| palette.light_scheme())
}

/// Generates a Material 3 dark scheme from the current accent color seed.
///
/// This function is available with the `seed` feature. On Android, use
/// `android::dark_color_scheme()` instead when the exact system palette is
/// required.
///
/// # Errors
///
/// Returns the same platform errors as [`accent_color`].
#[cfg(feature = "seed")]
pub fn accent_color_dark_scheme() -> Result<material::ColorScheme, Error> {
    accent_color_core_palette().map(|palette| palette.dark_scheme())
}
