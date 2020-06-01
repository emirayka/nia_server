use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{EventLoopHandle, NiaRemoveActionCommandResult};

use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::{NiaRemoveActionRequest, Serializable};
use nia_protocol_rust::RemoveActionResponse;

#[derive(Debug, Clone)]
pub struct NiaRemoveActionResponse {
    command_result: NiaRemoveActionCommandResult,
}

impl NiaRemoveActionResponse {
    fn try_from(
        nia_remove_action_request: NiaRemoveActionRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaRemoveActionResponse, NiaServerError> {
        let action_name = nia_remove_action_request.take_action_name();

        let interpreter_command =
            NiaInterpreterCommand::make_remove_action_command(action_name);

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
            NiaInterpreterCommandResult::RemoveAction(command_result) => {
                NiaRemoveActionResponse { command_result }
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
        nia_remove_action_request: NiaRemoveActionRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaRemoveActionResponse {
        let try_result = NiaRemoveActionResponse::try_from(
            nia_remove_action_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaRemoveActionCommandResult::Failure(message);

                NiaRemoveActionResponse { command_result }
            }
        }
    }
}

impl
    Serializable<
        NiaRemoveActionResponse,
        nia_protocol_rust::RemoveActionResponse,
    > for NiaRemoveActionResponse
{
    fn to_pb(&self) -> RemoveActionResponse {
        let result = &self.command_result;

        let mut remove_action_response =
            nia_protocol_rust::RemoveActionResponse::new();

        match result {
            NiaRemoveActionCommandResult::Success() => {
                let mut success_result =
                    nia_protocol_rust::RemoveActionResponse_SuccessResult::new(
                    );

                success_result.set_message(protobuf::Chars::from(
                    String::from("Success."),
                ));
                remove_action_response.set_success_result(success_result);
            }
            NiaRemoveActionCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::RemoveActionResponse_ErrorResult::new();

                error_result
                    .set_message(protobuf::Chars::from(error_message.clone()));
                remove_action_response.set_error_result(error_result);
            }
            NiaRemoveActionCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::RemoveActionResponse_FailureResult::new(
                    );

                failure_result.set_message(protobuf::Chars::from(
                    failure_message.clone(),
                ));
                remove_action_response.set_failure_result(failure_result);
            }
        }

        remove_action_response
    }

    fn from_pb(
        object_pb: RemoveActionResponse,
    ) -> NiaServerResult<NiaRemoveActionResponse> {
        unreachable!()
    }
}
