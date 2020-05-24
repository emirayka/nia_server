use crate::error::{NiaServerError, NiaServerResult};
use crate::protocol::{
    NiaConvertable, NiaGetDefinedModifiersRequest, NiaModifierDescription,
    Serializable,
};
use std::sync::MutexGuard;

use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{
    EventLoopHandle, NiaGetDefinedModifiersCommandResult,
};
use nia_protocol_rust::{GetDefinedModifiersResponse, ModifierDescription};

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
            NiaInterpreterCommandResult::GetDefinedModifiers(
                command_result,
            ) => NiaGetDefinedModifiersResponse { command_result },
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

impl
    Serializable<
        NiaGetDefinedModifiersResponse,
        nia_protocol_rust::GetDefinedModifiersResponse,
    > for NiaGetDefinedModifiersResponse
{
    fn to_pb(&self) -> GetDefinedModifiersResponse {
        let command_result = &self.command_result;

        let mut get_defined_modifiers_response =
            nia_protocol_rust::GetDefinedModifiersResponse::new();

        match command_result {
            NiaGetDefinedModifiersCommandResult::Success(defined_modifiers) => {
                let modifiers = defined_modifiers
                    .iter()
                    .map(|interpreter_modifier| {
                        NiaModifierDescription::from_interpreter_repr(
                            interpreter_modifier,
                        )
                        .map(|modifier| modifier.to_pb())
                    })
                    .collect::<NiaServerResult<Vec<ModifierDescription>>>();

                match modifiers {
                    Ok(modifiers) => {
                        let modifiers =
                            protobuf::RepeatedField::from(modifiers);

                        let mut success_result =
                            nia_protocol_rust::GetDefinedModifiersResponse_SuccessResult::new();

                        success_result.set_modifier_descriptions(modifiers);
                        get_defined_modifiers_response
                            .set_success_result(success_result);
                    }
                    Err(error) => {
                        let message = error.get_message();

                        let mut error_result =
                            nia_protocol_rust::GetDefinedModifiersResponse_ErrorResult::new();

                        error_result.set_message(protobuf::Chars::from(
                            message.clone(),
                        ));
                        get_defined_modifiers_response
                            .set_error_result(error_result);
                    }
                }
            }
            NiaGetDefinedModifiersCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::GetDefinedModifiersResponse_ErrorResult::new();

                error_result
                    .set_message(protobuf::Chars::from(error_message.clone()));
                get_defined_modifiers_response.set_error_result(error_result);
            }
            NiaGetDefinedModifiersCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::GetDefinedModifiersResponse_FailureResult::new();

                failure_result.set_message(protobuf::Chars::from(
                    failure_message.clone(),
                ));
                get_defined_modifiers_response
                    .set_failure_result(failure_result);
            }
        }

        get_defined_modifiers_response
    }

    fn from_pb(
        object_pb: GetDefinedModifiersResponse,
    ) -> NiaServerResult<NiaGetDefinedModifiersResponse> {
        unreachable!()
    }
}
