//! Contains interfaces for input and output controls for remote devices.

use std::any::{Any, TypeId};

/// Represents a type of value provided as an input to control a remote device.
pub trait InputType {
    /// Get the name and other unique info for this input type
    fn get_id(&self) -> &Identifier;

    /// Get the TypeId of parameters for this input type. Inputs may only accept one parameter,
    /// but the type may be a tuple or slice.
    fn get_parameter_typeid(&self) -> TypeId;
}

/// An identifier is used to name, identify, and compare specific inputs and outputs.
pub struct Identifier<'a> {
    /// The primary name is used internally for all comparisons between IDs
    pub name: &'a str,
    /// The display name is meant to be a user-friendly name for UIs to show, but has no functional impact on the ID
    pub display_name: &'a str,
    /// This can provide information about the input/output this ID represents that UIs may choose to show, but has no functional impact on the ID
    pub description: Option<&'a str>,
}

/// A dynamically-typed Input
pub struct AnyInput<'a>(&'a dyn Any);

/// A live input with a configurable value of a generic type
///
/// Inputs are a set of values used to control a remote device
///
/// InputType describes a type of input to distinguish it from other types, whereas an Input is an
/// live instance of some type of input (i.e. has some configurable value that can be changed)
pub trait Input<T> {
    /// Set the input to its default value
    ///
    /// This is important for user interfaces since input sources (such as a keyboard or gamepad)
    /// may physically disconnect from the system, and it will be necessary to reset the value to
    /// some default state after the value source is lost (Example: setting vehicle speed to 0
    /// when a gamepad disconnects, preventing loss-of-control accidents)
    fn restore_default(&mut self);

    /// Set the value
    fn set(&mut self, value: T);
}

impl Eq for Identifier<'_> {}

// Only compare Identifiers based on the name. The other fields are only for display
impl PartialEq for Identifier<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> AnyInput<'a> {
    pub fn new<T: 'static + Input<U>, U>(input: &'a T) -> AnyInput<'a> {
        Self(input)
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use std::{any::TypeId, collections::BTreeMap};

    use super::Identifier;

    struct BasicInputType {
        id: Identifier<'static>,
        parameters: Vec<TypeId>,
    }

    // Creates a map of inputs; mostly a compile-type design check
    #[test]
    fn collection_of_inputs() {
        let send_coordinates = BasicInputType {
            id: Identifier {
                name: "control.coordinates",
                display_name: "Coordinates",
                description: None,
            },
            parameters: vec![TypeId::of::<f32>(); 2],
        };
        let send_speed = BasicInputType {
            id: Identifier {
                name: "control.speed",
                display_name: "Forward Movement Speed",
                description: None,
            },
            parameters: vec![TypeId::of::<f32>(); 1],
        };

        let mut inputs = BTreeMap::new();

        inputs.insert(send_coordinates.id.name, send_coordinates);
        inputs.insert(send_speed.id.name, send_speed);

        inputs.get_key_value("control.speed").unwrap();
    }
}
