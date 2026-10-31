//! Material 3 tonal palettes and role mapping.
//!
//! Android supplies five tonal palettes with thirteen tones each. This module
//! stores those values without regenerating them and maps the supported tones
//! to the standard Material 3 color roles. With the `seed` feature, it can also
//! generate the same palette structure from any accent seed.

use crate::AccentColor;
#[cfg(feature = "seed")]
pub use material_colors;

/// A Material 3 tone available from the Android system palette.
#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Tone {
    /// Tone 0, the darkest tone.
    T0 = 0,
    /// Tone 10.
    T10 = 10,
    /// Tone 20.
    T20 = 20,
    /// Tone 30.
    T30 = 30,
    /// Tone 40.
    T40 = 40,
    /// Tone 50.
    T50 = 50,
    /// Tone 60.
    T60 = 60,
    /// Tone 70.
    T70 = 70,
    /// Tone 80.
    T80 = 80,
    /// Tone 90.
    T90 = 90,
    /// Tone 95.
    T95 = 95,
    /// Tone 99.
    T99 = 99,
    /// Tone 100, the lightest tone.
    T100 = 100,
}

impl Tone {
    /// Returns the numeric Material tone.
    #[must_use]
    pub const fn value(self) -> u16 {
        self as u16
    }

    const fn index(self) -> usize {
        match self {
            Self::T0 => 0,
            Self::T10 => 1,
            Self::T20 => 2,
            Self::T30 => 3,
            Self::T40 => 4,
            Self::T50 => 5,
            Self::T60 => 6,
            Self::T70 => 7,
            Self::T80 => 8,
            Self::T90 => 9,
            Self::T95 => 10,
            Self::T99 => 11,
            Self::T100 => 12,
        }
    }
}

/// Material tones stored in each [`TonalPalette`], ordered from darkest to
/// lightest.
pub const TONES: [Tone; 13] = [
    Tone::T0,
    Tone::T10,
    Tone::T20,
    Tone::T30,
    Tone::T40,
    Tone::T50,
    Tone::T60,
    Tone::T70,
    Tone::T80,
    Tone::T90,
    Tone::T95,
    Tone::T99,
    Tone::T100,
];

/// One Material tonal palette containing the thirteen Android system tones.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TonalPalette {
    tones: [AccentColor; 13],
}

impl TonalPalette {
    /// Creates a palette from tones ordered according to [`TONES`].
    #[must_use]
    pub const fn new(tones: [AccentColor; 13]) -> Self {
        Self { tones }
    }

    /// Returns one tone from the palette.
    #[must_use]
    pub const fn tone(&self, tone: Tone) -> AccentColor {
        self.tones[tone.index()]
    }

    /// Returns all tones ordered according to [`TONES`].
    #[must_use]
    pub const fn tones(&self) -> &[AccentColor; 13] {
        &self.tones
    }
}

/// The five tonal palettes returned by Android dynamic color.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CorePalette {
    /// Primary tonal palette (`system_accent1`).
    pub primary: TonalPalette,
    /// Secondary tonal palette (`system_accent2`).
    pub secondary: TonalPalette,
    /// Tertiary tonal palette (`system_accent3`).
    pub tertiary: TonalPalette,
    /// Neutral tonal palette (`system_neutral1`).
    pub neutral: TonalPalette,
    /// Neutral-variant tonal palette (`system_neutral2`).
    pub neutral_variant: TonalPalette,
}

impl CorePalette {
    /// Creates a core palette from five tonal palettes.
    #[must_use]
    pub const fn new(
        primary: TonalPalette,
        secondary: TonalPalette,
        tertiary: TonalPalette,
        neutral: TonalPalette,
        neutral_variant: TonalPalette,
    ) -> Self {
        Self {
            primary,
            secondary,
            tertiary,
            neutral,
            neutral_variant,
        }
    }

    /// Generates a core palette from an accent seed.
    ///
    /// This method is available with the `seed` feature. It uses Material Color
    /// Utilities' tonal-spot generation and stores the thirteen common tones
    /// from each of the five generated tonal palettes.
    #[cfg(feature = "seed")]
    #[must_use]
    pub fn from_seed(seed: AccentColor) -> Self {
        let source = material_colors::dynamic_color::DynamicScheme::by_variant(
            material_colors::color::Rgb::new(seed.red(), seed.green(), seed.blue()),
            &material_colors::dynamic_color::Variant::TonalSpot,
            false,
            None,
        );

        Self::new(
            tonal_palette_from_material(&source.primary_palette),
            tonal_palette_from_material(&source.secondary_palette),
            tonal_palette_from_material(&source.tertiary_palette),
            tonal_palette_from_material(&source.neutral_palette),
            tonal_palette_from_material(&source.neutral_variant_palette),
        )
    }

