use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{EventLoopHandle, NiaRemoveDeviceByIdCommandResult};

use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::{NiaRemoveDeviceByIdRequest, Serializable};
use crate::server::Server;
use nia_protocol_rust::RemoveDeviceByIdResponse;

#[derive(Debug, Clone)]
pub struct NiaRemoveDeviceByIdResponse {
    command_result: NiaRemoveDeviceByIdCommandResult,
}

impl NiaRemoveDeviceByIdResponse {
    fn try_from(
        server: &mut Server,
        nia_remove_keyboard_by_id_request: NiaRemoveDeviceByIdRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaRemoveDeviceByIdResponse, NiaServerError> {
        let device_id = nia_remove_keyboard_by_id_request.get_device_id();

        let interpreter_command =
            NiaInterpreterCommand::make_remove_device_by_id_command(
                device_id.clone(),
            );

        event_loop_handle
            .send_command(interpreter_command)
            .map_err(|_| {
                NiaServerError::interpreter_error(
                    "Error sending command to the interpreter.",
                )
            })?;

        let execution_result =
            event_loop_handle.receive_result().map_err(|_| {
                NiaServerError::interpreter_error(
                    "Error reading command from the interpreter.",
                )
            })?;

        let response = match execution_result {
            NiaInterpreterCommandResult::RemoveDeviceById(command_result) => {
                (*server).undefine_device_by_id(device_id);
                NiaRemoveDeviceByIdResponse { command_result }
            }
            _ => {
                return NiaServerError::interpreter_error(
                    "Unexpected command result.",
                )
                .into()
            }
        };

        Ok(response)
    }

    pub fn from(
        server: &mut Server,
        nia_remove_keyboard_by_id_request: NiaRemoveDeviceByIdRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaRemoveDeviceByIdResponse {
        let try_result = NiaRemoveDeviceByIdResponse::try_from(
            server,
            nia_remove_keyboard_by_id_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaRemoveDeviceByIdCommandResult::Failure(message);

                NiaRemoveDeviceByIdResponse { command_result }
            }
        }
    }
}

impl
    Serializable<
        NiaRemoveDeviceByIdResponse,
        nia_protocol_rust::RemoveDeviceByIdResponse,
    > for NiaRemoveDeviceByIdResponse
{
    fn to_pb(&self) -> RemoveDeviceByIdResponse {
        let result = &self.command_result;

        let mut remove_keyboard_by_id_response =
            nia_protocol_rust::RemoveDeviceByIdResponse::new();

        match result {
            NiaRemoveDeviceByIdCommandResult::Success() => {
                let mut success_result =
                    nia_protocol_rust::RemoveDeviceByIdResponse_SuccessResult::new();

                success_result.set_message(protobuf::Chars::from(
                    String::from("Success"),
                ));
                remove_keyboard_by_id_response
                    .set_success_result(success_result);
            }
            NiaRemoveDeviceByIdCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::RemoveDeviceByIdResponse_ErrorResult::new(
                    );

                error_result
                    .set_message(protobuf::Chars::from(error_message.clone()));
                remove_keyboard_by_id_response.set_error_result(error_result);
            }
            NiaRemoveDeviceByIdCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::RemoveDeviceByIdResponse_FailureResult::new();

                failure_result.set_message(protobuf::Chars::from(
                    failure_message.clone(),
                ));
                remove_keyboard_by_id_response
                    .set_failure_result(failure_result);
            }
        }

        remove_keyboard_by_id_response
    }

    fn from_pb(
        object_pb: RemoveDeviceByIdResponse,
    ) -> NiaServerResult<NiaRemoveDeviceByIdResponse> {
        unreachable!()
    }
}
