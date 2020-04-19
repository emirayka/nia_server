use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

use ws::listen;
use protobuf::Message;

use nia_protocol_rust::*;
use nia_interpreter_core::Interpreter;

const VERSION_MESSAGE: &'static str = "nia-server version '0.0.1'";
const INFO_MESSAGE: &'static str = "Some not yet useful info";

pub struct Server {
}

impl Server {
    pub fn new() -> Server {
        Server {
        }
    }
}

impl Server {
    fn make_handshake_response(&self) -> Response {
        let mut handshake_response = HandshakeResponse::new();

        handshake_response.set_version(protobuf::Chars::from(String::from(VERSION_MESSAGE)));
        handshake_response.set_info(protobuf::Chars::from(String::from(INFO_MESSAGE)));

        let mut response = Response::new();

        response.set_handshakeResponse(handshake_response);

        response
    }

    fn make_get_devices_response(&self) -> Response {
        let mut get_devices_response = GetDevicesResponse::new();
        let devices = crate::commands::get_devices()
            .expect("Failure getting device list.");

        let devices = devices.into_iter()
            .map(|s: String| {
                protobuf::Chars::from(s)
            }).collect();

        get_devices_response.set_devices(devices);

        let mut response = Response::new();

        response.set_getDevicesResponse(get_devices_response);

        response
    }

    fn make_get_device_info_response(&self, request: GetDeviceInfoRequest) -> Response {
        let device_path = request.get_device();
        let device_info = crate::commands::get_device_info(device_path)
            .expect("Failure getting device info");

        let mut get_device_info_response = GetDeviceInfoResponse::new();

        get_device_info_response.set_device(protobuf::Chars::from(String::from(device_path)));
        get_device_info_response.set_name(protobuf::Chars::from(String::from(device_info.get_name())));
        get_device_info_response.set_model(protobuf::Chars::from(String::from(device_info.get_model())));

        let mut response = Response::new();

        response.set_getDeviceInfoResponse(get_device_info_response);

        response
    }

    fn make_execute_code_response(&self, request: ExecuteCodeRequest, interpreter: &mut Interpreter) -> Response {
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
                        execute_code_response.set_message(protobuf::Chars::from(s));
                        execute_code_response.set_result(ExecuteCodeResponse_ExecutionResult::SUCCESS)
                    }
                    Err(error) => {
                        if error.is_failure() {
                            execute_code_response.set_result(ExecuteCodeResponse_ExecutionResult::FAILURE)
                        } else {
                            execute_code_response.set_result(ExecuteCodeResponse_ExecutionResult::ERROR)
                        }

                        execute_code_response.set_message(
                            protobuf::Chars::from(
                                format!("{}", error)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                if error.is_failure() {
                    execute_code_response.set_result(ExecuteCodeResponse_ExecutionResult::FAILURE)
                } else {
                    execute_code_response.set_result(ExecuteCodeResponse_ExecutionResult::ERROR)
                }

                execute_code_response.set_message(
                    protobuf::Chars::from(
                        format!("{}", error)
                    )
                );
            }
        }

        let mut response = Response::new();

        response.set_executeCodeResponse(execute_code_response);

        response
    }
}

impl Server {
    fn send_response(&self, sender: &ws::Sender, response: Response) {
        let bytes = response.write_to_bytes().expect("Failure converting response to bytes");

        sender.send(ws::Message::Binary(bytes)).expect("Failure sending response");
    }

    fn on_handshake_request(&self, sender: &ws::Sender, message: HandshakeRequest) {
        println!("Handshake request arrived: {:?}", message);

        let handshake_response = self.make_handshake_response();

        self.send_response(sender, handshake_response);
    }

    fn on_get_devices_request(&self, sender: &ws::Sender, message: GetDevicesRequest) {
        println!("Get devices request arrived: {:?}", message);

        let get_devices_response = self.make_get_devices_response();

        self.send_response(sender, get_devices_response);
    }

    fn on_get_device_info_request(&self, sender: &ws::Sender, message: GetDeviceInfoRequest) {
        println!("Get device info request arrived: {:?}", message);

        let get_device_info_response = self.make_get_device_info_response(message);

        self.send_response(sender, get_device_info_response);
    }

    fn on_execute_code_request(&self, sender: &ws::Sender, message: ExecuteCodeRequest, interpreter: &mut Interpreter) {
        println!("Execute code request arrived: {:?}", message);

        let execute_code_response = self.make_execute_code_response(message, interpreter);

        self.send_response(sender, execute_code_response);
    }

    pub fn start_listening(&self) {
        let interpreter = Arc::new(Mutex::new(Interpreter::new()));

        listen("127.0.0.1:12112", |out| {
            let interpreter = Arc::clone(&interpreter);

            move |msg| {
                let mut interpreter = interpreter.lock().unwrap();

                match msg {
                    ws::Message::Binary(bytes) => {
                        println!("Binary message: {:?}", bytes);

                        let mut request_message = nia_protocol_rust::Request::new();

                        request_message.merge_from_bytes(&bytes);

                        if request_message.has_handshakeRequest() {
                            let req = request_message.take_handshakeRequest();
                            self.on_handshake_request(&out, req.clone());
                            self.on_handshake_request(&out, req);
                        } else if request_message.has_getDevicesRequest() {
                            self.on_get_devices_request(&out, request_message.take_getDevicesRequest())
                        } else if request_message.has_getDeviceInfoRequest() {
                            self.on_get_device_info_request(&out, request_message.take_getDeviceInfoRequest())
                        } else if request_message.has_executeCodeRequest() {
                            self.on_execute_code_request(&out, request_message.take_executeCodeRequest(), &mut interpreter)
                        } else {
                            panic!("Unexpected message: {:?}", bytes);
                        }
                    }
                    ws::Message::Text(text) => {
                        println!("Text message: {:?}", text);
                    }
                }

                Ok(())
            }
        }).expect("Server failure: ws.");
    }
}

