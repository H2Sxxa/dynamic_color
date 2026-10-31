use std::fmt;

/// An opaque ARGB color returned by the operating system.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct AccentColor(u32);

impl AccentColor {
    /// Creates a color from a packed ARGB value.
    #[must_use]
    pub const fn from_argb(argb: u32) -> Self {
        Self(argb)
    }

    /// Creates an opaque color from RGB channels.
    #[must_use]
    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self(0xff00_0000 | (red as u32) << 16 | (green as u32) << 8 | blue as u32)
    }

    /// Returns the packed ARGB value.
    #[must_use]
    pub const fn argb(self) -> u32 {
        self.0
    }

    /// Returns the alpha channel.
    #[must_use]
    pub const fn alpha(self) -> u8 {
        (self.0 >> 24) as u8
    }

    /// Returns the red channel.
    #[must_use]
    pub const fn red(self) -> u8 {
        (self.0 >> 16) as u8
    }

    /// Returns the green channel.
    #[must_use]
    pub const fn green(self) -> u8 {
        (self.0 >> 8) as u8
    }

    /// Returns the blue channel.
    #[must_use]
    pub const fn blue(self) -> u8 {
        self.0 as u8
    }

    /// Formats the color as an opaque `#rrggbb` string.
    #[must_use]
    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.red(), self.green(), self.blue())
    }
}

impl From<u32> for AccentColor {
    fn from(argb: u32) -> Self {
        Self::from_argb(argb)
    }
}

impl From<AccentColor> for u32 {
    fn from(color: AccentColor) -> Self {
        color.argb()
    }
}

impl fmt::Display for AccentColor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_hex())
    }
}
