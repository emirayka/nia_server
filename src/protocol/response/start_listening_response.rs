use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{EventLoopHandle, NiaStartListeningCommandResult};

use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::{NiaConvertable, NiaStartListeningRequest, Serializable};
use nia_protocol_rust::StartListeningResponse;

#[derive(Debug, Clone)]
pub struct NiaStartListeningResponse {
    command_result: NiaStartListeningCommandResult,
}

impl NiaStartListeningResponse {
    fn try_from(
        nia_start_listening_request: NiaStartListeningRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaStartListeningResponse, NiaServerError> {
        let interpreter_command =
            NiaInterpreterCommand::make_start_listening_command();

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
            NiaInterpreterCommandResult::StartListening(command_result) => {
                NiaStartListeningResponse { command_result }
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
        nia_start_listening_request: NiaStartListeningRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaStartListeningResponse {
        let try_result = NiaStartListeningResponse::try_from(
            nia_start_listening_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaStartListeningCommandResult::Failure(message);

                NiaStartListeningResponse { command_result }
            }
        }
    }
}

impl
    Serializable<
        NiaStartListeningResponse,
        nia_protocol_rust::StartListeningResponse,
    > for NiaStartListeningResponse
{
    fn to_pb(&self) -> StartListeningResponse {
        let result = &self.command_result;

        let mut start_listening_response =
            nia_protocol_rust::StartListeningResponse::new();

        match result {
            NiaStartListeningCommandResult::Success() => {
                let mut success_result =
                    nia_protocol_rust::StartListeningResponse_SuccessResult::new();

                success_result.set_message(protobuf::Chars::from(
                    String::from("Success."),
                ));
                start_listening_response.set_success_result(success_result);
            }
            NiaStartListeningCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::StartListeningResponse_ErrorResult::new(
                    );

                error_result
                    .set_message(protobuf::Chars::from(error_message.clone()));
                start_listening_response.set_error_result(error_result);
            }
            NiaStartListeningCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::StartListeningResponse_FailureResult::new();

                failure_result.set_message(protobuf::Chars::from(
                    failure_message.clone(),
                ));
                start_listening_response.set_failure_result(failure_result);
            }
        }

        start_listening_response
    }

    fn from_pb(
        object_pb: StartListeningResponse,
    ) -> NiaServerResult<NiaStartListeningResponse> {
        unreachable!()
    }
}
