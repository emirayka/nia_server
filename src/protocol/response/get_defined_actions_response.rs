use std::sync::MutexGuard;

use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::EventLoopHandle;
use nia_interpreter_core::NiaGetDefinedActionsCommandResult;

use nia_protocol_rust::GetDefinedActionsResponse;

use crate::error::NiaServerError;
use crate::error::NiaServerResult;
use crate::protocol::NiaGetDefinedActionsRequest;
use crate::protocol::Serializable;


#[derive(Debug, Clone)]
pub struct NiaGetDefinedActionsResponse {
    command_result: NiaGetDefinedActionsCommandResult,
}

impl NiaGetDefinedActionsResponse {
    fn try_from(
        _nia_define_action_request: NiaGetDefinedActionsRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> Result<NiaGetDefinedActionsResponse, NiaServerError> {
        let interpreter_command =
            NiaInterpreterCommand::make_get_defined_actions();

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
            NiaInterpreterCommandResult::GetDefinedActions(
                command_result,
            ) => NiaGetDefinedActionsResponse { command_result },
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
        nia_define_action_request: NiaGetDefinedActionsRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaGetDefinedActionsResponse {
        println!("{:?}", nia_define_action_request);
        let try_result = NiaGetDefinedActionsResponse::try_from(
            nia_define_action_request,
            event_loop_handle,
        );

        match try_result {
            Ok(result) => result,
            Err(error) => {
                let message =
                    format!("Execution failure: {}", error.get_message());
                let command_result =
                    NiaGetDefinedActionsCommandResult::Failure(message);

                NiaGetDefinedActionsResponse { command_result }
            }
        }
    }
}

impl
Serializable<
    NiaGetDefinedActionsResponse,
    nia_protocol_rust::GetDefinedActionsResponse,
> for NiaGetDefinedActionsResponse
{
    fn to_pb(&self) -> GetDefinedActionsResponse {
        let command_result = &self.command_result;

        let mut get_defined_actions_response =
            nia_protocol_rust::GetDefinedActionsResponse::new();

        match command_result {
            NiaGetDefinedActionsCommandResult::Success(defined_actions) => {
                let mut success_result = nia_protocol_rust::GetDefinedActionsResponse_SuccessResult::new();

                for (name, action) in defined_actions {

                }

                get_defined_actions_response.set_success_result(success_result);
            }
            NiaGetDefinedActionsCommandResult::Error(error_message) => {
                let mut error_result =
                    nia_protocol_rust::GetDefinedActionsResponse_ErrorResult::new();

                error_result
                    .set_message(protobuf::Chars::from(error_message.clone()));
                get_defined_actions_response.set_error_result(error_result);
            }
            NiaGetDefinedActionsCommandResult::Failure(failure_message) => {
                let mut failure_result =
                    nia_protocol_rust::GetDefinedActionsResponse_FailureResult::new();

                failure_result.set_message(protobuf::Chars::from(
                    failure_message.clone(),
                ));
                get_defined_actions_response
                    .set_failure_result(failure_result);
            }
        }

        get_defined_actions_response
    }

    fn from_pb(
        object_pb: GetDefinedActionsResponse,
    ) -> NiaServerResult<NiaGetDefinedActionsResponse> {
        unreachable!()
    }
}
