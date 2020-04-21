mod utils;

use std::thread;
use std::sync::{Arc, mpsc};
use std::sync::Mutex;

use ws::listen;
use protobuf::Message;

use nia_protocol_rust::*;
use nia_interpreter_core::Interpreter;
use nia_events::KeyId;
use nia_events::KeyboardId;

use crate::error::Error;
use crate::event_handler::NiaEventHandler;

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

    fn on_execute_code_request(&self,
                               sender: &ws::Sender,
                               message: ExecuteCodeRequest,
                               interpreter: &mut Interpreter,
    ) -> Result<(), Error> {
        println!("Execute code request arrived: {:?}", message);

        let execute_code_response = make_execute_code_response(message, interpreter);

        self.send_response(sender, execute_code_response)
    }

    fn on_register_keyboard_request(&self,
                                    sender: &ws::Sender,
                                    message: RegisterKeyboardRequest,
    ) -> Result<(), Error> {
        println!("Register keyboard request arrived: {:?}", message);

        let register_keyboard_response = make_register_keyboard_response();

        self.send_response(sender, register_keyboard_response)
    }

    fn on_define_modifier_request(&self,
                                  sender: &ws::Sender,
                                  message: DefineModifierRequest,
    ) {
        println!("Define modifier request arrived: {:?}", message);

        let define_modifier_response = make_define_modifier_response(true);

        self.send_response(sender, define_modifier_response);
    }

    fn on_define_binding_request(&self,
                                 sender: &ws::Sender,
                                 message: DefineBindingRequest,
    ) {
        println!("Define binding request arrived: {:?}", message);

        let define_binding_response = make_define_binding_response(true);

        self.send_response(sender, define_binding_response);
    }

    fn on_start_listening_request(
        &self,
        sender: &ws::Sender,
        message: StartListeningRequest,
        success: bool,
    ) -> Result<(), Error> {
        println!("Start listening request arrived: {:?}", message);

        let start_listening_response = make_start_listening_response(success);

        self.send_response(sender, start_listening_response)
    }

    fn on_stop_listening_request(
        &self,
        sender: &ws::Sender,
        message: StopListeningRequest,
        success: bool,
    ) -> Result<(), Error> {
        println!("Start listening request arrived: {:?}", message);

        let stop_listening_response = make_stop_listening_response(success);

        self.send_response(sender, stop_listening_response)
    }

    fn on_request(&self,
                  sender: &ws::Sender,
                  request: Request,
                  interpreter: &mut Interpreter,
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
        } else if request.has_execute_code_request() {
            self.on_execute_code_request(
                sender,
                request.take_execute_code_request(),
                interpreter,
            )
        } else {
            println!("Unknown request; {:?}, ignoring...", request);

            Ok(())
        }
    }

    pub fn start(&self) {
        let mut event_handler = Arc::new(Mutex::new(NiaEventHandler::new()));
        let tuple = Arc::new(Mutex::new(None));

        let mut interpreter = Arc::new(Mutex::new(Interpreter::new()));

        listen("127.0.0.1:12112", |out| {
            let interpreter = Arc::clone(&interpreter);
            let tuple = Arc::clone(&tuple);
            let event_handler = Arc::clone(&event_handler);

            move |msg| {
                match msg {
                    ws::Message::Binary(bytes) => {
                        println!("Binary message: {:?}", bytes);

                        let mut request = nia_protocol_rust::Request::new();
                        request.merge_from_bytes(&bytes);
                        if request.has_register_keyboard_request() {
                            let mut event_handler = event_handler.lock().unwrap();

                            let register_keyboard_request = request.take_register_keyboard_request();

                            let device_path = register_keyboard_request.get_device_path();
                            let device_name = register_keyboard_request.get_device_name();

                            event_handler.add_keyboard(device_path, device_name);

                            let register_keyboard_request = request.take_register_keyboard_request();
                            self.on_register_keyboard_request(
                                &out,
                                register_keyboard_request,
                            );
                        } else if request.has_define_modifier_request() {
                            let mut event_handler = event_handler.lock().unwrap();
                            let define_modifier_request = request.take_define_modifier_request();

                            let modifier = define_modifier_request.get_key_chord_part();
                            let key_chord_part = parse_request_key_chord_part(
                                modifier
                            );
                            event_handler.add_modifier(key_chord_part);

                            let define_modifier_request = request.take_define_modifier_request();
                            self.on_define_modifier_request(
                                &out,
                                define_modifier_request,
                            );
                        } else if request.has_define_binding_request() {
                            let mut event_handler = event_handler.lock().unwrap();
                            let define_binding_request = request.get_define_binding_request();

                            let request_key_chords = define_binding_request.get_key_chords();
                            let key_chords = parse_request_key_chords(
                                request_key_chords
                            );

                            let action = crate::event_handler::Action::Empty;

                            event_handler.add_mapping(key_chords, action);

                            let define_binding_request = request.take_define_binding_request();
                            self.on_define_binding_request(
                                &out,
                                define_binding_request,
                            );
                        } else if request.has_start_listening_request() {
                            let event_handler = event_handler.lock().unwrap();
                            let mut tuple = tuple.lock().unwrap();

                            let success = if let Some(_) = *tuple {
                                false
                            } else {
                                match event_handler.start_listening() {
                                    Ok(t) => {
                                        *tuple = Some(t);
                                        true
                                    }
                                    Err(err) => {
                                        false
                                    }
                                }
                            };

                            let start_listening_request = request.take_start_listening_request();
                            self.on_start_listening_request(&out, start_listening_request, success);
                        } else if request.has_stop_listening_request() {
                            let event_handler = event_handler.lock().unwrap();
                            let mut tuple = tuple.lock().unwrap();

                            let success = if let Some((_, _, stopper)) = tuple.as_ref() {
                                stopper.send(());
                                true
                            } else {
                                true
                            };

                            let stop_listening_request = request.take_stop_listening_request();
                            self.on_stop_listening_request(
                                &out,
                                stop_listening_request,
                                success,
                            );
                        } else {
                            let mut interpreter = interpreter.lock().unwrap();
                            self.on_request(&out, request, &mut interpreter);
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

