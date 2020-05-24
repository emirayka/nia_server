use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::Serializable;
use nia_protocol_rust::GetDevicesRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaGetDevicesRequest {}

impl NiaGetDevicesRequest {
    pub fn new() -> NiaGetDevicesRequest {
        NiaGetDevicesRequest {}
    }
}

impl Serializable<NiaGetDevicesRequest, nia_protocol_rust::GetDevicesRequest>
    for NiaGetDevicesRequest
{
    fn to_pb(&self) -> GetDevicesRequest {
        nia_protocol_rust::GetDevicesRequest::new()
    }

    fn from_pb(
        object_pb: GetDevicesRequest,
    ) -> NiaServerResult<NiaGetDevicesRequest> {
        Ok(NiaGetDevicesRequest::new())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaGetDevicesRequest::new();

        let bytes = expected.to_bytes().unwrap();
        let result = NiaGetDevicesRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
