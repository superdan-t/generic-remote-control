//! Contains interfaces for creating and managing live sessions with remote entities

use std::any::Any;

use crate::control::Input;

/// This trait is for objects representing a remote session
pub trait RemoteSession {
    /// Search for an input by name
    fn get_input(name: &str) -> Option<Box<dyn Input<dyn Any>>>;
}
