use std::convert::TryFrom;

use crate::error::NiaServerError;

use crate::protocol::GetRequestType;
use crate::protocol::RequestType;

#[derive(Debug, Clone)]
pub struct NiaGetDeviceInfoRequest {
    device_path: String,
}

impl NiaGetDeviceInfoRequest {
    pub fn new<S>(device_path: S) -> NiaGetDeviceInfoRequest
    where
        S: Into<String>,
    {
        NiaGetDeviceInfoRequest {
            device_path: device_path.into(),
        }
    }

    pub fn get_device_path(self) -> String {
        self.device_path
    }
}

impl TryFrom<nia_protocol_rust::GetDeviceInfoRequest>
    for NiaGetDeviceInfoRequest
{
    type Error = NiaServerError;

    fn try_from(
        get_device_info_request: nia_protocol_rust::GetDeviceInfoRequest,
    ) -> Result<Self, Self::Error> {
        let device_path = get_device_info_request.get_device();

        Ok(NiaGetDeviceInfoRequest::new(device_path))
    }
}
