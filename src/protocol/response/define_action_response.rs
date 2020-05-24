use std::sync::MutexGuard;

use nia_interpreter_core::Interpreter;
use nia_interpreter_core::NiaInterpreterCommand;
use nia_interpreter_core::NiaInterpreterCommandResult;
use nia_interpreter_core::{EventLoopHandle, NiaDefineActionCommandResult};

use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::Serializable;
use crate::protocol::{NiaAction, NiaNamedAction};
use crate::protocol::{NiaActionEnum, NiaDefineActionRequest};
use nia_protocol_rust::DefineActionResponse;

#[derive(Debug, Clone)]
pub struct NiaDefineActionResponse {
    command_result: NiaDefineActionCommandResult,
}

fn make_define_action_command(action: NiaNamedAction) -> NiaInterpreterCommand {
    match action.get_action().get_action() {
        NiaActionEnum::KeyPress(action_key_press) => {
            NiaInterpreterCommand::make_define_key_press_action_command(
                action.get_action_name(),
                action_key_press.get_key_code()
            )
        }
        NiaActionEnum::KeyClick(action_key_click) => {
            NiaInterpreterCommand::make_define_key_click_action_command(
                action.get_action_name(),
                action_key_click.get_key_code()
            )
        }
        NiaActionEnum::KeyRelease(action_key_release) => {
            NiaInterpreterCommand::make_define_key_release_action_command(
                action.get_action_name(),
                action_key_release.get_key_code()
            )
        }
        NiaActionEnum::MouseButtonClick(action_mouse_button_click) => {
            NiaInterpreterCommand::make_define_mouse_button_click_action_command(
                action.get_action_name(),
                action_mouse_button_click.get_button_code()
            )
        }
        NiaActionEnum::MouseButtonPress(action_mouse_button_press) => {
            NiaInterpreterCommand::make_define_mouse_button_press_action_command(
                action.get_action_name(),
                action_mouse_button_press.get_button_code()
            )
        }
        NiaActionEnum::MouseButtonRelease(action_mouse_button_release) => {
            NiaInterpreterCommand::make_define_mouse_button_release_action_command(
                action.get_action_name(),
                action_mouse_button_release.get_button_code()
            )
        }
        NiaActionEnum::MouseAbsoluteMove(action_mouse_absolute_move) => {
            NiaInterpreterCommand::make_define_mouse_absolute_move_action_command(
                action.get_action_name(),
                action_mouse_absolute_move.get_x(),
                action_mouse_absolute_move.get_y(),
            )
        }
        NiaActionEnum::MouseRelativeMove(action_mouse_relative_move) => {
            NiaInterpreterCommand::make_define_mouse_relative_move_action_command(
                action.get_action_name(),
                action_mouse_relative_move.get_dx(),
                action_mouse_relative_move.get_dy(),
            )
        }
        NiaActionEnum::ExecuteCode(action_execute_code) => {
            NiaInterpreterCommand::make_define_execute_code_action_command(
                action.get_action_name(),
                action_execute_code.get_code(),
            )
        }
        NiaActionEnum::ExecuteFunction(action_execute_function) => {
            NiaInterpreterCommand::make_define_execute_code_action_command(
                action.get_action_name(),
                action_execute_function.get_function_name(),
            )
        }
        NiaActionEnum::ExecuteOSCommand(action_execute_os_command) => {
            NiaInterpreterCommand::make_define_execute_os_command_action_command(
                action.get_action_name(),
                action_execute_os_command.get_os_command(),
            )
        }
        NiaActionEnum::TextType(action_text_type) => {
            NiaInterpreterCommand::make_define_text_type_action_command(
                action.get_action_name(),
                action_text_type.get_text(),
            )
        }
        NiaActionEnum::Wait(action_wait) => {
            NiaInterpreterCommand::make_define_wait_action_command(
                action.get_action_name(),
                action_wait.get_ms(),
            )
        }
    }
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
