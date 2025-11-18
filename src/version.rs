//! Types and functions related to the JSON format version.

use std::{
    error::Error,
    fmt,
    num::{NonZero, ParseIntError},
    ops::RangeInclusive,
    str::FromStr,
};

use anyhow::Context;
use serde::Deserialize;

/// A range of supported format versions.
///
/// `migrate_rustdoc_types` is able to (de)serialize all format versions in this range, and is able
/// to migrate the lower bound all the way to the upper bound.
const SUPPORTED_VERSIONS: RangeInclusive<u32> = 41..=48;

/// Represents a format version supported by `migrate_rustdoc_types`.
///
/// This will only represent format versions that can be read and migrated, as governed by
/// [`SUPPORTED_VERSIONS`]. To get the number from this enum, call [`Self::format_version()`].
#[derive(Clone, Copy, Debug)]
pub enum ToVersion {
    Latest,
    // We can use `NonZero` here because the first format version was 1.
    Specific(NonZero<u32>),
}

impl ToVersion {
    pub fn format_version(&self) -> u32 {
        match self {
            Self::Latest => *SUPPORTED_VERSIONS.end(),
            Self::Specific(format_version) => format_version.get(),
        }
    }
}

impl FromStr for ToVersion {
    type Err = ParseToVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "latest" {
            return Ok(Self::Latest);
        }

        let n = NonZero::<u32>::from_str(s).map_err(ParseToVersionError::ParseInt)?;

        if SUPPORTED_VERSIONS.contains(&n.get()) {
            Ok(Self::Specific(n))
        } else {
            Err(ParseToVersionError::UnsupportedVersion(n))
        }
    }
}

/// Represents an error while parsing [`ToVersion`] from a string.
#[derive(Debug)]
pub enum ParseToVersionError {
    ParseInt(ParseIntError),
    UnsupportedVersion(NonZero<u32>),
}

impl fmt::Display for ParseToVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseInt(error) => error.fmt(f),
            Self::UnsupportedVersion(format_version) => write!(
                f,
                "format version {format_version} outside of supported range {SUPPORTED_VERSIONS:?}",
            ),
        }
    }
}

impl Error for ParseToVersionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParseInt(error) => Some(error),
            Self::UnsupportedVersion(_) => None,
        }
    }
}

/// Detects the format version from Rustdoc's JSON output.
pub fn detect_version(json: &str) -> anyhow::Result<u32> {
    /// A simplified version of `rustdoc_types::Crate` that only has the `format_version` field.
    ///
    /// This type is intended to be independent of the format version so that it may deserialize
    /// any version of Rustdoc's JSON output.
    #[derive(Deserialize)]
    struct Crate {
        pub format_version: u32,
    }

    let Crate { format_version } =
        serde_json::from_str(json).context("failed to read format version of JSON")?;

    Ok(format_version)
}
