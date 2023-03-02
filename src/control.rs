//! Contains interfaces for input and output controls for remote devices.

use std::string::String;

/// An identifier is used to name, identify, and compare specific inputs and outputs.
pub struct Identifier {
    /// The primary name is used internally for all comparisons between IDs
    name: String,

    /// The display name is meant to be a user-friendly name for UIs to show, but has no functional impact on the ID
    display_name: String,

    /// This can provide information about the input/output this ID represents that UIs may choose to show, but has no functional impact on the ID
    description: Option<String>,
}

impl Identifier {
    pub fn new(name: &str, display_name: &str, description: Option<&str>) -> Identifier {
        Identifier {
            name: String::from(name),
            display_name: String::from(display_name),
            description: description.map(str::to_string),
        }
    }

    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

/// Represents a type of value provided as an input to control a remote device.
pub struct Input {
    pub id: Identifier,
}

#[cfg(test)]
mod tests {
    use super::Identifier;

    /// Shows creating an identifier with string slices and tests that the getters match what it was constructed with
    #[test]
    fn create_identifier() {
        let name = "example.tree.endpoint";
        let display_name = "Endpoint";
        let description = "This is an example of an ID that can describe a control object";
        let id = Identifier::new(name, display_name, Some(description));

        assert_eq!(name, id.get_name());
        assert_eq!(display_name, id.get_display_name());
        assert_eq!(description, id.get_description().unwrap());
    }
}
