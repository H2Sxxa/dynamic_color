//! Optional GTK integration for Linux accent colors.
//!
//! This helper mirrors Flutter's Linux implementation by reading the active
//! GTK theme's `theme_selected_bg_color`. Call it on the GTK main thread with
//! an initialized widget.

use ::gtk::glib::object::IsA;
use ::gtk::prelude::{StyleContextExt, WidgetExt};

use crate::{AccentColor, Error};

/// Returns the accent color from a GTK widget's style context.
///
/// The widget must belong to an initialized GTK application, and this function
/// must run on the GTK main thread.
///
/// # Errors
///
/// Returns [`Error::Query`] when the active theme does not define
/// `theme_selected_bg_color` or supplies an invalid channel value.
pub fn accent_color_from_widget(widget: &impl IsA<::gtk::Widget>) -> Result<AccentColor, Error> {
    accent_color_from_style_context(&widget.style_context())
}

/// Returns the accent color from an existing GTK style context.
///
/// # Errors
///
/// Returns [`Error::Query`] when the style context does not define
/// `theme_selected_bg_color` or supplies an invalid channel value.
pub fn accent_color_from_style_context(
    style_context: &::gtk::StyleContext,
) -> Result<AccentColor, Error> {
    let color = style_context
        .lookup_color("theme_selected_bg_color")
        .ok_or_else(|| {
            Error::query(
                "linux",
                "GTK theme_selected_bg_color lookup",
                String::from("the active GTK theme does not define theme_selected_bg_color"),
            )
        })?;

    let red = channel_to_u8(color.red(), "red")?;
    let green = channel_to_u8(color.green(), "green")?;
    let blue = channel_to_u8(color.blue(), "blue")?;

    Ok(AccentColor::from_rgb(red, green, blue))
}

fn channel_to_u8(component: f64, channel: &'static str) -> Result<u8, Error> {
    if !(0.0..=1.0).contains(&component) {
        return Err(Error::query(
            "linux",
            "GTK theme_selected_bg_color lookup",
            format!("{channel} channel is outside the range 0.0..=1.0: {component}"),
        ));
    }

    Ok((component * 255.0).round() as u8)
}
