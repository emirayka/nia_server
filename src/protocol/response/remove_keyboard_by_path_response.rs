use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{
    EventLoopHandle, NiaRemoveKeyboardByPathCommandResult,
};

use crate::error::NiaServerError;

use crate::protocol::NiaRemoveKeyboardByPathRequest;

#[derive(Debug, Clone)]
pub struct NiaRemoveKeyboardByPathResponse {
    command_result: NiaRemoveKeyboardByPathCommandResult,
}

impl NiaRemoveKeyboardByPathResponse {
    fn try_from(
        nia_remove_keyboard_by_path_request: NiaRemoveKeyboardByPathRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaRemoveKeyboardByPathResponse, NiaServerError> {
        let keyboard_path =
            nia_remove_keyboard_by_path_request.get_keyboard_path();

        let interpreter_command =
            NiaInterpreterCommand::make_remove_keyboard_by_path_command(
                keyboard_path,
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
            NiaInterpreterCommandResult::RemoveKeyboardByPath(
                command_result,
            ) => NiaRemoveKeyboardByPathResponse { command_result },
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
        nia_remove_keyboard_by_path_request: NiaRemoveKeyboardByPathRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaRemoveKeyboardByPathResponse {
        println!("{:?}", nia_remove_keyboard_by_path_request);
        let try_result = NiaRemoveKeyboardByPathResponse::try_from(
            nia_remove_keyboard_by_path_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaRemoveKeyboardByPathCommandResult::Failure(message);

                NiaRemoveKeyboardByPathResponse { command_result }
            }
        }
    }
}

impl From<NiaRemoveKeyboardByPathResponse>
    for nia_protocol_rust::RemoveKeyboardByPathResponse
{
    fn from(
        nia_remove_keyboard_by_path_response: NiaRemoveKeyboardByPathResponse,
    ) -> Self {
        let result = nia_remove_keyboard_by_path_response.command_result;

        let mut remove_keyboard_by_path_response =
            nia_protocol_rust::RemoveKeyboardByPathResponse::new();

        match result {
            NiaRemoveKeyboardByPathCommandResult::Success() => {
                let mut success_result =
                    nia_protocol_rust::RemoveKeyboardByPathResponse_SuccessResult::new();

                success_result.set_message(protobuf::Chars::from(
                    String::from("Success."),
                ));
                remove_keyboard_by_path_response
                    .set_success_result(success_result);
            }
            NiaRemoveKeyboardByPathCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::RemoveKeyboardByPathResponse_ErrorResult::new(
                    );

                error_result.set_message(protobuf::Chars::from(error_message));
                remove_keyboard_by_path_response.set_error_result(error_result);
            }
            NiaRemoveKeyboardByPathCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::RemoveKeyboardByPathResponse_FailureResult::new();

                failure_result
                    .set_message(protobuf::Chars::from(failure_message));
                remove_keyboard_by_path_response
                    .set_failure_result(failure_result);
            }
        }

        remove_keyboard_by_path_response
    }
}
