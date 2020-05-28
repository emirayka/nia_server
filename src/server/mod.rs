use std::convert::TryFrom;

use std::sync::Mutex;
use std::sync::{mpsc, Arc};
use std::thread;

use protobuf::Message;
use ws::listen;

use nia_interpreter_core::EventLoop;
use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;

use crate::error::{from_interpreter_error, NiaServerError, NiaServerResult};

use crate::protocol::{DeviceInfo, NiaRequest, NiaResponse, Serializable};

pub struct Server {
    devices_info: Vec<DeviceInfo>,
}

impl Server {
    pub fn new() -> Server {
        let available_device_paths = crate::utils::get_available_devices()
            .expect("Cannot get device list.");

        let devices_info =
            crate::utils::get_devices_info(&available_device_paths)
                .expect("Cannot parse list.");

        Server { devices_info }
    }

    pub fn get_devices(&self) -> &Vec<DeviceInfo> {
        &self.devices_info
    }

    pub fn get_device_info_by_id(&self, id: i32) -> Option<&DeviceInfo> {
        for device_info in &self.devices_info {
            if device_info.get_device_id() == id {
                return Some(device_info);
            }
        }

        None
    }

    pub fn define_device_by_id(&mut self, device_id: i32) {
        for device in &mut self.devices_info {
            if device.get_device_id() == device_id {
                device.set_defined(true)
            }
        }
    }

    pub fn undefine_device_by_id(&mut self, device_id: i32) {
        for device in &mut self.devices_info {
            if device.get_device_id() == device_id {
                device.set_defined(false)
            }
        }
    }

    pub fn undefine_device_by_path(&mut self, path: &str) {
        for device in &mut self.devices_info {
            if device.get_device_path() == path {
                device.set_defined(false)
            }
        }
    }

    pub fn undefine_device_by_name(&mut self, name: &str) {
        for device in &mut self.devices_info {
            if device.get_device_name() == name {
                device.set_defined(false)
            }
        }
    }
}

impl Server {
    fn send_response(
        &self,
        sender: &ws::Sender,
        response: nia_protocol_rust::Response,
    ) -> Result<(), NiaServerError> {
        let bytes = response
            .write_to_bytes()
            .map_err(|_| NiaServerError::unknown(""))?;

        sender
            .send(ws::Message::Binary(bytes))
            .map_err(|_| NiaServerError::unknown(""))?;

        Ok(())
    }

    pub fn start(self) {
        let mut server = self;
        let mut interpreter = Interpreter::new();

        nia_interpreter_core::library::define_global_mapping(
            &mut interpreter,
            &nia_interpreter_core::Mapping::new(
                vec![nia_interpreter_core::KeyChord::new(
                    vec![
                        nia_interpreter_core::Key::new_device_key(1, 16),
                        nia_interpreter_core::Key::new_device_key(1, 17),
                    ],
                    nia_interpreter_core::Key::new_device_key(1, 18),
                )],
                nia_interpreter_core::Action::ExecuteOSCommand(String::from(
                    "kek",
                )),
            ),
        )
        .unwrap();

        let event_loop_handle = EventLoop::run_event_loop(interpreter);
        let event_loop_handle = Arc::new(Mutex::new(event_loop_handle));
        let server_handle = Arc::new(Mutex::new(server));

        listen("127.0.0.1:12112", |out| {
            let event_loop_handle = event_loop_handle.clone();
            let server_handle = server_handle.clone();

            move |msg| {
                match msg {
                    ws::Message::Binary(bytes) => {
                        let mut request = nia_protocol_rust::Request::new();
                        request.merge_from_bytes(&bytes);

                        println!("Got request: {:?}", request);

                        let nia_request = match NiaRequest::from_pb(request) {
                            Ok(nia_request) => nia_request,
                            Err(error) => {
                                println!("Error occurred: {:?}", error);
                                println!("Ignoring request");
                                return Ok(());
                            }
                        };

                        let event_loop_handle =
                            event_loop_handle.lock().unwrap();
                        let mut server_handle = server_handle.lock().unwrap();

                        let nia_response = NiaResponse::from(
                            &mut server_handle,
                            nia_request,
                            event_loop_handle,
                        );
                        let response = nia_response.to_pb();

                        println!("Sent response: {:?}", response);

                        server_handle.send_response(&out, response);
                    }
                    ws::Message::Text(text) => {
                        println!("Text message arrived: {:?}", text);
                        println!("Ignoring...");
                    }
                }

                Ok(())
            }
        })
        .expect("Server failure: ws.");
    }
}
