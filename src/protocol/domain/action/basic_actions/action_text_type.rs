use crate::error::{from_protobuf_error, NiaServerError, NiaServerResult};
use crate::protocol::Serializable;
use protobuf::Message;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionTextType {
    text: String,
}

impl ActionTextType {
    pub fn new<S>(text: S) -> ActionTextType
    where
        S: Into<String>,
    {
        ActionTextType { text: text.into() }
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }
}

impl Serializable<ActionTextType, nia_protocol_rust::ActionTextType>
    for ActionTextType
{
    fn to_pb(&self) -> nia_protocol_rust::ActionTextType {
        let mut action_execute_code_pb =
            nia_protocol_rust::ActionTextType::new();

        action_execute_code_pb
            .set_text(protobuf::Chars::from(self.text.clone()));

        action_execute_code_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ActionTextType,
    ) -> NiaServerResult<ActionTextType> {
        let action_execute_code =
            ActionTextType::new(String::from(object_pb.get_text()));

        Ok(action_execute_code)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializable_and_deserializable() {
        let expected = "let a = 1;";

        let action_wait = ActionTextType::new(expected);
        let bytes = action_wait.to_bytes().unwrap();
        let action_wait = ActionTextType::from_bytes(bytes).unwrap();

        let result = action_wait.text;

        assert_eq!(expected, result)
    }
}
