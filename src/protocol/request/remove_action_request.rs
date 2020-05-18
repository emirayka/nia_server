use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::GetRequestType;
use crate::protocol::NiaAction;
use crate::protocol::RequestType;
use crate::protocol::Serializable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaRemoveActionRequest {
    action_name: String,
}

impl NiaRemoveActionRequest {
    pub fn new<S>(action_name: S) -> NiaRemoveActionRequest
    where
        S: Into<String>,
    {
        NiaRemoveActionRequest {
            action_name: action_name.into(),
        }
    }

    pub fn take_action_name(self) -> String {
        self.action_name
    }
}

impl
    Serializable<NiaRemoveActionRequest, nia_protocol_rust::RemoveActionRequest>
    for NiaRemoveActionRequest
{
    fn to_pb(&self) -> nia_protocol_rust::RemoveActionRequest {
        let mut remove_action_request_pb =
            nia_protocol_rust::RemoveActionRequest::new();

        remove_action_request_pb
            .set_action_name(protobuf::Chars::from(self.action_name.clone()));

        remove_action_request_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::RemoveActionRequest,
    ) -> NiaServerResult<NiaRemoveActionRequest> {
        let mut object_pb = object_pb;

        let action_name = object_pb.take_action_name().to_string();

        Ok(NiaRemoveActionRequest::new(action_name))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::protocol::ActionExecuteOSCommand;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaRemoveActionRequest::new("dev");

        let bytes = expected.to_bytes().unwrap();
        let result = NiaRemoveActionRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
