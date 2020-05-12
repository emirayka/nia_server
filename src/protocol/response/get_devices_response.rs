use crate::error::NiaServerError;
use crate::protocol::NiaGetDevicesRequest;

#[derive(Debug, Clone)]
pub struct NiaGetDevicesResponse {
    devices_result: Result<Vec<String>, NiaServerError>,
}

impl NiaGetDevicesResponse {
    pub fn from(
        _nia_get_devices_request: NiaGetDevicesRequest,
    ) -> NiaGetDevicesResponse {
        let devices_result = crate::utils::get_devices();

        NiaGetDevicesResponse { devices_result }
    }
}

impl From<NiaGetDevicesResponse> for nia_protocol_rust::GetDevicesResponse {
    fn from(nia_get_devices_response: NiaGetDevicesResponse) -> Self {
        let devices_result = nia_get_devices_response.devices_result;
        let mut get_devices_response =
            nia_protocol_rust::GetDevicesResponse::new();

        match devices_result {
            Ok(devices) => {
                let devices = devices
                    .into_iter()
                    .map(|string| protobuf::Chars::from(string))
                    .collect();

                let mut success_result =
                    nia_protocol_rust::GetDevicesResponse_SuccessResult::new();

                success_result.set_devices(devices);

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
}
