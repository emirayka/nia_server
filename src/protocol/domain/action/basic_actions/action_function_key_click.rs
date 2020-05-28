use crate::error::{from_protobuf_error, NiaServerError, NiaServerResult};
use crate::protocol::Serializable;
use protobuf::Message;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionFunctionKeyClick {
    key_code: i32,
}

impl ActionFunctionKeyClick {
    pub fn new(key_code: i32) -> ActionFunctionKeyClick {
        ActionFunctionKeyClick { key_code }
    }

    pub fn get_key_code(&self) -> i32 {
        self.key_code
    }
}

impl
    Serializable<
        ActionFunctionKeyClick,
        nia_protocol_rust::ActionFunctionKeyClick,
    > for ActionFunctionKeyClick
{
    fn to_pb(&self) -> nia_protocol_rust::ActionFunctionKeyClick {
        let mut action_key_click_pb =
            nia_protocol_rust::ActionFunctionKeyClick::new();

        action_key_click_pb.set_key_code(self.get_key_code());

        action_key_click_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ActionFunctionKeyClick,
    ) -> NiaServerResult<ActionFunctionKeyClick> {
        let action_key_click =
            ActionFunctionKeyClick::new(object_pb.get_key_code());

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

        let action_key_click = ActionFunctionKeyClick::new(expected);
        let bytes = action_key_click.to_bytes().unwrap();
        let action_key_click =
            ActionFunctionKeyClick::from_bytes(bytes).unwrap();

        let result = action_key_click.key_code;

        assert_eq!(expected, result)
    }
}
