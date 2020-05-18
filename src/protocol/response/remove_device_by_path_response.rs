use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{
    EventLoopHandle, NiaRemoveDeviceByPathCommandResult,
};

use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::{NiaRemoveDeviceByPathRequest, Serializable};
use nia_protocol_rust::RemoveDeviceByPathResponse;
use crate::server::Server;

#[derive(Debug, Clone)]
pub struct NiaRemoveDeviceByPathResponse {
    command_result: NiaRemoveDeviceByPathCommandResult,
}

impl NiaRemoveDeviceByPathResponse {
    fn try_from(
        server: &mut Server,
        nia_remove_keyboard_by_path_request: NiaRemoveDeviceByPathRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaRemoveDeviceByPathResponse, NiaServerError> {
        let device_path =
            nia_remove_keyboard_by_path_request.get_device_path();

        let interpreter_command =
            NiaInterpreterCommand::make_remove_device_by_path_command(
                device_path.clone(),
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
            NiaInterpreterCommandResult::RemoveDeviceByPath(command_result) => {
                (*server).undefine_device_by_path(&device_path);
                NiaRemoveDeviceByPathResponse { command_result }
            }
            _ => {
                return NiaServerError::interpreter_execution(
                    "Unexpected command result.",
                )
                .into()
            }
        };

        Ok(response)
    }

    pub fn from(
        server: &mut Server,
        nia_remove_keyboard_by_path_request: NiaRemoveDeviceByPathRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaRemoveDeviceByPathResponse {
        println!("{:?}", nia_remove_keyboard_by_path_request);
        let try_result = NiaRemoveDeviceByPathResponse::try_from(
            server,
            nia_remove_keyboard_by_path_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaRemoveDeviceByPathCommandResult::Failure(message);

                NiaRemoveDeviceByPathResponse { command_result }
            }
        }
    }
}

impl
    Serializable<
        NiaRemoveDeviceByPathResponse,
        nia_protocol_rust::RemoveDeviceByPathResponse,
    > for NiaRemoveDeviceByPathResponse
{
    fn to_pb(&self) -> RemoveDeviceByPathResponse {
        let result = &self.command_result;

        let mut remove_keyboard_by_path_response =
            nia_protocol_rust::RemoveDeviceByPathResponse::new();

        match result {
            NiaRemoveDeviceByPathCommandResult::Success() => {
                let mut success_result =
                    nia_protocol_rust::RemoveDeviceByPathResponse_SuccessResult::new();

                success_result.set_message(protobuf::Chars::from(
                    String::from("Success."),
                ));
                remove_keyboard_by_path_response
                    .set_success_result(success_result);
            }
            NiaRemoveDeviceByPathCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::RemoveDeviceByPathResponse_ErrorResult::new(
                    );

                error_result
                    .set_message(protobuf::Chars::from(error_message.clone()));
                remove_keyboard_by_path_response.set_error_result(error_result);
            }
            NiaRemoveDeviceByPathCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::RemoveDeviceByPathResponse_FailureResult::new();

                failure_result.set_message(protobuf::Chars::from(
                    failure_message.clone(),
                ));
                remove_keyboard_by_path_response
                    .set_failure_result(failure_result);
            }
        }

        remove_keyboard_by_path_response
    }

    fn from_pb(
        object_pb: RemoveDeviceByPathResponse,
    ) -> NiaServerResult<NiaRemoveDeviceByPathResponse> {
        unreachable!()
    }
}
