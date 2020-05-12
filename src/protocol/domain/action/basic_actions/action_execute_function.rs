use crate::error::{from_protobuf_error, NiaServerError};
use crate::protocol::Serializable;
use protobuf::Message;

#[derive(Clone, Debug)]
pub struct ActionExecuteFunction {
    function_name: String,
}

impl ActionExecuteFunction {
    pub fn new<S>(function_name: S) -> ActionExecuteFunction
    where
        S: Into<String>,
    {
        ActionExecuteFunction {
            function_name: function_name.into(),
        }
    }

    pub fn get_function_name(&self) -> &str {
        &self.function_name
    }
}

impl
    Serializable<
        ActionExecuteFunction,
        nia_protocol_rust::ActionExecuteFunction,
    > for ActionExecuteFunction
{
    fn to_pb(&self) -> nia_protocol_rust::ActionExecuteFunction {
        let mut action_execute_function_pb =
            nia_protocol_rust::ActionExecuteFunction::new();

        action_execute_function_pb.set_function_name(protobuf::Chars::from(
            self.function_name.clone(),
        ));

        action_execute_function_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ActionExecuteFunction,
    ) -> ActionExecuteFunction {
        let action_execute_function = ActionExecuteFunction::new(String::from(
            object_pb.get_function_name(),
        ));

        action_execute_function
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializable_and_deserializable() {
        let expected = "let a = 1;";

        let action_execute_function = ActionExecuteFunction::new(expected);
        let bytes = action_execute_function.to_bytes().unwrap();
        let action_execute_function =
            ActionExecuteFunction::from_bytes(bytes).unwrap();

        let result = action_execute_function.function_name;

        assert_eq!(expected, result)
    }
}
