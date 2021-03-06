use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::NiaAction;
use crate::protocol::NiaNamedAction;
use crate::protocol::Serializable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaDefineActionRequest {
    action: NiaNamedAction,
}

impl NiaDefineActionRequest {
    pub fn new(action: NiaNamedAction) -> NiaDefineActionRequest {
        NiaDefineActionRequest { action }
    }

    pub fn get_action(&self) -> &NiaNamedAction {
        &self.action
    }

    pub fn take_action(self) -> NiaNamedAction {
        self.action
    }
}

impl
    Serializable<NiaDefineActionRequest, nia_protocol_rust::DefineActionRequest>
    for NiaDefineActionRequest
{
    fn to_pb(&self) -> nia_protocol_rust::DefineActionRequest {
        let mut define_action_request_pb =
            nia_protocol_rust::DefineActionRequest::new();

        let action_pb = self.action.to_pb();
        define_action_request_pb.set_action(action_pb);

        define_action_request_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::DefineActionRequest,
    ) -> NiaServerResult<NiaDefineActionRequest> {
        let mut object_pb = object_pb;

        let action = NiaNamedAction::from_pb(object_pb.take_action())?;

        Ok(NiaDefineActionRequest::new(action))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::protocol::ActionExecuteOSCommand;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaDefineActionRequest::new(NiaNamedAction::new(
            NiaAction::new(ActionExecuteOSCommand::new("kek").into()),
            "execute kek",
        ));

        let bytes = expected.to_bytes().unwrap();
        let result = NiaDefineActionRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
