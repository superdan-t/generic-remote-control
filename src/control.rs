//! Contains interfaces for input and output controls for remote devices.

use std::any::TypeId;

/// Represents a type of value provided as an input to control a remote device.
pub trait InputType {

    /// Get the name and other unique info for this input type
    fn get_id(&self) -> &Identifier;

    /// Get the types of parameters that this input type expects
    fn get_parameters(&self) -> [TypeId];
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


#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use std::{any::TypeId, collections::BTreeMap};

    use super::{Identifier};

    struct BasicInputType {
        id: Identifier<'static>,
        parameters: Vec<TypeId>,
    }

    // Creates a map of inputs; mostly a compile-type design check
    #[test]
    fn collection_of_inputs() {
        let send_coordinates = BasicInputType {
            id: Identifier { name: "control.coordinates", display_name: "Coordinates", description: None },
            parameters: vec![TypeId::of::<f32>(); 2],
        };
        let send_speed = BasicInputType {
            id: Identifier { name: "control.speed", display_name: "Forward Movement Speed", description: None },
            parameters: vec![TypeId::of::<f32>(); 1],
        };

        let mut inputs = BTreeMap::new();

        inputs.insert(send_coordinates.id.name, send_coordinates);
        inputs.insert(send_speed.id.name, send_speed);

        inputs.get_key_value("control.speed").unwrap();
    }
}
