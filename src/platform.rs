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
pub trait Platform {
    /// Get human-readable information about this platform
    fn meta_info(&self) -> &PlatformMetaInfo;

    /// Return the types of inputs that devices on this platform accept
    fn control_inputs(&self) -> &[Input];
}
