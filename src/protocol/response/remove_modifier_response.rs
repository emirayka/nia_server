use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{EventLoopHandle, NiaRemoveModifierCommandResult};

use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::{NiaRemoveModifierRequest, Serializable};
use nia_protocol_rust::RemoveModifierResponse;

#[derive(Debug, Clone)]
pub struct NiaRemoveModifierResponse {
    command_result: NiaRemoveModifierCommandResult,
}

impl NiaRemoveModifierResponse {
    fn try_from(
        nia_remove_modifier_request: NiaRemoveModifierRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaRemoveModifierResponse, NiaServerError> {
        let (path, key_code) =
            nia_remove_modifier_request.get_device_id_and_key_code();

        let interpreter_command =
            NiaInterpreterCommand::make_remove_modifier_command(path, key_code);

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
            NiaInterpreterCommandResult::RemoveModifier(command_result) => {
                NiaRemoveModifierResponse { command_result }
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
        nia_remove_modifier_request: NiaRemoveModifierRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaRemoveModifierResponse {
        println!("{:?}", nia_remove_modifier_request);
        let try_result = NiaRemoveModifierResponse::try_from(
            nia_remove_modifier_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaRemoveModifierCommandResult::Failure(message);

                NiaRemoveModifierResponse { command_result }
            }
        }
    }
}

impl
    Serializable<
        NiaRemoveModifierResponse,
        nia_protocol_rust::RemoveModifierResponse,
    > for NiaRemoveModifierResponse
{
    fn to_pb(&self) -> RemoveModifierResponse {
        let result = &self.command_result;

        let mut remove_modifier_response =
            nia_protocol_rust::RemoveModifierResponse::new();

        match result {
            NiaRemoveModifierCommandResult::Success() => {
                let mut success_result =
                    nia_protocol_rust::RemoveModifierResponse_SuccessResult::new();

                success_result
                    .set_message(protobuf::Chars::from(String::from("")));
                remove_modifier_response.set_success_result(success_result);
            }
            NiaRemoveModifierCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::RemoveModifierResponse_ErrorResult::new(
                    );

                error_result
                    .set_message(protobuf::Chars::from(error_message.clone()));
                remove_modifier_response.set_error_result(error_result);
            }
            NiaRemoveModifierCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::RemoveModifierResponse_FailureResult::new();

                failure_result.set_message(protobuf::Chars::from(
                    failure_message.clone(),
                ));
                remove_modifier_response.set_failure_result(failure_result);
            }
        }

        remove_modifier_response
    }

    fn from_pb(
        object_pb: RemoveModifierResponse,
    ) -> NiaServerResult<NiaRemoveModifierResponse> {
        unreachable!()
    }
}
