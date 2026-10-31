use std::fmt;

/// An error returned while querying the system accent color.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub enum Error {
    /// The current target does not have a supported accent-color backend.
    UnsupportedPlatform {
        /// The target reported by [`std::env::consts::OS`].
        platform: &'static str,
    },
    /// A supported platform backend could not provide a color.
    Query {
        /// The platform backend that failed.
        platform: &'static str,
        /// The operation that failed.
        operation: &'static str,
        /// Actionable details from the backend.
        detail: String,
    },
}

impl Error {
    pub(crate) fn query(platform: &'static str, operation: &'static str, detail: String) -> Self {
        Self::Query {
            platform,
            operation,
            detail,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPlatform { platform } => {
                write!(
                    formatter,
                    "system accent color is not supported on {platform}"
                )
            }
            Self::Query {
                platform,
                operation,
                detail,
            } => write!(
                formatter,
                "{platform} accent color query failed during {operation}: {detail}"
            ),
        }
    }
}

impl std::error::Error for Error {}
