use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommandResult;

use nia_protocol_rust::*;

pub fn make_handshake_response() -> Response {
    let mut success_response = HandshakeResponse_SuccessResult::new();

    let mut handshake_response = HandshakeResponse::new();
    handshake_response.set_success_result(success_response);

    let mut response = Response::new();

    response.set_handshake_response(handshake_response);
    response
}

pub fn make_get_devices_response() -> Response {
    let mut get_devices_response = GetDevicesResponse::new();
    let devices = crate::utils::get_devices();

    match devices {
        Ok(devices) => {
            let devices = devices
                .into_iter()
                .map(|s: String| protobuf::Chars::from(s))
                .collect();

            let mut success_result = GetDevicesResponse_SuccessResult::new();

            success_result.set_devices(devices);
            get_devices_response.set_success_result(success_result);
        }
        Err(_) => {
            let mut error_result = GetDevicesResponse_ErrorResult::new();

            error_result.set_message(protobuf::Chars::from(String::from(
                "Cannot get device list.",
            )));
            get_devices_response.set_error_result(error_result);
        }
    }

    let mut response = Response::new();

    response.set_get_devices_response(get_devices_response);
    response
}

pub fn make_get_device_info_response(
    request: GetDeviceInfoRequest,
) -> Response {
    let device_path = request.get_device();
    let device_info = crate::utils::get_device_info(device_path);

    let mut get_device_info_response = GetDeviceInfoResponse::new();

    match device_info {
        Ok(device_info) => {
            let mut success_result = GetDeviceInfoResponse_SuccessResult::new();

            success_result
                .set_device(protobuf::Chars::from(String::from(device_path)));
            success_result.set_name(protobuf::Chars::from(String::from(
                device_info.get_name(),
            )));
            success_result.set_model(protobuf::Chars::from(String::from(
                device_info.get_model(),
            )));

            get_device_info_response.set_success_result(success_result)
        }
        Err(_) => {
            let mut error_result = GetDeviceInfoResponse_ErrorResult::new();

            error_result.set_message(protobuf::Chars::from(String::from(
                "Cannot get device info.",
            )));

            get_device_info_response.set_error_result(error_result)
        }
    }

    let mut response = Response::new();

    response.set_get_device_info_response(get_device_info_response);
    response
}
