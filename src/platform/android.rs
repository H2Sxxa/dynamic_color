//! Android dynamic-color backend.
//!
//! The backend uses JNI for every Android build. The optional `android-activity`
//! feature exposes adapters for applications using `android-activity`.

use jni::{
    JValue, jni_sig, jni_str,
    objects::{JClass, JObject},
    strings::JNIString,
    vm::JavaVM,
};

#[cfg(feature = "material")]
use crate::material::{CorePalette, TonalPalette};
use crate::{AccentColor, Error};

const MIN_ANDROID_SDK: i32 = 31;

// Android resource suffixes use perceptual darkness, so they run opposite to
// Material tones: tone 0 is system_*_1000 and tone 100 is system_*_0.
#[cfg(feature = "material")]
const TONE_RESOURCE_SUFFIXES: [u16; 13] =
    [1000, 900, 800, 700, 600, 500, 400, 300, 200, 100, 50, 10, 0];

pub(crate) fn accent_color() -> Result<AccentColor, Error> {
    with_attached_environment("system accent color query", read_accent_color)
}

/// Returns the system accent color from an `android-activity` application.
///
/// Call this after `android_main` has started. The application handle proves
/// that `android-activity` has initialized the shared `jni` 0.22 `JavaVM`.
///
/// # Errors
///
/// Returns [`Error::Query`] when the JavaVM is unavailable or the system color
/// cannot be queried.
#[cfg(feature = "android-activity")]
pub fn accent_color_from_app(_app: &android_activity::AndroidApp) -> Result<AccentColor, Error> {
    accent_color()
}

/// Returns the exact 65 colors used by Android dynamic color.
///
/// The result contains primary, secondary, tertiary, neutral, and
/// neutral-variant palettes. Each palette contains Material tones 0, 10, 20,
/// 30, 40, 50, 60, 70, 80, 90, 95, 99, and 100.
///
/// Android 12 (API 31) or newer is required.
///
/// # Errors
///
/// Returns [`Error::Query`] when the JavaVM is unavailable, the Android version
/// is unsupported, or one of the system colors cannot be read.
#[cfg(feature = "material")]
pub fn core_palette() -> Result<CorePalette, Error> {
    with_attached_environment("system core palette query", read_core_palette)
}

/// Returns the Material 3 light scheme generated from Android colors.
///
/// # Errors
///
/// Returns the same errors as [`core_palette`].
#[cfg(feature = "material")]
pub fn light_color_scheme() -> Result<crate::ColorScheme, Error> {
    core_palette().map(|palette| palette.light_scheme())
}

/// Returns the Material 3 dark scheme generated from Android colors.
///
/// # Errors
///
/// Returns the same errors as [`core_palette`].
#[cfg(feature = "material")]
pub fn dark_color_scheme() -> Result<crate::ColorScheme, Error> {
    core_palette().map(|palette| palette.dark_scheme())
}

/// Returns the exact 65 system colors from an `android-activity` application.
///
/// # Errors
///
/// Returns the same errors as [`core_palette`].
#[cfg(all(feature = "android-activity", feature = "material"))]
pub fn core_palette_from_app(_app: &android_activity::AndroidApp) -> Result<CorePalette, Error> {
    core_palette()
}

/// Returns the Material 3 light scheme from an `android-activity` application.
///
/// # Errors
///
/// Returns the same errors as [`core_palette`].
#[cfg(all(feature = "android-activity", feature = "material"))]
pub fn light_color_scheme_from_app(
    _app: &android_activity::AndroidApp,
) -> Result<crate::ColorScheme, Error> {
    light_color_scheme()
}

/// Returns the Material 3 dark scheme from an `android-activity` application.
///
/// # Errors
///
/// Returns the same errors as [`core_palette`].
#[cfg(all(feature = "android-activity", feature = "material"))]
pub fn dark_color_scheme_from_app(
    _app: &android_activity::AndroidApp,
) -> Result<crate::ColorScheme, Error> {
    dark_color_scheme()
}

fn with_attached_environment<T>(
    operation: &'static str,
    query: impl FnOnce(&mut jni::Env<'_>) -> Result<T, String>,
) -> Result<T, Error> {
    let vm = JavaVM::singleton().map_err(|error| {
        Error::query(
            "android",
            "JavaVM lookup",
            format!(
                "the host must initialize the jni 0.22 JavaVM singleton before this call: {error}"
            ),
        )
    })?;

    let result = vm.attach_current_thread(|env| -> jni::errors::Result<_> { Ok(query(env)) });

    match result {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(detail)) => Err(Error::query("android", operation, detail)),
        Err(error) => Err(Error::query(
            "android",
            "JNI thread attachment",
            format!("failed to attach the current thread to the JavaVM: {error}"),
        )),
    }
}

