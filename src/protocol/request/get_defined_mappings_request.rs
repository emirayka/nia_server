use std::convert::TryFrom;

use crate::protocol::Serializable;

use nia_protocol_rust::GetDefinedMappingsRequest;

use crate::error::NiaServerError;
use crate::error::NiaServerResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaGetDefinedMappingsRequest {}

impl NiaGetDefinedMappingsRequest {
    pub fn new() -> NiaGetDefinedMappingsRequest {
        NiaGetDefinedMappingsRequest {}
    }
}

impl TryFrom<nia_protocol_rust::GetDefinedMappingsRequest>
    for NiaGetDefinedMappingsRequest
{
    type Error = NiaServerError;

    fn try_from(
        _get_devices_request: nia_protocol_rust::GetDefinedMappingsRequest,
    ) -> Result<Self, Self::Error> {
        Ok(NiaGetDefinedMappingsRequest::new())
    }
}

impl
    Serializable<
        NiaGetDefinedMappingsRequest,
        nia_protocol_rust::GetDefinedMappingsRequest,
    > for NiaGetDefinedMappingsRequest
{
    fn to_pb(&self) -> nia_protocol_rust::GetDefinedMappingsRequest {
        let mut get_defined_mappings_request_pb =
            nia_protocol_rust::GetDefinedMappingsRequest::new();

        get_defined_mappings_request_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::GetDefinedMappingsRequest,
    ) -> NiaServerResult<NiaGetDefinedMappingsRequest> {
        Ok(NiaGetDefinedMappingsRequest::new())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaGetDefinedMappingsRequest::new();

        let bytes = expected.to_bytes().unwrap();
        let result = NiaGetDefinedMappingsRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
