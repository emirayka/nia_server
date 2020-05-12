use std::convert::TryFrom;

use crate::error::NiaServerError;

use crate::protocol::GetRequestType;
use crate::protocol::RequestType;

#[derive(Debug, Clone)]
pub struct NiaGetDevicesRequest {}

impl NiaGetDevicesRequest {
    pub fn new() -> NiaGetDevicesRequest {
        NiaGetDevicesRequest {}
    }
}

impl TryFrom<nia_protocol_rust::GetDevicesRequest> for NiaGetDevicesRequest {
    type Error = NiaServerError;

    fn try_from(
        _get_devices_request: nia_protocol_rust::GetDevicesRequest,
    ) -> Result<Self, Self::Error> {
        Ok(NiaGetDevicesRequest::new())
    }
}
