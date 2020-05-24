use nia_interpreter_core::EventLoopHandle;
use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaDefineMappingCommandResult;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use std::sync::MutexGuard;

use nia_protocol_rust::DefineMappingResponse;

use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::Serializable;
use crate::protocol::{NiaConvertable, NiaDefineMappingRequest};
use crate::server::Server;

#[derive(Debug, Clone)]
pub struct NiaDefineMappingResponse {
    command_result: NiaDefineMappingCommandResult,
}

impl NiaDefineMappingResponse {
    fn try_from(
        nia_define_mapping_request: NiaDefineMappingRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaDefineMappingResponse, NiaServerError> {
        let mapping = nia_define_mapping_request
            .get_mapping()
            .to_interpreter_repr();

        let interpreter_command =
            NiaInterpreterCommand::make_define_mapping_command(mapping);

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
            NiaInterpreterCommandResult::DefineMapping(command_result) => {
                NiaDefineMappingResponse { command_result }
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
        nia_define_mapping_request: NiaDefineMappingRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaDefineMappingResponse {
        println!("{:?}", nia_define_mapping_request);
        let try_result = NiaDefineMappingResponse::try_from(
            nia_define_mapping_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());

                let command_result =
                    NiaDefineMappingCommandResult::Failure(message);

                NiaDefineMappingResponse { command_result }
            }
        }
    }
}

impl
    Serializable<
        NiaDefineMappingResponse,
        nia_protocol_rust::DefineMappingResponse,
    > for NiaDefineMappingResponse
{
    fn to_pb(&self) -> nia_protocol_rust::DefineMappingResponse {
        let result = &self.command_result;

        let mut define_mapping_response =
            nia_protocol_rust::DefineMappingResponse::new();

        match result {
            NiaDefineMappingCommandResult::Success() => {
                let mut success_result =
                    nia_protocol_rust::DefineMappingResponse_SuccessResult::new(
                    );

                success_result.set_message(protobuf::Chars::from(
                    String::from("Success."),
                ));
                define_mapping_response.set_success_result(success_result);
            }
            NiaDefineMappingCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::DefineMappingResponse_ErrorResult::new();

                error_result
                    .set_message(protobuf::Chars::from(error_message.clone()));
                define_mapping_response.set_error_result(error_result);
            }
            NiaDefineMappingCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::DefineMappingResponse_FailureResult::new(
                    );

                failure_result.set_message(protobuf::Chars::from(
                    failure_message.clone(),
                ));
                define_mapping_response.set_failure_result(failure_result);
            }
        }

        define_mapping_response
    }

    fn from_pb(
        object_pb: DefineMappingResponse,
    ) -> NiaServerResult<NiaDefineMappingResponse> {
        unreachable!()
    }
}
