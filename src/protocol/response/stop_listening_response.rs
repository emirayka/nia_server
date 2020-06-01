use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{EventLoopHandle, NiaStopListeningCommandResult};

use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::{NiaConvertable, NiaStopListeningRequest, Serializable};
use nia_protocol_rust::StopListeningResponse;

#[derive(Debug, Clone)]
pub struct NiaStopListeningResponse {
    command_result: NiaStopListeningCommandResult,
}

impl NiaStopListeningResponse {
    fn try_from(
        nia_stop_listening_request: NiaStopListeningRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaStopListeningResponse, NiaServerError> {
        let interpreter_command =
            NiaInterpreterCommand::make_stop_listening_command();

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
            NiaInterpreterCommandResult::StopListening(command_result) => {
                NiaStopListeningResponse { command_result }
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
        nia_stop_listening_request: NiaStopListeningRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaStopListeningResponse {
        let try_result = NiaStopListeningResponse::try_from(
            nia_stop_listening_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaStopListeningCommandResult::Failure(message);

                NiaStopListeningResponse { command_result }
            }
        }
    }
}

impl
    Serializable<
        NiaStopListeningResponse,
        nia_protocol_rust::StopListeningResponse,
    > for NiaStopListeningResponse
{
    fn to_pb(&self) -> StopListeningResponse {
        let result = &self.command_result;

        let mut stop_listening_response =
            nia_protocol_rust::StopListeningResponse::new();

        match result {
            NiaStopListeningCommandResult::Success() => {
                let mut success_result =
                    nia_protocol_rust::StopListeningResponse_SuccessResult::new(
                    );

                success_result.set_message(protobuf::Chars::from(
                    String::from("Success."),
                ));
                stop_listening_response.set_success_result(success_result);
            }
            NiaStopListeningCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::StopListeningResponse_ErrorResult::new();

                error_result
                    .set_message(protobuf::Chars::from(error_message.clone()));
                stop_listening_response.set_error_result(error_result);
            }
            NiaStopListeningCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::StopListeningResponse_FailureResult::new(
                    );

                failure_result.set_message(protobuf::Chars::from(
                    failure_message.clone(),
                ));
                stop_listening_response.set_failure_result(failure_result);
            }
        }

        stop_listening_response
    }

    fn from_pb(
        object_pb: StopListeningResponse,
    ) -> NiaServerResult<NiaStopListeningResponse> {
        unreachable!()
    }
}
