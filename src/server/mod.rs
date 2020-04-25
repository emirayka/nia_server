mod utils;

use std::thread;
use std::sync::{Arc, mpsc};
use std::sync::Mutex;

use ws::listen;
use protobuf::Message;

use nia_protocol_rust::*;
use nia_interpreter_core::{Interpreter, EventLoop, InterpreterCommand, CommandResult};
use nia_events::KeyId;
use nia_events::KeyboardId;

use crate::error::Error;

pub use utils::*;


pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }
}

impl Server {
    fn send_response(&self,
                     sender: &ws::Sender,
                     response: Response,
    ) -> Result<(), Error> {
        let bytes = response.write_to_bytes()
            .map_err(|_| Error::unknown())?;

        sender.send(ws::Message::Binary(bytes))
            .map_err(|_| Error::unknown())?;

        Ok(())
    }

    fn on_handshake_request(&self,
                            sender: &ws::Sender,
                            message: HandshakeRequest,
    ) -> Result<(), Error> {
        println!("Handshake request arrived: {:?}", message);

        let handshake_response = make_handshake_response();

        self.send_response(sender, handshake_response)
    }

    fn on_get_devices_request(&self,
                              sender: &ws::Sender,
                              message: GetDevicesRequest,
    ) -> Result<(), Error> {
        println!("Get devices request arrived: {:?}", message);

        let get_devices_response = make_get_devices_response();

        self.send_response(sender, get_devices_response)
    }

    fn on_get_device_info_request(&self,
                                  sender: &ws::Sender,
                                  message: GetDeviceInfoRequest,
    ) -> Result<(), Error> {
        println!("Get device info request arrived: {:?}", message);

        let get_device_info_response = make_get_device_info_response(message);

        self.send_response(sender, get_device_info_response)
    }

    fn on_request(&self,
                  sender: &ws::Sender,
                  request: Request,
    ) -> Result<(), Error> {
        let mut request = request;

        if request.has_handshake_request() {
            self.on_handshake_request(
                sender,
                request.take_handshake_request(),
            )
        } else if request.has_get_devices_request() {
            self.on_get_devices_request(
                sender,
                request.take_get_devices_request(),
            )
        } else if request.has_get_device_info_request() {
            self.on_get_device_info_request(
                sender,
                request.take_get_device_info_request(),
            )
        } else {
            println!("Unknown request; {:?}, ignoring...", request);

            Ok(())
        }
    }

    pub fn start(&self) {
        let mut interpreter = Interpreter::new();
        let (
            interpreter_command_sender,
            interpreter_command_result_receiver
        ) = EventLoop::run_event_loop(interpreter);

        let interpreter_command_sender = Arc::new(Mutex::new(
            interpreter_command_sender
        ));
        let interpreter_command_result_receiver = Arc::new(Mutex::new(
            interpreter_command_result_receiver
        ));

        listen("127.0.0.1:12112", |out| {
            let interpreter_command_sender = Arc::clone(
                &interpreter_command_sender
            );
            let interpreter_command_result_receiver = Arc::clone(
                &interpreter_command_result_receiver
            );

            move |msg| {
                match msg {
                    ws::Message::Binary(bytes) => {
                        println!("Binary message: {:?}", bytes);

                        let mut request = nia_protocol_rust::Request::new();
                        request.merge_from_bytes(&bytes);

                        if request.has_execute_code_request() {
                            let execute_code_request = request.take_execute_code_request();

                            let interpreter_command_sender = interpreter_command_sender
                                .lock().unwrap();

                            let interpreter_command_result_receiver = interpreter_command_result_receiver
                                .lock().unwrap();

                            let code = String::from(execute_code_request.get_code());
                            println!("Execution code: {}", code);

                            match interpreter_command_sender.send(InterpreterCommand::Execution(code)) {
                                Ok(_) => {},
                                Err(_) => {
                                    // interpreter is dead now
                                }
                            }

                            match interpreter_command_result_receiver.recv() {
                                Ok(CommandResult::ExecutionResult(result)) => {
                                    let response = make_execute_code_response(result);
                                    self.send_response(&out, response);
                                },
                                Err(_) => {
                                    // interpreter is dead now
                                }
                            }
                        } else {
                            self.on_request(&out, request);
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

