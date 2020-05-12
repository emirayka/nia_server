use crate::error::NiaServerError;
use crate::protocol::NiaHandshakeRequest;

const VERSION_MESSAGE: &'static str = "nia-server version '0.0.1'.";
const INFO_MESSAGE: &'static str = "Have a good day :3";

#[derive(Debug, Clone)]
pub struct NiaHandshakeResponse {
    version: String,
    info: String,
}

impl NiaHandshakeResponse {
    pub fn from(
        _nia_handshake_request: NiaHandshakeRequest,
    ) -> NiaHandshakeResponse {
        NiaHandshakeResponse {
            version: String::from(VERSION_MESSAGE),
            info: String::from(INFO_MESSAGE),
        }
    }
}

impl From<NiaHandshakeResponse> for nia_protocol_rust::HandshakeResponse {
    fn from(nia_handshake_response: NiaHandshakeResponse) -> Self {
        let (version, info) =
            (nia_handshake_response.version, nia_handshake_response.info);

        let mut handshake_response =
            nia_protocol_rust::HandshakeResponse::new();

        let mut success_result =
            nia_protocol_rust::HandshakeResponse_SuccessResult::new();

        success_result.set_info(protobuf::Chars::from(info));
        success_result.set_version(protobuf::Chars::from(version));

        handshake_response.set_success_result(success_result);

        handshake_response
    }
}
