use crate::error::{from_protobuf_error, NiaServerError, NiaServerResult};
use crate::protocol::Serializable;
use protobuf::Message;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionMouseButtonRelease {
    button_code: i32,
}

impl ActionMouseButtonRelease {
    pub fn new(button_code: i32) -> ActionMouseButtonRelease {
        ActionMouseButtonRelease { button_code }
    }

    pub fn get_button_code(&self) -> i32 {
        self.button_code
    }
}

impl
    Serializable<
        ActionMouseButtonRelease,
        nia_protocol_rust::ActionMouseButtonRelease,
    > for ActionMouseButtonRelease
{
    fn to_pb(&self) -> nia_protocol_rust::ActionMouseButtonRelease {
        let mut action_mouse_button_release_pb =
            nia_protocol_rust::ActionMouseButtonRelease::new();

        action_mouse_button_release_pb.set_button_code(self.button_code);

        action_mouse_button_release_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ActionMouseButtonRelease,
    ) -> NiaServerResult<ActionMouseButtonRelease> {
        let action_mouse_button_release =
            ActionMouseButtonRelease::new(object_pb.get_button_code());

        Ok(action_mouse_button_release)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializable_and_deserializable() {
        let expected = 123;

        let action_mouse_button_release =
            ActionMouseButtonRelease::new(expected);
        let bytes = action_mouse_button_release.to_bytes().unwrap();
        let action_mouse_button_release =
            ActionMouseButtonRelease::from_bytes(bytes).unwrap();

        let result = action_mouse_button_release.button_code;

        assert_eq!(expected, result)
    }
}
