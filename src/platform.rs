//! The platform module provides the interfaces used to describe an operable platform.
//!
//! Platform backends implement these interfaces, while frontends use these interfaces to get
//! information about available platforms.

use crate::control::Input;

/// Provides information about a supported platform that can be shown in UIs. None of these values are used for internal functionality.
pub struct PlatformMetaInfo<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub description: &'a str,
}

/// The Platform trait provides information about controlling one type of remote device (a platform)
///
/// **This trait may be changed to a struct:** The defining info for each type of platform will
/// likely just be returning const data, so it doesn't make sense to require a separate
/// impl when it can all be the same struct.
pub trait Platform {
    /// Get human-readable information about this platform
    fn meta_info(&self) -> &PlatformMetaInfo;

    /// Return the types of inputs that devices on this platform accept
    fn control_inputs(&self) -> &[Input];
}

#[cfg(test)]
mod tests {
    use crate::control::Input;

    use super::{Platform, PlatformMetaInfo};

    struct ExamplePlatform {}

    impl ExamplePlatform {
        const META_INFO: PlatformMetaInfo<'_> = PlatformMetaInfo {
            name: "Example Platform",
            version: "0.0.1b",
            description: "This is an example of a platform type. It isn't very useful.",
        };
        const INPUTS: [Input; 0] = [];
    }

    impl Platform for ExamplePlatform {
        fn meta_info(&self) -> &PlatformMetaInfo {
            &ExamplePlatform::META_INFO
        }

        fn control_inputs(&self) -> &[Input] {
            &ExamplePlatform::INPUTS
        }
    }

    /// Tests initialization and makes sure some variables can be accessed as intended
    #[test]
    fn platform_init_and_lifetime() {
        let my_platform = ExamplePlatform {};

        let info = my_platform.meta_info();
        assert_eq!("Example Platform", info.name);

        assert!(my_platform.control_inputs().is_empty());
    }
}
