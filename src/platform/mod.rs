use crate::{AccentColor, Error};

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "android")]
pub mod android;

#[cfg(all(target_os = "linux", feature = "gtk"))]
#[path = "linux_gtk.rs"]
pub mod gtk;

#[cfg(target_os = "windows")]
pub(crate) fn accent_color() -> Result<AccentColor, Error> {
    windows::accent_color()
}

#[cfg(target_os = "macos")]
pub(crate) fn accent_color() -> Result<AccentColor, Error> {
    macos::accent_color()
}

#[cfg(target_os = "linux")]
pub(crate) fn accent_color() -> Result<AccentColor, Error> {
    linux::accent_color()
}

#[cfg(target_os = "android")]
pub(crate) fn accent_color() -> Result<AccentColor, Error> {
    android::accent_color()
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "linux",
    target_os = "android"
)))]
pub(crate) fn accent_color() -> Result<AccentColor, Error> {
    Err(Error::UnsupportedPlatform {
        platform: std::env::consts::OS,
    })
}
