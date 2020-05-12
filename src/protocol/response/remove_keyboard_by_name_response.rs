use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{
    EventLoopHandle, NiaRemoveKeyboardByNameCommandResult,
};

use crate::error::NiaServerError;

use crate::protocol::NiaRemoveKeyboardByNameRequest;

#[derive(Debug, Clone)]
pub struct NiaRemoveKeyboardByNameResponse {
    command_result: NiaRemoveKeyboardByNameCommandResult,
}

impl NiaRemoveKeyboardByNameResponse {
    fn try_from(
        nia_remove_keyboard_by_name_request: NiaRemoveKeyboardByNameRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaRemoveKeyboardByNameResponse, NiaServerError> {
        let keyboard_name =
            nia_remove_keyboard_by_name_request.get_keyboard_name();

        let interpreter_command =
            NiaInterpreterCommand::make_remove_keyboard_by_name_command(
                keyboard_name,
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
            NiaInterpreterCommandResult::RemoveKeyboardByName(
                command_result,
            ) => NiaRemoveKeyboardByNameResponse { command_result },
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
        nia_remove_keyboard_by_name_request: NiaRemoveKeyboardByNameRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaRemoveKeyboardByNameResponse {
        let try_result = NiaRemoveKeyboardByNameResponse::try_from(
            nia_remove_keyboard_by_name_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaRemoveKeyboardByNameCommandResult::Failure(message);

                NiaRemoveKeyboardByNameResponse { command_result }
            }
        }
    }
}

impl From<NiaRemoveKeyboardByNameResponse>
    for nia_protocol_rust::RemoveKeyboardByNameResponse
{
    fn from(
        nia_remove_keyboard_by_name_response: NiaRemoveKeyboardByNameResponse,
    ) -> Self {
        let result = nia_remove_keyboard_by_name_response.command_result;

        let mut remove_keyboard_by_name_response =
            nia_protocol_rust::RemoveKeyboardByNameResponse::new();

        match result {
            NiaRemoveKeyboardByNameCommandResult::Success() => {
                let mut success_result =
                    nia_protocol_rust::RemoveKeyboardByNameResponse_SuccessResult::new();

                success_result.set_message(protobuf::Chars::from(
                    String::from("Success"),
                ));
                remove_keyboard_by_name_response
                    .set_success_result(success_result);
            }
            NiaRemoveKeyboardByNameCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::RemoveKeyboardByNameResponse_ErrorResult::new(
                    );

                error_result.set_message(protobuf::Chars::from(error_message));
                remove_keyboard_by_name_response.set_error_result(error_result);
            }
            NiaRemoveKeyboardByNameCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::RemoveKeyboardByNameResponse_FailureResult::new();

                failure_result
                    .set_message(protobuf::Chars::from(failure_message));
                remove_keyboard_by_name_response
                    .set_failure_result(failure_result);
            }
        }

        remove_keyboard_by_name_response
    }
}
