# dynamic-color

`dynamic-color` is a lightweight Rust crate for reading the operating system's
dynamic accent color. It queries the platform directly, returns an opaque
`AccentColor`, and can optionally map or generate Material 3 tonal palettes.

Reference to the famous Flutter package
[`dynamic_color`](https://github.com/material-foundation/flutter-packages/tree/main/packages/dynamic_color):
this crate follows its platform lookup strategy, Android's 65-color resource
ordering, and `CorePalette` to `ColorScheme` role mapping.

## What it provides

- One seed accent color from Windows, macOS, Linux, or Android.
- The exact 65 dynamic colors exposed by Android 12+.
- Material 3 light and dark schemes with legacy `Scheme` roles.
- Optional seed-based palette generation for platforms that expose only one
  accent color.
- Optional serialization, Tauri, GTK, and `android-activity` integrations.
- Target-specific dependencies: platform selection uses `target_os`, not Cargo
  platform features.
- No `unsafe` code in this crate (`#![forbid(unsafe_code)]`).

## Feature flags

The crate has no default features.

| Feature | Default | Purpose |
| --- | --- | --- |
| `serde` | No | Adds serialization support for colors, palettes, schemes, and command errors. |
| `material` | No | Adds `CorePalette`, `TonalPalette`, `Tone`, and `ColorScheme` without an external palette dependency. On Android, also exposes the exact 65 system colors. |
| `seed` | No | Generates a core palette and schemes from any accent seed. Implies `material` and adds `material-colors` 0.4.x. |
| `android-activity` | No | Adds Android adapters for applications using `android-activity` 0.6. |
| `gtk` | No | Adds a Linux-only GTK 0.18 helper for reading the active theme's selected background color. |
| `tauri` | No | Adds a Tauri 2 command returning `#rrggbb`. Implies `serde`. |

Only the selected platform backend and its target-specific dependencies are
compiled. Android support itself is not a feature flag.

## Supported platforms

| Platform | Backend | Notes |
| --- | --- | --- |
| Windows | `UISettings.GetColorValue(UIColorType::Accent)` | Uses the Windows UI accent color. |
| macOS 10.14+ | `NSColor.controlAccentColor` | Converted to sRGB. |
| Linux | XDG Desktop Portal, then GNOME `accent-color` | The default path invokes `gdbus` and falls back to `gsettings`. |
| Android 12+ | `android.R.color` system resources through JNI | Requires API 31 or newer. The seed path reads `system_accent1_500`. |
| Other targets | `Error::UnsupportedPlatform` | No fallback backend is selected. |

## Installation

```toml
[dependencies]
dynamic-color = "0.1"
```

## Read the accent color

```rust
fn main() -> Result<(), dynamic_color::Error> {
    let accent = dynamic_color::accent_color()?;

    println!("hex: {}", accent.to_hex());
    println!("argb: {:#010x}", accent.argb());
    println!(
        "rgb: ({}, {}, {})",
        accent.red(),
        accent.green(),
        accent.blue()
    );

    Ok(())
}
```

`AccentColor` is a packed ARGB value and exposes `argb()`, `alpha()`, `red()`,
`green()`, `blue()`, and `to_hex()`. `to_hex()` always returns an opaque
lowercase `#rrggbb` string.

Enable `serde` when the values need to cross a serialization boundary:

```toml
[dependencies]
dynamic-color = { version = "0.1", features = ["serde"] }
```

## Exact Android 65 colors

Android 12+ exposes five tonal palettes with thirteen common tones each:

| Palette | Android resources |
| --- | --- |
| Primary | `system_accent1_*` |
| Secondary | `system_accent2_*` |
| Tertiary | `system_accent3_*` |
| Neutral | `system_neutral1_*` |
| Neutral variant | `system_neutral2_*` |

The stored Material tones are:

```text
0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99, 100
```

Android resource suffixes describe perceptual darkness and run in the opposite
direction. Material tone 0 is `system_accent1_1000`, while tone 100 is
`system_accent1_0`.

Enable the dependency-free `material` feature to read the exact colors:

```toml
[dependencies]
dynamic-color = { version = "0.1", features = ["material"] }
```

```rust
#[cfg(target_os = "android")]
fn main() -> Result<(), dynamic_color::Error> {
    use dynamic_color::Tone;

    let palette = dynamic_color::android::core_palette()?;
    let primary_40 = palette.primary.tone(Tone::T40);

    println!("primary tone 40: {}", primary_40.to_hex());
    println!("all 65 colors: {:?}", palette.colors());

    let light = palette.light_scheme();
    let dark = palette.dark_scheme();
    println!("light primary: {}", light.primary.to_hex());
    println!("dark primary: {}", dark.primary.to_hex());

    Ok(())
}

#[cfg(not(target_os = "android"))]
fn main() {}
```

`palette.colors()` returns all 65 values in primary, secondary, tertiary,
neutral, and neutral-variant order. Each palette's tones are ordered from tone 0
to tone 100.

`android::light_color_scheme()` and `android::dark_color_scheme()` are
convenience functions that query and map the palette in one call.

The 65-color resource order follows `DynamicColorPlugin.kt` in Flutter's
`dynamic_color` package. The role mapping follows
`corepalette_to_colorscheme.dart` from the same package.

The five Android palettes do not include error colors. `ColorScheme` therefore
uses the standard Material 3 error tones, which match Android's default
`system_error_*` colors. Every non-error role comes directly from one of the 65
system colors.

The `material` feature does not use `material-colors` or another Material Color
Utilities crate. Those crates generate tones from a seed hue and chroma; they do
not accept Android's explicit color list. This crate reads the list through JNI
and maps the same tones directly. Seed generation is isolated behind the
separate `seed` feature.

## Generate from a seed

Desktop platforms usually expose only one accent color. Enable `seed` to
generate the same five-palette structure from that seed:

```toml
[dependencies]
dynamic-color = { version = "0.1", features = ["seed"] }
```

The `seed` feature implies `material` and adds the optional `material-colors`
dependency only to builds that need generation.

```rust
fn main() -> Result<(), dynamic_color::Error> {
    let palette = dynamic_color::accent_color_core_palette()?;
    let light = palette.light_scheme();
    let dark = palette.dark_scheme();

    println!("light primary: {}", light.primary.to_hex());
    println!("dark primary: {}", dark.primary.to_hex());

    Ok(())
}
```

`accent_color_light_scheme()` and `accent_color_dark_scheme()` provide the same
result without holding the intermediate palette.

A known seed can be converted directly:

```rust
use dynamic_color::{AccentColor, CorePalette};

let seed = AccentColor::from_rgb(0x4c, 0xaf, 0x50);
let palette = CorePalette::from_seed(seed);
let scheme = palette.dark_scheme();
```

Seed generation uses `material-colors` 0.4.x and the existing legacy `Scheme`
role mapping. It does not synthesize newer Material 3 surface-container roles.
On Android, prefer `android::core_palette()` when the exact system colors must
be preserved instead of being regenerated from a seed.

Version 0.5 of `material-colors` requires Rust 1.97, while this crate keeps its
Rust 1.85 MSRV. The dependency remains optional.

## Tauri feature

The `tauri` feature adds a prebuilt application command without adding Tauri to
the default dependency graph:

```toml
[dependencies]
dynamic-color = { version = "0.1", features = ["tauri"] }
```

```rust
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        dynamic_color::tauri::system_accent_color
    ]);
```

```ts
import { invoke } from "@tauri-apps/api/core";

const accentColor = await invoke<string>("system_accent_color");
```

The command returns `#rrggbb`. The `tauri` feature also enables `serde` so query
errors can be serialized for the frontend.

## GTK feature

The `gtk` feature is Linux-only and exposes a helper that reads
`theme_selected_bg_color` from an initialized GTK widget or style context. This
matches Flutter's Linux behavior without making GTK a default dependency:

```toml
[dependencies]
dynamic-color = { version = "0.1", features = ["gtk"] }
gtk = "0.18"
```

```rust
use gtk::prelude::*;

let accent = dynamic_color::gtk::accent_color_from_widget(&window)?;
println!("{}", accent.to_hex());
```

`accent_color_from_style_context()` is available when an existing
`gtk::StyleContext` should be queried directly. Call either helper on the GTK
main thread with a widget from a running application. The crate does not
initialize GTK or create a hidden window because doing so would violate GTK's
main-thread and application-lifecycle requirements.

## Android runtime

Android support is selected by `target_os` and does not require a backend
feature. Applications that initialize JNI through `android-activity` can enable
the adapter feature:

```toml
[dependencies]
dynamic-color = { version = "0.1", features = ["android-activity", "material"] }
android-activity = { version = "0.6.1", features = ["native-activity"] }
```

The `android-activity` feature must not select an Activity type itself because
the host application owns that choice. Call the adapters after `android_main`
has started:

```rust
fn android_main(app: android_activity::AndroidApp) {
    let accent = dynamic_color::android::accent_color_from_app(&app);
    let palette = dynamic_color::android::core_palette_from_app(&app);
    println!("{accent:?} {palette:?}");
}
```

Applications that do not enable `android-activity` still get the Android
backend, but must initialize the `jni` 0.22 `JavaVM` singleton through their own
JNI entry point. Tauri's current Android runtime uses a different JNI version,
so it needs an explicit adapter before this backend can be used there.

## Scope and limitations

This crate performs read-once queries. It does not:

- extract palettes from wallpapers or images;
- notify callers when the system palette changes;
- synthesize newer Material 3 surface-container roles;
- promise identical colors across Android vendors, desktop themes, or GTK
  themes.

`ColorScheme` contains the legacy Material `Scheme` roles plus `surface_tint`.
The Android exact-color path preserves the 65 system values and only supplies
standard error tones for the four error roles.

## MSRV

The minimum supported Rust version is 1.85. The `seed` feature uses
`material-colors` 0.4.x to preserve that MSRV.

## License

MIT