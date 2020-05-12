use crate::error::NiaServerError;
use crate::protocol::Modifier;
use crate::protocol::NiaGetDefinedModifiersRequest;
use std::sync::MutexGuard;

use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{
    EventLoopHandle, NiaGetDefinedModifiersCommandResult,
};

#[derive(Debug, Clone)]
pub struct NiaGetDefinedModifiersResponse {
    command_result: NiaGetDefinedModifiersCommandResult,
}

impl NiaGetDefinedModifiersResponse {
    fn try_from(
        nia_define_modifier_request: NiaGetDefinedModifiersRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaGetDefinedModifiersResponse, NiaServerError> {
        let interpreter_command =
            NiaInterpreterCommand::make_get_defined_modifiers();

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
            NiaInterpreterCommandResult::GetDefinedModifiers(
                command_result,
            ) => NiaGetDefinedModifiersResponse { command_result },
            _ => {
                return NiaServerError::interpreter_execution(
                    "Unexpected command result.",
                )
                .into();
            }
        };

        Ok(response)
    }

    pub fn from(
        nia_define_modifier_request: NiaGetDefinedModifiersRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaGetDefinedModifiersResponse {
        println!("{:?}", nia_define_modifier_request);
        let try_result = NiaGetDefinedModifiersResponse::try_from(
            nia_define_modifier_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaGetDefinedModifiersCommandResult::Failure(message);

                NiaGetDefinedModifiersResponse { command_result }
            }
        }
    }
}

impl From<NiaGetDefinedModifiersResponse>
    for nia_protocol_rust::GetDefinedModifiersResponse
{
    fn from(
        nia_get_defined_modifiers_response: NiaGetDefinedModifiersResponse,
    ) -> Self {
        let command_result = nia_get_defined_modifiers_response.command_result;

        let mut get_defined_modifiers_response =
            nia_protocol_rust::GetDefinedModifiersResponse::new();

        match command_result {
            NiaGetDefinedModifiersCommandResult::Success(defined_modifiers) => {
                let mut modifiers = Vec::new();

                for defined_modifier in defined_modifiers {
                    let mut modifier = nia_protocol_rust::GetDefinedModifiersResponse_Modifier::new();

                    modifier.set_keyboard_path(protobuf::Chars::from(
                        defined_modifier.0,
                    ));
                    modifier.set_key_code(defined_modifier.1);
                    modifier.set_modifier_alias(protobuf::Chars::from(
                        defined_modifier.2,
                    ));

                    modifiers.push(modifier);
                }

                let modifiers = protobuf::RepeatedField::from(modifiers);

                let mut success_result =
                    nia_protocol_rust::GetDefinedModifiersResponse_SuccessResult::new();

                success_result.set_modifiers(modifiers);
                get_defined_modifiers_response
                    .set_success_result(success_result);
            }
            NiaGetDefinedModifiersCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::GetDefinedModifiersResponse_ErrorResult::new();

                error_result.set_message(protobuf::Chars::from(error_message));
                get_defined_modifiers_response.set_error_result(error_result);
            }
            NiaGetDefinedModifiersCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::GetDefinedModifiersResponse_FailureResult::new();

                failure_result
                    .set_message(protobuf::Chars::from(failure_message));
                get_defined_modifiers_response
                    .set_failure_result(failure_result);
            }
        }

        get_defined_modifiers_response
    }
}
