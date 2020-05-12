use std::convert::TryFrom;

use crate::error::NiaServerError;

use crate::protocol::GetRequestType;
use crate::protocol::RequestType;

#[derive(Debug, Clone)]
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