fn read_accent_color(env: &mut jni::Env<'_>) -> Result<AccentColor, String> {
    ensure_supported_sdk(env)?;

    let color_class = find_color_class(env)?;
    let resources = system_resources(env)?;
    let resource_name = "system_accent1_500";
    let resource_id = read_color_resource_id(env, &color_class, resource_name)?;

    read_color(env, &resources, resource_id, resource_name)
}

#[cfg(feature = "material")]
fn read_core_palette(env: &mut jni::Env<'_>) -> Result<CorePalette, String> {
    ensure_supported_sdk(env)?;

    let color_class = find_color_class(env)?;
    let resources = system_resources(env)?;
    let palettes = [
        read_tonal_palette(env, &color_class, &resources, "accent1")?,
        read_tonal_palette(env, &color_class, &resources, "accent2")?,
        read_tonal_palette(env, &color_class, &resources, "accent3")?,
        read_tonal_palette(env, &color_class, &resources, "neutral1")?,
        read_tonal_palette(env, &color_class, &resources, "neutral2")?,
    ];

    Ok(CorePalette::new(
        palettes[0],
        palettes[1],
        palettes[2],
        palettes[3],
        palettes[4],
    ))
}

#[cfg(feature = "material")]
fn read_tonal_palette(
    env: &mut jni::Env<'_>,
    color_class: &JClass<'_>,
    resources: &JObject<'_>,
    palette_name: &str,
) -> Result<TonalPalette, String> {
    let mut tones = [AccentColor::from_argb(0); 13];

    for (index, resource_suffix) in TONE_RESOURCE_SUFFIXES.into_iter().enumerate() {
        let resource_name = format!("system_{palette_name}_{resource_suffix}");
        let resource_id = read_color_resource_id(env, color_class, &resource_name)?;
        tones[index] = read_color(env, resources, resource_id, &resource_name)?;
    }

    Ok(TonalPalette::new(tones))
}

fn ensure_supported_sdk(env: &mut jni::Env<'_>) -> Result<(), String> {
    let version_class = env
        .find_class(jni_str!("android/os/Build$VERSION"))
        .map_err(|error| format!("failed to find android.os.Build.VERSION: {error}"))?;
    let sdk = env
        .get_static_field(version_class, jni_str!("SDK_INT"), jni_sig!(int))
        .and_then(|value| value.i())
        .map_err(|error| format!("failed to read android.os.Build.VERSION.SDK_INT: {error}"))?;

    if sdk < MIN_ANDROID_SDK {
        return Err(format!(
            "system accent colors require Android 12 (API {MIN_ANDROID_SDK}) or newer; device reports API {sdk}"
        ));
    }

    Ok(())
}

fn find_color_class<'local>(env: &mut jni::Env<'local>) -> Result<JClass<'local>, String> {
    env.find_class(jni_str!("android/R$color"))
        .map_err(|error| format!("failed to find android.R.color: {error}"))
}

fn system_resources<'local>(env: &mut jni::Env<'local>) -> Result<JObject<'local>, String> {
    env.call_static_method(
        jni_str!("android/content/res/Resources"),
        jni_str!("getSystem"),
        jni_sig!("()Landroid/content/res/Resources;"),
        &[],
    )
    .and_then(|value| value.l())
    .map_err(|error| format!("failed to get Android system resources: {error}"))
}

fn read_color_resource_id(
    env: &mut jni::Env<'_>,
    color_class: &JClass<'_>,
    resource_name: &str,
) -> Result<i32, String> {
    let field_name = JNIString::new(resource_name);
    env.get_static_field(color_class, field_name.borrowed(), jni_sig!(int))
        .and_then(|value| value.i())
        .map_err(|error| format!("failed to read android.R.color.{resource_name}: {error}"))
}

fn read_color(
    env: &mut jni::Env<'_>,
    resources: &JObject<'_>,
    resource_id: i32,
    resource_name: &str,
) -> Result<AccentColor, String> {
    let argb = env
        .call_method(
            resources,
            jni_str!("getColor"),
            jni_sig!("(I)I"),
            &[JValue::Int(resource_id)],
        )
        .and_then(|value| value.i())
        .map_err(|error| format!("failed to resolve android.R.color.{resource_name}: {error}"))?;

    Ok(AccentColor::from_argb(argb as u32))
}
