use objc2_app_kit::{NSColor, NSColorSpace};

use crate::{AccentColor, Error};

pub(crate) fn accent_color() -> Result<AccentColor, Error> {
    let accent = NSColor::controlAccentColor();
    let srgb = NSColorSpace::sRGBColorSpace();
    let converted = accent.colorUsingColorSpace(&srgb).ok_or_else(|| {
        Error::query(
            "macos",
            "convert NSColor.controlAccentColor",
            "could not convert the system accent color to the sRGB color space".to_string(),
        )
    })?;

    Ok(AccentColor::from_rgb(
        channel_to_u8(converted.redComponent()),
        channel_to_u8(converted.greenComponent()),
        channel_to_u8(converted.blueComponent()),
    ))
}

fn channel_to_u8(component: f64) -> u8 {
    (component.clamp(0.0, 1.0) * 255.0).round() as u8
}
