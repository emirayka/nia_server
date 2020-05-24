use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::{NiaAction, Serializable};
use crate::protocol::{NiaActionEnum, NiaConvertable};

use crate::protocol::domain::action::basic_actions::*;
use nia_interpreter_core::NamedAction;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NiaNamedAction {
    action: NiaAction,
    action_name: String,
}

impl NiaNamedAction {
    pub fn new<S>(action: NiaAction, action_name: S) -> NiaNamedAction
    where
        S: Into<String>,
    {
        NiaNamedAction {
            action,
            action_name: action_name.into(),
        }
    }

    pub fn get_action(&self) -> &NiaAction {
        &self.action
    }

    pub fn get_action_name(&self) -> &String {
        &self.action_name
    }
}

impl NiaConvertable<NiaNamedAction, nia_interpreter_core::NamedAction>
    for NiaNamedAction
{
    fn to_interpreter_repr(&self) -> nia_interpreter_core::NamedAction {
        let interpreter_action = self.action.to_interpreter_repr();
        let action_name = self.action_name.clone();

        nia_interpreter_core::NamedAction::new(interpreter_action, action_name)
    }

    fn from_interpreter_repr(
        object_pb: &nia_interpreter_core::NamedAction,
    ) -> NiaServerResult<NiaNamedAction> {
        let action = NiaAction::from_interpreter_repr(object_pb.get_action())?;
        let action_name = object_pb.get_action_name().clone();

        let named_action = NiaNamedAction::new(action, action_name);

        Ok(named_action)
    }
}

impl Serializable<NiaNamedAction, nia_protocol_rust::NamedAction>
    for NiaNamedAction
{
    fn to_pb(&self) -> nia_protocol_rust::NamedAction {
        let action_pb = self.action.to_pb();
        let action_name_pb =
            protobuf::Chars::from(String::from(&self.action_name));

        let mut named_action_pb = nia_protocol_rust::NamedAction::new();

        named_action_pb.set_action(action_pb);
        named_action_pb.set_action_name(action_name_pb);

        named_action_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::NamedAction,
    ) -> NiaServerResult<NiaNamedAction> {
        let mut object_pb = object_pb;

        let action = NiaAction::from_pb(object_pb.take_action())?;
        let action_name = object_pb.take_action_name().to_string();

        let named_action = NiaNamedAction::new(action, action_name);

        Ok(named_action)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(test)]
    mod serializable {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn serializes_and_deserializes() {
            let expected = NiaNamedAction::new(
                NiaAction::new(NiaActionEnum::TextType(ActionTextType::new(
                    "test",
                ))),
                String::from("type-test"),
            );

            let bytes = expected.to_bytes().unwrap();
            let result = NiaNamedAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, result);
        }
    }

    #[cfg(test)]
    mod convertable {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn converts_between_server_and_interpreter_representations() {
            let expected = NiaNamedAction::new(
                NiaAction::new(NiaActionEnum::TextType(ActionTextType::new(
                    "test",
                ))),
                String::from("type-test"),
            );

            let interpreter_named_action = expected.to_interpreter_repr();
            let result = NiaNamedAction::from_interpreter_repr(
                &interpreter_named_action,
            )
            .unwrap();

            assert_eq!(expected, result);
        }
    }
}
