use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{EventLoopHandle, NiaIsListeningCommandResult};

use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::{NiaConvertable, NiaIsListeningRequest, Serializable};
use nia_protocol_rust::IsListeningResponse;

#[derive(Debug, Clone)]
pub struct NiaIsListeningResponse {
    command_result: NiaIsListeningCommandResult,
}

impl NiaIsListeningResponse {
    fn try_from(
        nia_is_listening_request: NiaIsListeningRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaIsListeningResponse, NiaServerError> {
        let interpreter_command =
            NiaInterpreterCommand::make_is_listening_command();

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
            NiaInterpreterCommandResult::IsListening(command_result) => {
                NiaIsListeningResponse { command_result }
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
        nia_is_listening_request: NiaIsListeningRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaIsListeningResponse {
        let try_result = NiaIsListeningResponse::try_from(
            nia_is_listening_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaIsListeningCommandResult::Failure(message);

                NiaIsListeningResponse { command_result }
            }
        }
    }
}

impl
    Serializable<NiaIsListeningResponse, nia_protocol_rust::IsListeningResponse>
    for NiaIsListeningResponse
{
    fn to_pb(&self) -> IsListeningResponse {
        let result = &self.command_result;

        let mut is_listening_response =
            nia_protocol_rust::IsListeningResponse::new();

        match result {
            NiaIsListeningCommandResult::Success(result) => {
                let mut success_result =
                    nia_protocol_rust::IsListeningResponse_SuccessResult::new();

                success_result.set_is_listening(*result);
                is_listening_response.set_success_result(success_result);
            }
            NiaIsListeningCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::IsListeningResponse_ErrorResult::new();

                error_result
                    .set_message(protobuf::Chars::from(error_message.clone()));
                is_listening_response.set_error_result(error_result);
            }
            NiaIsListeningCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::IsListeningResponse_FailureResult::new();

                failure_result.set_message(protobuf::Chars::from(
                    failure_message.clone(),
                ));
                is_listening_response.set_failure_result(failure_result);
            }
        }

        is_listening_response
    }

    fn from_pb(
        object_pb: IsListeningResponse,
    ) -> NiaServerResult<NiaIsListeningResponse> {
        unreachable!()
    }
}
