use std::convert::TryFrom;

use crate::protocol::Serializable;

use nia_protocol_rust::GetDefinedActionsRequest;

use crate::error::NiaServerError;
use crate::error::NiaServerResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaGetDefinedActionsRequest {}

impl NiaGetDefinedActionsRequest {
    pub fn new() -> NiaGetDefinedActionsRequest {
        NiaGetDefinedActionsRequest {}
    }
}

impl TryFrom<nia_protocol_rust::GetDefinedActionsRequest>
    for NiaGetDefinedActionsRequest
{
    type Error = NiaServerError;

    fn try_from(
        _get_devices_request: nia_protocol_rust::GetDefinedActionsRequest,
    ) -> Result<Self, Self::Error> {
        Ok(NiaGetDefinedActionsRequest::new())
    }
}

impl
    Serializable<
        NiaGetDefinedActionsRequest,
        nia_protocol_rust::GetDefinedActionsRequest,
    > for NiaGetDefinedActionsRequest
{
    fn to_pb(&self) -> nia_protocol_rust::GetDefinedActionsRequest {
        let mut get_defined_actions_request_pb =
            nia_protocol_rust::GetDefinedActionsRequest::new();

        get_defined_actions_request_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::GetDefinedActionsRequest,
    ) -> NiaServerResult<NiaGetDefinedActionsRequest> {
        Ok(NiaGetDefinedActionsRequest::new())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaGetDefinedActionsRequest::new();

        let bytes = expected.to_bytes().unwrap();
        let result = NiaGetDefinedActionsRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
