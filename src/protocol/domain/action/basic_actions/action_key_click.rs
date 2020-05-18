use crate::error::{from_protobuf_error, NiaServerError, NiaServerResult};
use crate::protocol::Serializable;
use protobuf::Message;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionKeyClick {
    key_code: i32,
}

impl ActionKeyClick {
    pub fn new(key_code: i32) -> ActionKeyClick {
        ActionKeyClick { key_code }
    }

    pub fn get_key_code(&self) -> i32 {
        self.key_code
    }
}

impl Serializable<ActionKeyClick, nia_protocol_rust::ActionKeyClick>
    for ActionKeyClick
{
    fn to_pb(&self) -> nia_protocol_rust::ActionKeyClick {
        let mut action_key_click_pb = nia_protocol_rust::ActionKeyClick::new();

        action_key_click_pb.set_key_code(self.get_key_code());

        action_key_click_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ActionKeyClick,
    ) -> NiaServerResult<ActionKeyClick> {
        let action_key_click = ActionKeyClick::new(object_pb.get_key_code());

        Ok(action_key_click)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializable_and_deserializable() {
        let expected = 123;

        let action_key_click = ActionKeyClick::new(expected);
        let bytes = action_key_click.to_bytes().unwrap();
        let action_key_click = ActionKeyClick::from_bytes(bytes).unwrap();

        let result = action_key_click.key_code;

        assert_eq!(expected, result)
    }
}
