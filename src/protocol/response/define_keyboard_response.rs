use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{EventLoopHandle, NiaDefineKeyboardCommandResult};

use crate::error::NiaServerError;

use crate::protocol::NiaDefineKeyboardRequest;

#[derive(Debug, Clone)]
pub struct NiaDefineKeyboardResponse {
    command_result: NiaDefineKeyboardCommandResult,
}

impl NiaDefineKeyboardResponse {
    fn try_from(
        nia_define_keyboard_request: NiaDefineKeyboardRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaDefineKeyboardResponse, NiaServerError> {
        let (path, name) = nia_define_keyboard_request.get_path_and_name();

        let interpreter_command =
            NiaInterpreterCommand::make_define_keyboard_command(path, name);

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
            NiaInterpreterCommandResult::DefineKeyboard(command_result) => {
                NiaDefineKeyboardResponse { command_result }
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
        nia_define_keyboard_request: NiaDefineKeyboardRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaDefineKeyboardResponse {
        println!("{:?}", nia_define_keyboard_request);
        let try_result = NiaDefineKeyboardResponse::try_from(
            nia_define_keyboard_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());

                let command_result =
                    NiaDefineKeyboardCommandResult::Failure(message);

                NiaDefineKeyboardResponse { command_result }
            }
        }
    }
}

impl From<NiaDefineKeyboardResponse>
    for nia_protocol_rust::DefineKeyboardResponse
{
    fn from(nia_define_keyboard_response: NiaDefineKeyboardResponse) -> Self {
        let result = nia_define_keyboard_response.command_result;

        let mut define_keyboard_response =
            nia_protocol_rust::DefineKeyboardResponse::new();

        match result {
            NiaDefineKeyboardCommandResult::Success() => {
                let mut success_result =
                    nia_protocol_rust::DefineKeyboardResponse_SuccessResult::new();

                success_result.set_message(protobuf::Chars::from(
                    String::from("Success."),
                ));
                define_keyboard_response.set_success_result(success_result);
            }
            NiaDefineKeyboardCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::DefineKeyboardResponse_ErrorResult::new(
                    );

                error_result.set_message(protobuf::Chars::from(error_message));
                define_keyboard_response.set_error_result(error_result);
            }
            NiaDefineKeyboardCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::DefineKeyboardResponse_FailureResult::new();

                failure_result
                    .set_message(protobuf::Chars::from(failure_message));
                define_keyboard_response.set_failure_result(failure_result);
            }
        }

        define_keyboard_response
    }
}
