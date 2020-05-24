use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::Serializable;
use nia_protocol_rust::StopListeningRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaStopListeningRequest {}

impl NiaStopListeningRequest {
    pub fn new() -> NiaStopListeningRequest {
        NiaStopListeningRequest {}
    }
}

impl
    Serializable<
        NiaStopListeningRequest,
        nia_protocol_rust::StopListeningRequest,
    > for NiaStopListeningRequest
{
    fn to_pb(&self) -> StopListeningRequest {
        let mut stop_listening_request_pb =
            nia_protocol_rust::StopListeningRequest::new();

        stop_listening_request_pb
    }

    fn from_pb(
        object_pb: StopListeningRequest,
    ) -> NiaServerResult<NiaStopListeningRequest> {
        let mut stop_listening_request = NiaStopListeningRequest::new();

        Ok(stop_listening_request)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaStopListeningRequest::new();

        let bytes = expected.to_bytes().unwrap();
        let result = NiaStopListeningRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
