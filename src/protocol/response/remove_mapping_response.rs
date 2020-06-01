use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{EventLoopHandle, NiaRemoveMappingCommandResult};

use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::{NiaConvertable, NiaRemoveMappingRequest, Serializable};
use nia_protocol_rust::RemoveMappingResponse;

#[derive(Debug, Clone)]
pub struct NiaRemoveMappingResponse {
    command_result: NiaRemoveMappingCommandResult,
}

impl NiaRemoveMappingResponse {
    fn try_from(
        nia_remove_mapping_request: NiaRemoveMappingRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaRemoveMappingResponse, NiaServerError> {
        let key_chords = nia_remove_mapping_request
            .take_key_chords()
            .iter()
            .map(|key_chord| key_chord.to_interpreter_repr())
            .collect::<Vec<nia_interpreter_core::KeyChord>>();

        let interpreter_command =
            NiaInterpreterCommand::make_remove_mapping_command(key_chords);

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
            NiaInterpreterCommandResult::RemoveMapping(command_result) => {
                NiaRemoveMappingResponse { command_result }
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
        nia_remove_mapping_request: NiaRemoveMappingRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaRemoveMappingResponse {
        let try_result = NiaRemoveMappingResponse::try_from(
            nia_remove_mapping_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaRemoveMappingCommandResult::Failure(message);

                NiaRemoveMappingResponse { command_result }
            }
        }
    }
}

impl
    Serializable<
        NiaRemoveMappingResponse,
        nia_protocol_rust::RemoveMappingResponse,
    > for NiaRemoveMappingResponse
{
    fn to_pb(&self) -> RemoveMappingResponse {
        let result = &self.command_result;

        let mut remove_mapping_response =
            nia_protocol_rust::RemoveMappingResponse::new();

        match result {
            NiaRemoveMappingCommandResult::Success() => {
                let mut success_result =
                    nia_protocol_rust::RemoveMappingResponse_SuccessResult::new(
                    );

                success_result.set_message(protobuf::Chars::from(
                    String::from("Success."),
                ));
                remove_mapping_response.set_success_result(success_result);
            }
            NiaRemoveMappingCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::RemoveMappingResponse_ErrorResult::new();

                error_result
                    .set_message(protobuf::Chars::from(error_message.clone()));
                remove_mapping_response.set_error_result(error_result);
            }
            NiaRemoveMappingCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::RemoveMappingResponse_FailureResult::new(
                    );

                failure_result.set_message(protobuf::Chars::from(
                    failure_message.clone(),
                ));
                remove_mapping_response.set_failure_result(failure_result);
            }
        }

        remove_mapping_response
    }

    fn from_pb(
        object_pb: RemoveMappingResponse,
    ) -> NiaServerResult<NiaRemoveMappingResponse> {
        unreachable!()
    }
}
