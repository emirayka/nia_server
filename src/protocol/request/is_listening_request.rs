use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::Serializable;
use nia_protocol_rust::IsListeningRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaIsListeningRequest {}

impl NiaIsListeningRequest {
    pub fn new() -> NiaIsListeningRequest {
        NiaIsListeningRequest {}
    }
}

impl Serializable<NiaIsListeningRequest, nia_protocol_rust::IsListeningRequest>
    for NiaIsListeningRequest
{
    fn to_pb(&self) -> IsListeningRequest {
        let mut is_listening_request_pb =
            nia_protocol_rust::IsListeningRequest::new();

        is_listening_request_pb
    }

    fn from_pb(
        object_pb: IsListeningRequest,
    ) -> NiaServerResult<NiaIsListeningRequest> {
        let mut is_listening_request = NiaIsListeningRequest::new();

        Ok(is_listening_request)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaIsListeningRequest::new();

        let bytes = expected.to_bytes().unwrap();
        let result = NiaIsListeningRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
