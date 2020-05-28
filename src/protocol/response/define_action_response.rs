use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{EventLoopHandle, NiaDefineActionCommandResult};

use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::{NiaAction, NiaNamedAction};
use crate::protocol::{NiaActionEnum, NiaDefineActionRequest};
use crate::protocol::{NiaConvertable, Serializable};
use nia_protocol_rust::DefineActionResponse;

#[derive(Debug, Clone)]
pub struct NiaDefineActionResponse {
    command_result: NiaDefineActionCommandResult,
}

fn make_define_action_command(action: NiaNamedAction) -> NiaInterpreterCommand {
    let interpreter_named_action = action.to_interpreter_repr();

    NiaInterpreterCommand::make_define_action_command(interpreter_named_action)
}

impl NiaDefineActionResponse {
    fn try_from(
        nia_define_action_request: NiaDefineActionRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaDefineActionResponse, NiaServerError> {
        let action = nia_define_action_request.take_action();

        let interpreter_command = make_define_action_command(action);

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
            NiaInterpreterCommandResult::DefineAction(command_result) => {
                NiaDefineActionResponse { command_result }
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
        nia_define_action_request: NiaDefineActionRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaDefineActionResponse {
        println!("{:?}", nia_define_action_request);
        let try_result = NiaDefineActionResponse::try_from(
            nia_define_action_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaDefineActionCommandResult::Failure(message);

                NiaDefineActionResponse { command_result }
            }
        }
    }
}

impl
    Serializable<
        NiaDefineActionResponse,
        nia_protocol_rust::DefineActionResponse,
    > for NiaDefineActionResponse
{
    fn to_pb(&self) -> DefineActionResponse {
        let result = &self.command_result;

        let mut define_action_response =
            nia_protocol_rust::DefineActionResponse::new();

        match result {
            NiaDefineActionCommandResult::Success() => {
                let mut success_result =
                    nia_protocol_rust::DefineActionResponse_SuccessResult::new(
                    );

                success_result.set_message(protobuf::Chars::from(
                    String::from("Success."),
                ));
                define_action_response.set_success_result(success_result);
            }
            NiaDefineActionCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::DefineActionResponse_ErrorResult::new();

                error_result
                    .set_message(protobuf::Chars::from(error_message.clone()));
                define_action_response.set_error_result(error_result);
            }
            NiaDefineActionCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::DefineActionResponse_FailureResult::new(
                    );

                failure_result.set_message(protobuf::Chars::from(
                    failure_message.clone(),
                ));
                define_action_response.set_failure_result(failure_result);
            }
        }

        define_action_response
    }

    fn from_pb(
        object_pb: DefineActionResponse,
    ) -> NiaServerResult<NiaDefineActionResponse> {
        unreachable!()
    }
}
