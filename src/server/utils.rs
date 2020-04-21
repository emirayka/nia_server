use nia_protocol_rust::*;
use nia_interpreter_core::Interpreter;
use nia_events::KeyboardId;
use nia_events::KeyId;

const VERSION_MESSAGE: &'static str = "nia-server version '0.0.1'";
const INFO_MESSAGE: &'static str = "Some not yet useful info";

pub fn parse_request_key_chord_part(request_key_chord_part: &KeyChordPart) -> nia_events::KeyChordPart {
    if request_key_chord_part.has_key_chord_part_1() {
        let request_key_chord_part_1 = request_key_chord_part.get_key_chord_part_1();
        let key_id = request_key_chord_part_1.get_key_id();

        nia_events::KeyChordPart::Key1(KeyId::new(key_id as u16))
    } else {
        let request_key_chord_part_2 = request_key_chord_part.get_key_chord_part_2();
        let keyboard_id = request_key_chord_part_2.get_keyboard_id();
        let key_id = request_key_chord_part_2.get_key_id();

        nia_events::KeyChordPart::Key2(
            KeyboardId::new(keyboard_id as u16),
            KeyId::new(key_id as u16),
        )
    }
}

pub fn parse_request_key_chord(request_key_chord: &KeyChord) -> nia_events::KeyChord {
    let mut modifier_key_chord_parts = Vec::new();
    let modifier_request_key_chord_parts = request_key_chord.get_modifiers();

    for modifier_request_key_chord_part in modifier_request_key_chord_parts {
        modifier_key_chord_parts.push(
            parse_request_key_chord_part(modifier_request_key_chord_part)
        );
    }

    let key_chord_part = parse_request_key_chord_part(
        request_key_chord.get_key()
    );

    nia_events::KeyChord::new(modifier_key_chord_parts, key_chord_part)
}

pub fn parse_request_key_chords(request_key_chords: &[KeyChord]) -> Vec<nia_events::KeyChord> {
    request_key_chords.iter()
        .map(parse_request_key_chord)
        .collect()
}

pub fn make_handshake_response() -> Response {
    let mut success_response = HandshakeResponse_SuccessResult::new();
    success_response.set_version(protobuf::Chars::from(String::from(VERSION_MESSAGE)));
    success_response.set_info(protobuf::Chars::from(String::from(INFO_MESSAGE)));

    let mut handshake_response = HandshakeResponse::new();
    handshake_response.set_success_result(success_response);

    let mut response = Response::new();

    response.set_handshake_response(handshake_response);
    response
}

pub fn make_get_devices_response() -> Response {
    let mut get_devices_response = GetDevicesResponse::new();
    let devices = crate::commands::get_devices();

    match devices {
        Ok(devices) => {
            let devices = devices.into_iter()
                .map(|s: String| {
                    protobuf::Chars::from(s)
                }).collect();

            let mut success_result = GetDevicesResponse_SuccessResult::new();

            success_result.set_devices(devices);
            get_devices_response.set_success_result(success_result);
        }
        Err(_) => {
            let mut error_result = GetDevicesResponse_ErrorResult::new();

            error_result.set_message(
                protobuf::Chars::from(String::from("Cannot get device list."))
            );
            get_devices_response.set_error_result(error_result);
        }
    }

    let mut response = Response::new();

    response.set_get_devices_response(get_devices_response);
    response
}

pub fn make_get_device_info_response(request: GetDeviceInfoRequest) -> Response {
    let device_path = request.get_device();
    let device_info = crate::commands::get_device_info(device_path);

    let mut get_device_info_response = GetDeviceInfoResponse::new();

    match device_info {
        Ok(device_info) => {
            let mut success_result = GetDeviceInfoResponse_SuccessResult::new();

            success_result.set_device(protobuf::Chars::from(String::from(device_path)));
            success_result.set_name(protobuf::Chars::from(String::from(device_info.get_name())));
            success_result.set_model(protobuf::Chars::from(String::from(device_info.get_model())));

            get_device_info_response.set_success_result(success_result)
        }
        Err(_) => {
            let mut error_result = GetDeviceInfoResponse_ErrorResult::new();

            error_result.set_message(protobuf::Chars::from(String::from(
                "Cannot get device info."
            )));

            get_device_info_response.set_error_result(error_result)
        }
    }

    let mut response = Response::new();

    response.set_get_device_info_response(get_device_info_response);
    response
}

