use crate::error::NiaServerError;
use crate::protocol::{DeviceInfo, NiaGetDeviceInfoRequest};

#[derive(Debug, Clone)]
pub struct NiaGetDeviceInfoResponse {
    device_info_result: Result<DeviceInfo, NiaServerError>,
}

impl NiaGetDeviceInfoResponse {
    pub fn from(
        nia_get_device_info_request: NiaGetDeviceInfoRequest,
    ) -> NiaGetDeviceInfoResponse {
        let device_path = nia_get_device_info_request.get_device_path();

        let device_info_result = crate::utils::get_device_info(&device_path)
            .map_err(|_| NiaServerError::unknown(""));

        NiaGetDeviceInfoResponse { device_info_result }
    }
}

impl From<NiaGetDeviceInfoResponse>
    for nia_protocol_rust::GetDeviceInfoResponse
{
    fn from(nia_get_device_info_response: NiaGetDeviceInfoResponse) -> Self {
        let device_info = nia_get_device_info_response.device_info_result;

        let mut get_device_info_response =
            nia_protocol_rust::GetDeviceInfoResponse::new();

        match device_info {
            Ok(device_info) => {
                let mut success_result =
                    nia_protocol_rust::GetDeviceInfoResponse_SuccessResult::new(
                    );

                success_result.set_device(protobuf::Chars::from(String::from(
                    device_info.get_path(),
                )));
                success_result.set_name(protobuf::Chars::from(String::from(
                    device_info.get_name(),
                )));
                success_result.set_model(protobuf::Chars::from(String::from(
                    device_info.get_model(),
                )));

                get_device_info_response.set_success_result(success_result)
            }
            Err(error) => {
                let message = error.get_message();
                let mut error_result =
                    nia_protocol_rust::GetDeviceInfoResponse_ErrorResult::new();

                error_result.set_message(protobuf::Chars::from(format!(
                    "Cannot get device info: {}.",
                    message
                )));

                get_device_info_response.set_error_result(error_result)
            }
        }

        get_device_info_response
    }
}