    /// Returns the five palettes in Android dynamic-color order.
    #[must_use]
    pub const fn palettes(&self) -> [&TonalPalette; 5] {
        [
            &self.primary,
            &self.secondary,
            &self.tertiary,
            &self.neutral,
            &self.neutral_variant,
        ]
    }

    /// Returns all 65 colors in palette order, with tones ordered according to
    /// [`TONES`].
    #[must_use]
    pub fn colors(&self) -> [AccentColor; 65] {
        let mut colors = [AccentColor::from_argb(0); 65];

        for (palette_index, palette) in self.palettes().into_iter().enumerate() {
            let offset = palette_index * TONES.len();
            colors[offset..offset + TONES.len()].copy_from_slice(palette.tones());
        }

        colors
    }

    /// Maps this palette to the Material 3 light color scheme.
    #[must_use]
    pub fn light_scheme(&self) -> ColorScheme {
        ColorScheme {
            primary: self.primary.tone(Tone::T40),
            on_primary: self.primary.tone(Tone::T100),
            primary_container: self.primary.tone(Tone::T90),
            on_primary_container: self.primary.tone(Tone::T10),
            secondary: self.secondary.tone(Tone::T40),
            on_secondary: self.secondary.tone(Tone::T100),
            secondary_container: self.secondary.tone(Tone::T90),
            on_secondary_container: self.secondary.tone(Tone::T10),
            tertiary: self.tertiary.tone(Tone::T40),
            on_tertiary: self.tertiary.tone(Tone::T100),
            tertiary_container: self.tertiary.tone(Tone::T90),
            on_tertiary_container: self.tertiary.tone(Tone::T10),
            error: error_palette().tone(Tone::T40),
            on_error: error_palette().tone(Tone::T100),
            error_container: error_palette().tone(Tone::T90),
            on_error_container: error_palette().tone(Tone::T10),
            background: self.neutral.tone(Tone::T99),
            on_background: self.neutral.tone(Tone::T10),
            surface: self.neutral.tone(Tone::T99),
            on_surface: self.neutral.tone(Tone::T10),
            surface_variant: self.neutral_variant.tone(Tone::T90),
            on_surface_variant: self.neutral_variant.tone(Tone::T30),
            outline: self.neutral_variant.tone(Tone::T50),
            outline_variant: self.neutral_variant.tone(Tone::T80),
            shadow: self.neutral.tone(Tone::T0),
            scrim: self.neutral.tone(Tone::T0),
            inverse_surface: self.neutral.tone(Tone::T20),
            inverse_on_surface: self.neutral.tone(Tone::T95),
            inverse_primary: self.primary.tone(Tone::T80),
            surface_tint: self.primary.tone(Tone::T40),
        }
    }

    /// Maps this palette to the Material 3 dark color scheme.
    #[must_use]
    pub fn dark_scheme(&self) -> ColorScheme {
        ColorScheme {
            primary: self.primary.tone(Tone::T80),
            on_primary: self.primary.tone(Tone::T20),
            primary_container: self.primary.tone(Tone::T30),
            on_primary_container: self.primary.tone(Tone::T90),
            secondary: self.secondary.tone(Tone::T80),
            on_secondary: self.secondary.tone(Tone::T20),
            secondary_container: self.secondary.tone(Tone::T30),
            on_secondary_container: self.secondary.tone(Tone::T90),
            tertiary: self.tertiary.tone(Tone::T80),
            on_tertiary: self.tertiary.tone(Tone::T20),
            tertiary_container: self.tertiary.tone(Tone::T30),
            on_tertiary_container: self.tertiary.tone(Tone::T90),
            error: error_palette().tone(Tone::T80),
            on_error: error_palette().tone(Tone::T20),
            error_container: error_palette().tone(Tone::T30),
            on_error_container: error_palette().tone(Tone::T80),
            background: self.neutral.tone(Tone::T10),
            on_background: self.neutral.tone(Tone::T90),
            surface: self.neutral.tone(Tone::T10),
            on_surface: self.neutral.tone(Tone::T90),
            surface_variant: self.neutral_variant.tone(Tone::T30),
            on_surface_variant: self.neutral_variant.tone(Tone::T80),
            outline: self.neutral_variant.tone(Tone::T60),
            outline_variant: self.neutral_variant.tone(Tone::T30),
            shadow: self.neutral.tone(Tone::T0),
            scrim: self.neutral.tone(Tone::T0),
            inverse_surface: self.neutral.tone(Tone::T90),
            inverse_on_surface: self.neutral.tone(Tone::T20),
            inverse_primary: self.primary.tone(Tone::T40),
            surface_tint: self.primary.tone(Tone::T80),
        }
    }
}

