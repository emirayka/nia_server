use nia_interpreter_core::EventLoopHandle;
use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaChangeMappingCommandResult;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use std::sync::MutexGuard;

use nia_protocol_rust::ChangeMappingResponse;

use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::Serializable;
use crate::protocol::{NiaChangeMappingRequest, NiaConvertable};
use crate::server::Server;

#[derive(Debug, Clone)]
pub struct NiaChangeMappingResponse {
    command_result: NiaChangeMappingCommandResult,
}

impl NiaChangeMappingResponse {
    fn try_from(
        nia_change_mapping_request: NiaChangeMappingRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaChangeMappingResponse, NiaServerError> {
        let (key_chords, action) = nia_change_mapping_request.into_tuple();

        let mut interpreter_key_chords = key_chords
            .into_iter()
            .map(|key_chord| key_chord.to_interpreter_repr())
            .collect();
        let interpreter_action = action.to_interpreter_repr();

        let interpreter_command =
            NiaInterpreterCommand::make_change_mapping_command(
                interpreter_key_chords,
                interpreter_action,
            );

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
            NiaInterpreterCommandResult::ChangeMapping(command_result) => {
                NiaChangeMappingResponse { command_result }
            }
            _ => {
                return NiaServerError::interpreter_error(
                    "Unexpected command result.",
                )
                .into();
            }
        };

        Ok(response)
    }

    pub fn from(
        nia_change_mapping_request: NiaChangeMappingRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaChangeMappingResponse {
        println!("{:?}", nia_change_mapping_request);
        let try_result = NiaChangeMappingResponse::try_from(
            nia_change_mapping_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());

                let command_result =
                    NiaChangeMappingCommandResult::Failure(message);

                NiaChangeMappingResponse { command_result }
            }
        }
    }
}

impl
    Serializable<
        NiaChangeMappingResponse,
        nia_protocol_rust::ChangeMappingResponse,
    > for NiaChangeMappingResponse
{
    fn to_pb(&self) -> nia_protocol_rust::ChangeMappingResponse {
        let result = &self.command_result;

        let mut change_mapping_response =
            nia_protocol_rust::ChangeMappingResponse::new();

        match result {
            NiaChangeMappingCommandResult::Success() => {
                let mut success_result =
                    nia_protocol_rust::ChangeMappingResponse_SuccessResult::new(
                    );

                success_result.set_message(protobuf::Chars::from(
                    String::from("Success."),
                ));
                change_mapping_response.set_success_result(success_result);
            }
            NiaChangeMappingCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::ChangeMappingResponse_ErrorResult::new();

                error_result
                    .set_message(protobuf::Chars::from(error_message.clone()));
                change_mapping_response.set_error_result(error_result);
            }
            NiaChangeMappingCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::ChangeMappingResponse_FailureResult::new(
                    );

                failure_result.set_message(protobuf::Chars::from(
                    failure_message.clone(),
                ));
                change_mapping_response.set_failure_result(failure_result);
            }
        }

        change_mapping_response
    }

    fn from_pb(
        object_pb: ChangeMappingResponse,
    ) -> NiaServerResult<NiaChangeMappingResponse> {
        unreachable!()
    }
}
