use crate::error::{from_protobuf_error, NiaServerError};
use crate::protocol::Serializable;
use protobuf::Message;

#[derive(Clone, Copy, Debug)]
pub struct ActionMouseButtonClick {
    button_code: i32,
}

impl ActionMouseButtonClick {
    pub fn new(button_code: i32) -> ActionMouseButtonClick {
        ActionMouseButtonClick { button_code }
    }

    pub fn get_button_code(&self) -> i32 {
        self.button_code
    }
}

impl
    Serializable<
        ActionMouseButtonClick,
        nia_protocol_rust::ActionMouseButtonClick,
    > for ActionMouseButtonClick
{
    fn to_pb(&self) -> nia_protocol_rust::ActionMouseButtonClick {
        let mut action_mouse_button_click_pb =
            nia_protocol_rust::ActionMouseButtonClick::new();

        action_mouse_button_click_pb.set_button_code(self.button_code);

        action_mouse_button_click_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ActionMouseButtonClick,
    ) -> ActionMouseButtonClick {
        let action_mouse_button_click =
            ActionMouseButtonClick::new(object_pb.get_button_code());

        action_mouse_button_click
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializable_and_deserializable() {
        let expected = 123;

        let action_mouse_button_click = ActionMouseButtonClick::new(expected);
        let bytes = action_mouse_button_click.to_bytes().unwrap();
        let action_mouse_button_click =
            ActionMouseButtonClick::from_bytes(bytes).unwrap();

        let result = action_mouse_button_click.button_code;

        assert_eq!(expected, result)
    }
}