#[cfg(feature = "seed")]
fn tonal_palette_from_material(palette: &material_colors::palette::TonalPalette) -> TonalPalette {
    TonalPalette::new(TONES.map(|tone| {
        let color = palette.tone(i32::from(tone.value()));
        AccentColor::from_rgb(color.red, color.green, color.blue)
    }))
}

// Android's five-palette contract does not include error colors. These are the
// standard Material 3 error tones, matching Android's default error resources.
const ERROR_PALETTE: TonalPalette = TonalPalette::new([
    AccentColor::from_rgb(0x00, 0x00, 0x00),
    AccentColor::from_rgb(0x41, 0x00, 0x02),
    AccentColor::from_rgb(0x69, 0x00, 0x05),
    AccentColor::from_rgb(0x93, 0x00, 0x0a),
    AccentColor::from_rgb(0xba, 0x1a, 0x1a),
    AccentColor::from_rgb(0xde, 0x37, 0x30),
    AccentColor::from_rgb(0xff, 0x54, 0x49),
    AccentColor::from_rgb(0xff, 0x89, 0x7d),
    AccentColor::from_rgb(0xff, 0xb4, 0xab),
    AccentColor::from_rgb(0xff, 0xda, 0xd6),
    AccentColor::from_rgb(0xff, 0xed, 0xea),
    AccentColor::from_rgb(0xff, 0xfb, 0xff),
    AccentColor::from_rgb(0xff, 0xff, 0xff),
]);

const fn error_palette() -> TonalPalette {
    ERROR_PALETTE
}

/// A Material 3 color scheme generated from an Android [`CorePalette`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColorScheme {
    /// Primary color.
    pub primary: AccentColor,
    /// Content color used on top of [`Self::primary`].
    pub on_primary: AccentColor,
    /// Primary container color.
    pub primary_container: AccentColor,
    /// Content color used on top of [`Self::primary_container`].
    pub on_primary_container: AccentColor,
    /// Secondary color.
    pub secondary: AccentColor,
    /// Content color used on top of [`Self::secondary`].
    pub on_secondary: AccentColor,
    /// Secondary container color.
    pub secondary_container: AccentColor,
    /// Content color used on top of [`Self::secondary_container`].
    pub on_secondary_container: AccentColor,
    /// Tertiary color.
    pub tertiary: AccentColor,
    /// Content color used on top of [`Self::tertiary`].
    pub on_tertiary: AccentColor,
    /// Tertiary container color.
    pub tertiary_container: AccentColor,
    /// Content color used on top of [`Self::tertiary_container`].
    pub on_tertiary_container: AccentColor,
    /// Error color.
    pub error: AccentColor,
    /// Content color used on top of [`Self::error`].
    pub on_error: AccentColor,
    /// Error container color.
    pub error_container: AccentColor,
    /// Content color used on top of [`Self::error_container`].
    pub on_error_container: AccentColor,
    /// Deprecated Material 2 background color.
    pub background: AccentColor,
    /// Deprecated Material 2 content color used on top of the background.
    pub on_background: AccentColor,
    /// Surface color.
    pub surface: AccentColor,
    /// Content color used on top of [`Self::surface`].
    pub on_surface: AccentColor,
    /// Surface-variant color.
    pub surface_variant: AccentColor,
    /// Content color used on top of [`Self::surface_variant`].
    pub on_surface_variant: AccentColor,
    /// Outline color.
    pub outline: AccentColor,
    /// Subtle outline color.
    pub outline_variant: AccentColor,
    /// Shadow color.
    pub shadow: AccentColor,
    /// Scrim color.
    pub scrim: AccentColor,
    /// Surface color used for inverse content.
    pub inverse_surface: AccentColor,
    /// Content color used on top of [`Self::inverse_surface`].
    pub inverse_on_surface: AccentColor,
    /// Primary color used on top of [`Self::inverse_surface`].
    pub inverse_primary: AccentColor,
    /// Color used to tint elevated surfaces.
    pub surface_tint: AccentColor,
}
