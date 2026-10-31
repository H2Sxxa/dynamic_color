use windows::UI::ViewManagement::{UIColorType, UISettings};

use crate::{AccentColor, Error};

pub(crate) fn accent_color() -> Result<AccentColor, Error> {
    let settings = UISettings::new().map_err(|error| {
        Error::query(
            "windows",
            "create UISettings",
            format!("WinRT UISettings initialization failed: {error}"),
        )
    })?;

    let color = settings
        .GetColorValue(UIColorType::Accent)
        .map_err(|error| {
            Error::query(
                "windows",
                "UISettings.GetColorValue",
                format!("could not read UIColorType::Accent: {error}"),
            )
        })?;

    Ok(AccentColor::from_rgb(color.R, color.G, color.B))
}
