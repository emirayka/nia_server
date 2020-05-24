use std::convert::TryFrom;

use nia_protocol_rust::GetDefinedModifiersRequest;

use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::Serializable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaGetDefinedModifiersRequest {}

impl NiaGetDefinedModifiersRequest {
    pub fn new() -> NiaGetDefinedModifiersRequest {
        NiaGetDefinedModifiersRequest {}
    }
}

impl TryFrom<nia_protocol_rust::GetDefinedModifiersRequest>
    for NiaGetDefinedModifiersRequest
{
    type Error = NiaServerError;

    fn try_from(
        _get_devices_request: nia_protocol_rust::GetDefinedModifiersRequest,
    ) -> Result<Self, Self::Error> {
        Ok(NiaGetDefinedModifiersRequest::new())
    }
}

impl
    Serializable<
        NiaGetDefinedModifiersRequest,
        nia_protocol_rust::GetDefinedModifiersRequest,
    > for NiaGetDefinedModifiersRequest
{
    fn to_pb(&self) -> nia_protocol_rust::GetDefinedModifiersRequest {
        let mut get_defined_modifiers_request_pb =
            nia_protocol_rust::GetDefinedModifiersRequest::new();

        get_defined_modifiers_request_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::GetDefinedModifiersRequest,
    ) -> NiaServerResult<NiaGetDefinedModifiersRequest> {
        Ok(NiaGetDefinedModifiersRequest::new())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaGetDefinedModifiersRequest::new();

        let bytes = expected.to_bytes().unwrap();
        let result = NiaGetDefinedModifiersRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
