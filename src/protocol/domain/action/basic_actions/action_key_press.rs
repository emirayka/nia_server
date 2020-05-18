use crate::error::{from_protobuf_error, NiaServerError, NiaServerResult};
use crate::protocol::Serializable;
use protobuf::Message;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionKeyPress {
    key_code: i32,
}

impl ActionKeyPress {
    pub fn new(key_code: i32) -> ActionKeyPress {
        ActionKeyPress { key_code }
    }

    pub fn get_key_code(&self) -> i32 {
        self.key_code
    }
}

impl Serializable<ActionKeyPress, nia_protocol_rust::ActionKeyPress>
    for ActionKeyPress
{
    fn to_pb(&self) -> nia_protocol_rust::ActionKeyPress {
        let mut action_key_press_pb = nia_protocol_rust::ActionKeyPress::new();

        action_key_press_pb.set_key_code(self.get_key_code());

        action_key_press_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ActionKeyPress,
    ) -> NiaServerResult<ActionKeyPress> {
        let action_key_press = ActionKeyPress::new(object_pb.get_key_code());

        Ok(action_key_press)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializable_and_deserializable() {
        let expected = 123;

        let action_key_press = ActionKeyPress::new(expected);
        let bytes = action_key_press.to_bytes().unwrap();
        let action_key_press = ActionKeyPress::from_bytes(bytes).unwrap();

        let result = action_key_press.key_code;

        assert_eq!(expected, result)
    }
}
