use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::Serializable;
use nia_protocol_rust::StartListeningRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaStartListeningRequest {}

impl NiaStartListeningRequest {
    pub fn new() -> NiaStartListeningRequest {
        NiaStartListeningRequest {}
    }
}

impl
    Serializable<
        NiaStartListeningRequest,
        nia_protocol_rust::StartListeningRequest,
    > for NiaStartListeningRequest
{
    fn to_pb(&self) -> StartListeningRequest {
        let mut start_listening_request_pb =
            nia_protocol_rust::StartListeningRequest::new();

        start_listening_request_pb
    }

    fn from_pb(
        object_pb: StartListeningRequest,
    ) -> NiaServerResult<NiaStartListeningRequest> {
        let mut start_listening_request = NiaStartListeningRequest::new();

        Ok(start_listening_request)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaStartListeningRequest::new();

        let bytes = expected.to_bytes().unwrap();
        let result = NiaStartListeningRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
