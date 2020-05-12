use crate::error::{from_protobuf_error, NiaServerError};
use crate::protocol::Serializable;
use protobuf::Message;

#[derive(Clone, Copy, Debug)]
pub struct ActionMouseButtonPress {
    button_code: i32,
}

impl ActionMouseButtonPress {
    pub fn new(button_code: i32) -> ActionMouseButtonPress {
        ActionMouseButtonPress { button_code }
    }

    pub fn get_button_code(&self) -> i32 {
        self.button_code
    }
}

impl
    Serializable<
        ActionMouseButtonPress,
        nia_protocol_rust::ActionMouseButtonPress,
    > for ActionMouseButtonPress
{
    fn to_pb(&self) -> nia_protocol_rust::ActionMouseButtonPress {
        let mut action_mouse_button_press_pb =
            nia_protocol_rust::ActionMouseButtonPress::new();

        action_mouse_button_press_pb.set_button_code(self.button_code);

        action_mouse_button_press_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ActionMouseButtonPress,
    ) -> ActionMouseButtonPress {
        let action_mouse_button_press =
            ActionMouseButtonPress::new(object_pb.get_button_code());

        action_mouse_button_press
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializable_and_deserializable() {
        let expected = 123;

        let action_mouse_button_press = ActionMouseButtonPress::new(expected);
        let bytes = action_mouse_button_press.to_bytes().unwrap();
        let action_mouse_button_press =
            ActionMouseButtonPress::from_bytes(bytes).unwrap();

        let result = action_mouse_button_press.button_code;

        assert_eq!(expected, result)
    }
}
