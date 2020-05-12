use std::convert::TryFrom;

use crate::error::NiaServerError;

#[derive(Debug, Clone)]
pub struct NiaHandshakeRequest {}

impl NiaHandshakeRequest {
    pub fn new() -> NiaHandshakeRequest {
        NiaHandshakeRequest {}
    }
}

impl TryFrom<nia_protocol_rust::HandshakeRequest> for NiaHandshakeRequest {
    type Error = NiaServerError;

    fn try_from(
        _handshake_request: nia_protocol_rust::HandshakeRequest,
    ) -> Result<Self, Self::Error> {
        Ok(NiaHandshakeRequest::new())
    }
}
