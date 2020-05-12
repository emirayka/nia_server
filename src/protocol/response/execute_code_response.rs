use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{EventLoopHandle, NiaExecuteCodeCommandResult};

use crate::error::NiaServerError;

use crate::protocol::NiaExecuteCodeRequest;

#[derive(Debug, Clone)]
pub struct NiaExecuteCodeResponse {
    command_result: NiaExecuteCodeCommandResult,
}

impl NiaExecuteCodeResponse {
    fn try_from(
        nia_execute_code_request: NiaExecuteCodeRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaExecuteCodeResponse, NiaServerError> {
        let code = nia_execute_code_request.get_code();

        let interpreter_command =
            NiaInterpreterCommand::make_execute_code_command(code);

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
            NiaInterpreterCommandResult::ExecuteCode(command_result) => {
                NiaExecuteCodeResponse { command_result }
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
        nia_execute_code_request: NiaExecuteCodeRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaExecuteCodeResponse {
        let try_result = NiaExecuteCodeResponse::try_from(
            nia_execute_code_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaExecuteCodeCommandResult::Failure(message);

                NiaExecuteCodeResponse { command_result }
            }
        }
    }
}

impl From<NiaExecuteCodeResponse> for nia_protocol_rust::ExecuteCodeResponse {
    fn from(nia_execute_code_response: NiaExecuteCodeResponse) -> Self {
        let result = nia_execute_code_response.command_result;

        let mut execute_code_response =
            nia_protocol_rust::ExecuteCodeResponse::new();

        match result {
            NiaExecuteCodeCommandResult::Success(string_result) => {
                let mut success_result =
                    nia_protocol_rust::ExecuteCodeResponse_SuccessResult::new();

                success_result
                    .set_message(protobuf::Chars::from(string_result));
                execute_code_response.set_success_result(success_result);
            }
            NiaExecuteCodeCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::ExecuteCodeResponse_ErrorResult::new();

                error_result.set_message(protobuf::Chars::from(error_message));
                execute_code_response.set_error_result(error_result);
            }
            NiaExecuteCodeCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::ExecuteCodeResponse_FailureResult::new();

                failure_result
                    .set_message(protobuf::Chars::from(failure_message));
                execute_code_response.set_failure_result(failure_result);
            }
        }

        execute_code_response
    }
}
