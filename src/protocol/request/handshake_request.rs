use crate::error::{NiaServerError, NiaServerResult};
use crate::protocol::Serializable;
use nia_protocol_rust::HandshakeRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaHandshakeRequest {}

impl NiaHandshakeRequest {
    pub fn new() -> NiaHandshakeRequest {
        NiaHandshakeRequest {}
    }
}

impl Serializable<NiaHandshakeRequest, nia_protocol_rust::HandshakeRequest>
    for NiaHandshakeRequest
{
    fn to_pb(&self) -> nia_protocol_rust::HandshakeRequest {
        nia_protocol_rust::HandshakeRequest::new()
    }

    fn from_pb(
        object_pb: nia_protocol_rust::HandshakeRequest,
    ) -> NiaServerResult<NiaHandshakeRequest> {
        Ok(NiaHandshakeRequest::new())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaHandshakeRequest::new();

        let bytes = expected.to_bytes().unwrap();
        let result = NiaHandshakeRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result)
    }
}
