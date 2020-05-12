mod utils;
use std::convert::TryFrom;

use std::sync::Mutex;
use std::sync::{mpsc, Arc};
use std::thread;

use protobuf::Message;
use ws::listen;

use nia_protocol_rust::*;

use nia_interpreter_core::EventLoop;
use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;

use crate::error::NiaServerError;

use crate::protocol::{NiaRequest, NiaResponse};
pub use utils::*;

pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }
}

impl Server {
    fn send_response(
        &self,
        sender: &ws::Sender,
        response: Response,
    ) -> Result<(), NiaServerError> {
        let bytes = response
            .write_to_bytes()
            .map_err(|_| NiaServerError::unknown(""))?;

        sender
            .send(ws::Message::Binary(bytes))
            .map_err(|_| NiaServerError::unknown(""))?;

        Ok(())
    }

    pub fn start(&self) {
        let mut interpreter = Interpreter::new();

        let event_loop_handle = EventLoop::run_event_loop(interpreter);
        let event_loop_handle = Arc::new(Mutex::new(event_loop_handle));

        listen("127.0.0.1:12112", |out| {
            let event_loop_handle = Arc::clone(&event_loop_handle);

            move |msg| {
                match msg {
                    ws::Message::Binary(bytes) => {
                        let mut request = nia_protocol_rust::Request::new();
                        request.merge_from_bytes(&bytes);

                        println!("Got request: {:?}", request);

                        let nia_request = match NiaRequest::try_from(request) {
                            Ok(nia_request) => nia_request,
                            Err(error) => {
                                println!("Error occured: {:?}", error);
                                println!("Ignoring request");
                                return Ok(());
                            }
                        };

                        let event_loop_handle =
                            event_loop_handle.lock().unwrap();

                        let nia_response =
                            NiaResponse::from(nia_request, event_loop_handle);
                        let response = nia_response.into();

                        println!("Got response: {:?}", response);

                        self.send_response(&out, response);
                    }
                    ws::Message::Text(text) => {
                        println!("Text message arrived: {:?}", text);
                    }
                }

                Ok(())
            }
        })
        .expect("Server failure: ws.");
    }
}
