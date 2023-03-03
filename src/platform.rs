//! The platform module provides the interfaces used to describe an operable platform.
//!
//! Platform backends implement these interfaces, while frontends use these interfaces to get
//! information about available platforms.

/// Provides information about a supported platform that can be shown in UIs. None of these values are used for internal functionality.
pub struct PlatformMetaInfo<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub description: &'a str,
}

/// The Platform struct provides information about controlling one type of remote device (a platform)
///
/// This struct only provides the information necessary for a UI to start interfacing with a remote device. It does not
/// represent a session/link to an active device. This information is generally static since the interface won't change
/// at runtime.
pub struct Platform {
    pub meta_info: PlatformMetaInfo<'static>,
}

#[cfg(test)]
mod tests {
    use super::{Platform, PlatformMetaInfo};

    /// The real test isn't at runtime: this is really a design/compile-time test to make sure the interfaces makes
    /// sense and can be used as intended
    #[test]
    fn platform_init_and_lifetime() {
        const PLATFORM_A: Platform = Platform {
            meta_info: PlatformMetaInfo {
                name: "Example Platform A",
                version: "0.0.1a",
                description: "",
            },
        };

        // Members should be accessible
        assert_eq!("Example Platform A", PLATFORM_A.meta_info.name);
    }
}
