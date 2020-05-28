use crate::error::{from_protobuf_error, NiaServerError, NiaServerResult};
use crate::protocol::Serializable;
use protobuf::Message;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionExecuteNamedAction {
    action_name: String,
}

impl ActionExecuteNamedAction {
    pub fn new<S>(action_name: S) -> ActionExecuteNamedAction
    where
        S: Into<String>,
    {
        ActionExecuteNamedAction {
            action_name: action_name.into(),
        }
    }

    pub fn get_action_name(&self) -> &String {
        &self.action_name
    }
}

impl
    Serializable<
        ActionExecuteNamedAction,
        nia_protocol_rust::ActionExecuteNamedAction,
    > for ActionExecuteNamedAction
{
    fn to_pb(&self) -> nia_protocol_rust::ActionExecuteNamedAction {
        let mut action_execute_named_action_pb =
            nia_protocol_rust::ActionExecuteNamedAction::new();

        action_execute_named_action_pb.set_action_name(protobuf::Chars::from(
            String::from(&self.action_name),
        ));

        action_execute_named_action_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ActionExecuteNamedAction,
    ) -> NiaServerResult<ActionExecuteNamedAction> {
        let action_execute_named_action = ActionExecuteNamedAction::new(
            String::from(object_pb.get_action_name()),
        );

        Ok(action_execute_named_action)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializable_and_deserializable() {
        let expected = "let a = 1;";

        let action_execute_function = ActionExecuteNamedAction::new(expected);
        let bytes = action_execute_function.to_bytes().unwrap();
        let action_execute_function =
            ActionExecuteNamedAction::from_bytes(bytes).unwrap();

        let result = action_execute_function.action_name;

        assert_eq!(expected, result)
    }
}
