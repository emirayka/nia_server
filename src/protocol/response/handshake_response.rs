use crate::error::{NiaServerError, NiaServerResult};
use crate::protocol::{NiaHandshakeRequest, Serializable};
use nia_protocol_rust::HandshakeResponse;

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

impl Serializable<NiaHandshakeResponse, nia_protocol_rust::HandshakeResponse>
    for NiaHandshakeResponse
{
    fn to_pb(&self) -> HandshakeResponse {
        let (version, info) = (self.version.clone(), self.info.clone());

        let mut handshake_response =
            nia_protocol_rust::HandshakeResponse::new();

        let mut success_result =
            nia_protocol_rust::HandshakeResponse_SuccessResult::new();

        success_result.set_info(protobuf::Chars::from(info));
        success_result.set_version(protobuf::Chars::from(version));

        handshake_response.set_success_result(success_result);

        handshake_response
    }

    fn from_pb(
        object_pb: HandshakeResponse,
    ) -> NiaServerResult<NiaHandshakeResponse> {
        unreachable!()
    }
}
