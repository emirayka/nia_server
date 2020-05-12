use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{EventLoopHandle, NiaDefineModifierCommandResult};

use crate::error::NiaServerError;

use crate::protocol::NiaDefineModifierRequest;

#[derive(Debug, Clone)]
pub struct NiaDefineModifierResponse {
    command_result: NiaDefineModifierCommandResult,
}

impl NiaDefineModifierResponse {
    fn try_from(
        nia_define_modifier_request: NiaDefineModifierRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaDefineModifierResponse, NiaServerError> {
        let (keyboard_path, key_code, modifier_alias) =
            nia_define_modifier_request.into_tuple();

        let interpreter_command =
            NiaInterpreterCommand::make_define_modifier_command(
                keyboard_path,
                key_code,
                modifier_alias,
            );

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
            NiaInterpreterCommandResult::DefineModifier(command_result) => {
                NiaDefineModifierResponse { command_result }
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
        nia_define_modifier_request: NiaDefineModifierRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaDefineModifierResponse {
        println!("{:?}", nia_define_modifier_request);
        let try_result = NiaDefineModifierResponse::try_from(
            nia_define_modifier_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaDefineModifierCommandResult::Failure(message);

                NiaDefineModifierResponse { command_result }
            }
        }
    }
}

impl From<NiaDefineModifierResponse>
    for nia_protocol_rust::DefineModifierResponse
{
    fn from(nia_define_modifier_response: NiaDefineModifierResponse) -> Self {
        let result = nia_define_modifier_response.command_result;

        let mut define_modifier_response =
            nia_protocol_rust::DefineModifierResponse::new();

        match result {
            NiaDefineModifierCommandResult::Success() => {
                let mut success_result =
                    nia_protocol_rust::DefineModifierResponse_SuccessResult::new();

                success_result.set_message(protobuf::Chars::from(
                    String::from("Success."),
                ));
                define_modifier_response.set_success_result(success_result);
            }
            NiaDefineModifierCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::DefineModifierResponse_ErrorResult::new(
                    );

                error_result.set_message(protobuf::Chars::from(error_message));
                define_modifier_response.set_error_result(error_result);
            }
            NiaDefineModifierCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::DefineModifierResponse_FailureResult::new();

                failure_result
                    .set_message(protobuf::Chars::from(failure_message));
                define_modifier_response.set_failure_result(failure_result);
            }
        }

        define_modifier_response
    }
}
