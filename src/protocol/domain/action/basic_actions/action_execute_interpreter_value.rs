use crate::error::{from_protobuf_error, NiaServerError, NiaServerResult};
use crate::protocol::Serializable;
use protobuf::Message;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionExecuteInterpreterValue {}

impl ActionExecuteInterpreterValue {
    pub fn new() -> ActionExecuteInterpreterValue {
        ActionExecuteInterpreterValue {}
    }
}

impl
    Serializable<
        ActionExecuteInterpreterValue,
        nia_protocol_rust::ActionExecuteInterpreterValue,
    > for ActionExecuteInterpreterValue
{
    fn to_pb(&self) -> nia_protocol_rust::ActionExecuteInterpreterValue {
        let mut action_execute_interpreter_value_pb =
            nia_protocol_rust::ActionExecuteInterpreterValue::new();

        action_execute_interpreter_value_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ActionExecuteInterpreterValue,
    ) -> NiaServerResult<ActionExecuteInterpreterValue> {
        let action_execute_interpreter_value =
            ActionExecuteInterpreterValue::new();

        Ok(action_execute_interpreter_value)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializable_and_deserializable() {
        let action_execute_function = ActionExecuteInterpreterValue::new();
        let bytes = action_execute_function.to_bytes().unwrap();
        let action_execute_function =
            ActionExecuteInterpreterValue::from_bytes(bytes).unwrap();

        let result = action_execute_function.action_name;

        assert_eq!(expected, result)
    }
}
