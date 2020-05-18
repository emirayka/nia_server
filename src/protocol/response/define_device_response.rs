use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{EventLoopHandle, NiaDefineDeviceCommandResult};

use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::{NiaDefineDeviceRequest, Serializable};
use crate::server::Server;
use nia_protocol_rust::DefineDeviceResponse;

#[derive(Debug, Clone)]
pub struct NiaDefineDeviceResponse {
    command_result: NiaDefineDeviceCommandResult,
}

impl NiaDefineDeviceResponse {
    fn try_from(
        server: &mut Server,
        nia_define_device_request: NiaDefineDeviceRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaDefineDeviceResponse, NiaServerError> {
        let device_id = nia_define_device_request.get_device_id();

        let device = match server.get_device_info_by_id(device_id) {
            Some(device) => device,
            None => {
                return NiaServerError::invalid_request(format!(
                    "There is no device with id: {}",
                    device_id
                ))
                .into()
            }
        };

        let device_name = device.get_device_name().to_string();
        let device_path = device.get_device_path().to_string();

        let interpreter_command =
            NiaInterpreterCommand::make_define_device_command(
                device_id,
                device_path,
                device_name,
            );

        event_loop_handle
            .send_command(interpreter_command)
            .map_err(|_| {
                NiaServerError::interpreter_execution(
                    "Error sending command to the interpreter.",
                )
            })?;

        let execution_result =
            event_loop_handle.receive_result().map_err(|_| {
                NiaServerError::interpreter_execution(
                    "Error reading command from the interpreter.",
                )
            })?;

        let response = match execution_result {
            NiaInterpreterCommandResult::DefineDevice(command_result) => {
                server.define_device_by_id(device_id);
                NiaDefineDeviceResponse { command_result }
            }
            _ => {
                return NiaServerError::interpreter_execution(
                    "Unexpected command result.",
                )
                .into();
            }
        };

        Ok(response)
    }

    pub fn from(
        server: &mut Server,
        nia_define_device_request: NiaDefineDeviceRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaDefineDeviceResponse {
        println!("{:?}", nia_define_device_request);
        let try_result = NiaDefineDeviceResponse::try_from(
            server,
            nia_define_device_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => {
                result
            },
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());

                let command_result =
                    NiaDefineDeviceCommandResult::Failure(message);

                NiaDefineDeviceResponse { command_result }
            }
        }
    }
}

impl
    Serializable<
        NiaDefineDeviceResponse,
        nia_protocol_rust::DefineDeviceResponse,
    > for NiaDefineDeviceResponse
{
    fn to_pb(&self) -> nia_protocol_rust::DefineDeviceResponse {
        let result = &self.command_result;

        let mut define_device_response =
            nia_protocol_rust::DefineDeviceResponse::new();

        match result {
            NiaDefineDeviceCommandResult::Success() => {
                let mut success_result =
                    nia_protocol_rust::DefineDeviceResponse_SuccessResult::new(
                    );

                success_result.set_message(protobuf::Chars::from(
                    String::from("Success."),
                ));
                define_device_response.set_success_result(success_result);
            }
            NiaDefineDeviceCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::DefineDeviceResponse_ErrorResult::new();

                error_result
                    .set_message(protobuf::Chars::from(error_message.clone()));
                define_device_response.set_error_result(error_result);
            }
            NiaDefineDeviceCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::DefineDeviceResponse_FailureResult::new(
                    );

                failure_result.set_message(protobuf::Chars::from(
                    failure_message.clone(),
                ));
                define_device_response.set_failure_result(failure_result);
            }
        }

        define_device_response
    }

    fn from_pb(
        object_pb: DefineDeviceResponse,
    ) -> NiaServerResult<NiaDefineDeviceResponse> {
        unreachable!()
    }
}
