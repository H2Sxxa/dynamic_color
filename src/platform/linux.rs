use std::process::Command;

use crate::{AccentColor, Error};

pub(crate) fn accent_color() -> Result<AccentColor, Error> {
    let portal_error = match read_portal_accent_color() {
        Ok(color) => return Ok(color),
        Err(error) => error,
    };

    let gsettings_error = match read_gsettings_accent_color() {
        Ok(color) => return Ok(color),
        Err(error) => error,
    };

    Err(Error::query(
        "linux",
        "accent color query",
        format!(
            "XDG Desktop Portal failed: {portal_error}; GNOME accent-color fallback failed: {gsettings_error}"
        ),
    ))
}

fn read_portal_accent_color() -> Result<AccentColor, String> {
    let output = run_command(
        "gdbus",
        &[
            "call",
            "--session",
            "--dest",
            "org.freedesktop.portal.Desktop",
            "--object-path",
            "/org/freedesktop/portal/desktop",
            "--method",
            "org.freedesktop.portal.Settings.Read",
            "org.freedesktop.appearance",
            "accent-color",
        ],
    )?;

    parse_portal_rgb(&output)
}

// Legacy GNOME-only fallback. This maps a setting name to fixed RGB values
// rather than reading the active GTK theme. Enable the `gtk` feature for a
// theme-aware `theme_selected_bg_color` lookup.
fn read_gsettings_accent_color() -> Result<AccentColor, String> {
    let output = run_command(
        "gsettings",
        &["get", "org.gnome.desktop.interface", "accent-color"],
    )?;
    let name = output
        .trim()
        .trim_matches(|character| character == '\'' || character == '"');

    accent_name_to_color(name)
        .ok_or_else(|| format!("unsupported GNOME accent-color value: {name:?}"))
}

// Dependency-free legacy path for the GNOME fallback. It trades process-spawn
// and text-parsing fragility for not linking GLib or a D-Bus client.
fn run_command(program: &str, arguments: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(arguments)
        .output()
        .map_err(|error| format!("failed to start {program}: {error}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "{program} exited with {}: {}",
            output.status,
            stderr.trim()
        ));
    }

    String::from_utf8(output.stdout)
        .map_err(|error| format!("{program} returned non-UTF-8 output: {error}"))
}

fn parse_portal_rgb(output: &str) -> Result<AccentColor, String> {
    let start = output
        .find("<(")
        .map(|index| index + 2)
        .ok_or_else(|| format!("unexpected XDG portal response: {output:?}"))?;
    let remainder = &output[start..];
    let end = remainder
        .find(")>")
        .ok_or_else(|| format!("unexpected XDG portal response: {output:?}"))?;
    let mut channels = remainder[..end].split(',');

    let red = parse_channel(channels.next(), "red")?;
    let green = parse_channel(channels.next(), "green")?;
    let blue = parse_channel(channels.next(), "blue")?;

    if channels.next().is_some() {
        return Err(format!("unexpected XDG portal response: {output:?}"));
    }

    Ok(AccentColor::from_rgb(red, green, blue))
}

fn parse_channel(value: Option<&str>, channel: &str) -> Result<u8, String> {
    let component = value
        .ok_or_else(|| format!("XDG portal response is missing the {channel} channel"))?
        .trim()
        .parse::<f64>()
        .map_err(|error| format!("invalid {channel} channel: {error}"))?;

    if !(0.0..=1.0).contains(&component) {
        return Err(format!(
            "{channel} channel is outside the range 0.0..=1.0: {component}"
        ));
    }

    Ok((component * 255.0).round() as u8)
}

fn accent_name_to_color(name: &str) -> Option<AccentColor> {
    let (red, green, blue) = match name {
        "blue" => (0x35, 0x84, 0xe4),
        "teal" => (0x21, 0x90, 0xa4),
        "green" => (0x3a, 0x94, 0x4a),
        "yellow" => (0xc8, 0x88, 0x00),
        "orange" => (0xed, 0x5b, 0x00),
        "red" => (0xe6, 0x2d, 0x42),
        "pink" => (0xd5, 0x61, 0x99),
        "purple" => (0x91, 0x41, 0xac),
        "slate" => (0x6f, 0x83, 0x96),
        _ => return None,
    };

    Some(AccentColor::from_rgb(red, green, blue))
}
