use crate::error::{from_protobuf_error, NiaServerError, NiaServerResult};
use crate::protocol::Serializable;
use protobuf::Message;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionExecuteCode {
    code: String,
}

impl ActionExecuteCode {
    pub fn new<S>(code: S) -> ActionExecuteCode
    where
        S: Into<String>,
    {
        ActionExecuteCode { code: code.into() }
    }

    pub fn get_code(&self) -> &str {
        &self.code
    }
}

impl Serializable<ActionExecuteCode, nia_protocol_rust::ActionExecuteCode>
    for ActionExecuteCode
{
    fn to_pb(&self) -> nia_protocol_rust::ActionExecuteCode {
        let mut action_execute_code_pb =
            nia_protocol_rust::ActionExecuteCode::new();

        action_execute_code_pb
            .set_code(protobuf::Chars::from(self.code.clone()));

        action_execute_code_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ActionExecuteCode,
    ) -> NiaServerResult<ActionExecuteCode> {
        let action_execute_code =
            ActionExecuteCode::new(String::from(object_pb.get_code()));

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

        let action_wait = ActionExecuteCode::new(expected);
        let bytes = action_wait.to_bytes().unwrap();
        let action_wait = ActionExecuteCode::from_bytes(bytes).unwrap();

        let result = action_wait.code;

        assert_eq!(expected, result)
    }
}
