use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{
    EventLoopHandle, NiaRemoveDeviceByNameCommandResult,
};

use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::{NiaRemoveDeviceByNameRequest, Serializable};
use crate::server::Server;
use nia_protocol_rust::RemoveDeviceByNameResponse;

#[derive(Debug, Clone)]
pub struct NiaRemoveDeviceByNameResponse {
    command_result: NiaRemoveDeviceByNameCommandResult,
}

impl NiaRemoveDeviceByNameResponse {
    fn try_from(
        server: &mut Server,
        nia_remove_keyboard_by_name_request: NiaRemoveDeviceByNameRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaRemoveDeviceByNameResponse, NiaServerError> {
        let device_name = nia_remove_keyboard_by_name_request.get_device_name();

        let interpreter_command =
            NiaInterpreterCommand::make_remove_device_by_name_command(
                device_name.clone(),
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
            NiaInterpreterCommandResult::RemoveDeviceByName(command_result) => {
                (*server).undefine_device_by_name(&device_name);
                NiaRemoveDeviceByNameResponse { command_result }
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
        nia_remove_keyboard_by_name_request: NiaRemoveDeviceByNameRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaRemoveDeviceByNameResponse {
        let try_result = NiaRemoveDeviceByNameResponse::try_from(
            server,
            nia_remove_keyboard_by_name_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaRemoveDeviceByNameCommandResult::Failure(message);

                NiaRemoveDeviceByNameResponse { command_result }
            }
        }
    }
}

impl
    Serializable<
        NiaRemoveDeviceByNameResponse,
        nia_protocol_rust::RemoveDeviceByNameResponse,
    > for NiaRemoveDeviceByNameResponse
{
    fn to_pb(&self) -> RemoveDeviceByNameResponse {
        let result = &self.command_result;

        let mut remove_keyboard_by_name_response =
            nia_protocol_rust::RemoveDeviceByNameResponse::new();

        match result {
            NiaRemoveDeviceByNameCommandResult::Success() => {
                let mut success_result =
                    nia_protocol_rust::RemoveDeviceByNameResponse_SuccessResult::new();

                success_result.set_message(protobuf::Chars::from(
                    String::from("Success"),
                ));
                remove_keyboard_by_name_response
                    .set_success_result(success_result);
            }
            NiaRemoveDeviceByNameCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::RemoveDeviceByNameResponse_ErrorResult::new(
                    );

                error_result
                    .set_message(protobuf::Chars::from(error_message.clone()));
                remove_keyboard_by_name_response.set_error_result(error_result);
            }
            NiaRemoveDeviceByNameCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::RemoveDeviceByNameResponse_FailureResult::new();

                failure_result.set_message(protobuf::Chars::from(
                    failure_message.clone(),
                ));
                remove_keyboard_by_name_response
                    .set_failure_result(failure_result);
            }
        }

        remove_keyboard_by_name_response
    }

    fn from_pb(
        object_pb: RemoveDeviceByNameResponse,
    ) -> NiaServerResult<NiaRemoveDeviceByNameResponse> {
        unreachable!()
    }
}
