use std::sync::MutexGuard;

use nia_interpreter_core::EventLoopHandle;
use nia_interpreter_core::NiaGetDefinedMappingsCommandResult;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;

use nia_protocol_rust::GetDefinedMappingsResponse;

use crate::error::NiaServerError;
use crate::error::NiaServerResult;
use crate::protocol::{NiaConvertable, Serializable};
use crate::protocol::{NiaGetDefinedMappingsRequest, NiaMapping};

#[derive(Debug, Clone)]
pub struct NiaGetDefinedMappingsResponse {
    command_result: NiaGetDefinedMappingsCommandResult,
}

impl NiaGetDefinedMappingsResponse {
    fn try_from(
        _nia_define_mapping_request: NiaGetDefinedMappingsRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaGetDefinedMappingsResponse, NiaServerError> {
        let interpreter_command =
            NiaInterpreterCommand::make_get_defined_mappings_command();

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
            NiaInterpreterCommandResult::GetDefinedMappings(command_result) => {
                NiaGetDefinedMappingsResponse { command_result }
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
        nia_define_mapping_request: NiaGetDefinedMappingsRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaGetDefinedMappingsResponse {
        let try_result = NiaGetDefinedMappingsResponse::try_from(
            nia_define_mapping_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaGetDefinedMappingsCommandResult::Failure(message);

                NiaGetDefinedMappingsResponse { command_result }
            }
        }
    }
}

impl
    Serializable<
        NiaGetDefinedMappingsResponse,
        nia_protocol_rust::GetDefinedMappingsResponse,
    > for NiaGetDefinedMappingsResponse
{
    fn to_pb(&self) -> GetDefinedMappingsResponse {
        let command_result = &self.command_result;

        let mut get_defined_mappings_response =
            nia_protocol_rust::GetDefinedMappingsResponse::new();

        match command_result {
            NiaGetDefinedMappingsCommandResult::Success(defined_mappings) => {
                let mut mappings = defined_mappings
                    .iter()
                    .map(|interpreter_mapping| {
                        NiaMapping::from_interpreter_repr(interpreter_mapping)
                            .map(|mapping_result| mapping_result.to_pb())
                    })
                    .collect::<NiaServerResult<Vec<nia_protocol_rust::Mapping>>>();

                match mappings {
                    Ok(mappings) => {
                        let repeated =
                            protobuf::RepeatedField::from_vec(mappings);

                        let mut success_result = nia_protocol_rust::GetDefinedMappingsResponse_SuccessResult::new();
                        success_result.set_mappings(repeated);

                        get_defined_mappings_response
                            .set_success_result(success_result);
                    }
                    Err(error) => {
                        let message = error.get_message();
                        let mut error_result =
                            nia_protocol_rust::GetDefinedMappingsResponse_ErrorResult::new();

                        error_result
                            .set_message(protobuf::Chars::from(message));
                        get_defined_mappings_response
                            .set_error_result(error_result);
                    }
                }
            }
            NiaGetDefinedMappingsCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::GetDefinedMappingsResponse_ErrorResult::new();

                error_result
                    .set_message(protobuf::Chars::from(error_message.clone()));
                get_defined_mappings_response.set_error_result(error_result);
            }
            NiaGetDefinedMappingsCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::GetDefinedMappingsResponse_FailureResult::new();

                failure_result.set_message(protobuf::Chars::from(
                    failure_message.clone(),
                ));
                get_defined_mappings_response
                    .set_failure_result(failure_result);
            }
        }

        get_defined_mappings_response
    }

    fn from_pb(
        object_pb: GetDefinedMappingsResponse,
    ) -> NiaServerResult<NiaGetDefinedMappingsResponse> {
        unreachable!()
    }
}