pub fn make_execute_code_response(
    request: ExecuteCodeRequest,
    interpreter: &mut Interpreter
) -> Response {
    let code = request.get_code();
    let result = interpreter.execute(code);

    let mut execute_code_response = ExecuteCodeResponse::new();

    match result {
        Ok(value) => {
            let code_result = nia_interpreter_core::library::value_to_string(
                interpreter,
                value,
            );

            match code_result {
                Ok(s) => {
                    let mut success_result = ExecuteCodeResponse_SuccessResult::new();

                    success_result.set_execution_result(protobuf::Chars::from(s));

                    execute_code_response.set_success_result(success_result);
                }
                Err(error) => {
                    if error.is_failure() {
                        let mut failure_result = ExecuteCodeResponse_FailureResult::new();

                        failure_result.set_message(
                            protobuf::Chars::from(
                                format!("{}", error)
                            )
                        );

                        execute_code_response.set_failure_result(failure_result);
                    } else {
                        let mut error_result = ExecuteCodeResponse_ErrorResult::new();

                        error_result.set_message(
                            protobuf::Chars::from(
                                format!("{}", error)
                            )
                        );

                        execute_code_response.set_error_result(error_result);
                    }
                }
            }
        }
        Err(error) => {
            if error.is_failure() {
                let mut failure_result = ExecuteCodeResponse_FailureResult::new();

                failure_result.set_message(
                    protobuf::Chars::from(
                        format!("{}", error)
                    )
                );

                execute_code_response.set_failure_result(failure_result);
            } else {
                let mut error_result = ExecuteCodeResponse_ErrorResult::new();

                error_result.set_message(
                    protobuf::Chars::from(
                        format!("{}", error)
                    )
                );

                execute_code_response.set_error_result(error_result);
            }
        }
    }

    let mut response = Response::new();

    response.set_execute_code_response(execute_code_response);
    response
}

pub fn make_register_keyboard_response() -> Response {
    let success_result = RegisterKeyboardResponse_SuccessResult::new();

    let mut register_keyboard_response = RegisterKeyboardResponse::new();
    register_keyboard_response.set_success_result(success_result);

    let mut response = Response::new();
    response.set_register_keyboard_response(register_keyboard_response);

    response
}

pub fn make_define_modifier_response(success: bool) -> Response {
    let mut define_modifier_response = DefineModifierResponse::new();

    if success {
        let success_result = DefineModifierResponse_SuccessResult::new();
        define_modifier_response.set_success_result(success_result);
    } else {
        let error_result = DefineModifierResponse_ErrorResult::new();
        define_modifier_response.set_error_result(error_result);
    }

    let mut response = Response::new();
    response.set_define_modifier_response(define_modifier_response);

    response
}

pub fn make_define_binding_response(success: bool) -> Response {
    let mut define_binding_response = DefineBindingResponse::new();

    if success {
        let success_result = DefineBindingResponse_SuccessResult::new();
        define_binding_response.set_success_result(success_result);
    } else {
        let error_result = DefineBindingResponse_ErrorResult::new();
        define_binding_response.set_error_result(error_result);
    }

    let mut response = Response::new();
    response.set_define_binding_response(define_binding_response);

    response
}

pub fn make_start_listening_response(success: bool) -> Response {
    let mut start_listening_response = StartListeningResponse::new();

    if success {
        let success_result = StartListeningResponse_SuccessResult::new();
        start_listening_response.set_success_result(success_result);
    } else {
        let error_result = StartListeningResponse_ErrorResult::new();
        start_listening_response.set_error_result(error_result);
    }

    let mut response = Response::new();
    response.set_start_listening_response(start_listening_response);

    response
}

pub fn make_stop_listening_response(success: bool) -> Response {
    let mut stop_listening_response = StopListeningResponse::new();

    if success {
        let success_result = StopListeningResponse_SuccessResult::new();
        stop_listening_response.set_success_result(success_result);
    } else {
        let error_result = StopListeningResponse_ErrorResult::new();
        stop_listening_response.set_error_result(error_result);
    }

    let mut response = Response::new();
    response.set_stop_listening_response(stop_listening_response);

    response
}
