use crate::error::{NiaServerError, NiaServerResult};
use crate::protocol::{DeviceInfo, NiaGetDevicesRequest, Serializable};
use crate::server::Server;
use std::sync::MutexGuard;

#[derive(Debug, Clone)]
pub struct NiaGetDevicesResponse {
    devices_result: Result<Vec<DeviceInfo>, NiaServerError>,
}

impl NiaGetDevicesResponse {
    pub fn from(
        server: &mut Server,
        _nia_get_devices_request: NiaGetDevicesRequest,
    ) -> NiaGetDevicesResponse {
        let devices_result = server.get_devices().clone();

        NiaGetDevicesResponse {
            devices_result: Ok(devices_result),
        }
    }
}

impl Serializable<NiaGetDevicesResponse, nia_protocol_rust::GetDevicesResponse>
    for NiaGetDevicesResponse
{
    fn to_pb(&self) -> nia_protocol_rust::GetDevicesResponse {
        let devices_result = &self.devices_result;
        let mut get_devices_response =
            nia_protocol_rust::GetDevicesResponse::new();

        match devices_result {
            Ok(devices) => {
                let devices = devices
                    .into_iter()
                    .map(|device_info| device_info.to_pb())
                    .collect();

                let mut success_result =
                    nia_protocol_rust::GetDevicesResponse_SuccessResult::new();

                success_result.set_devices_info(devices);

                get_devices_response.set_success_result(success_result);
            }
            Err(error) => {
                let message = error.get_message();
                let mut error_result =
                    nia_protocol_rust::GetDevicesResponse_ErrorResult::new();

                error_result
                    .set_message(protobuf::Chars::from(String::from(message)));
                get_devices_response.set_error_result(error_result);
            }
        }

        get_devices_response
    }

    fn from_pb(
        object_pb: nia_protocol_rust::GetDevicesResponse,
    ) -> NiaServerResult<NiaGetDevicesResponse> {
        unreachable!()
    }
}
