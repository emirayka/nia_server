use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::ActionEnum;
use crate::protocol::Serializable;

use crate::protocol::domain::action::basic_actions::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NiaAction {
    action: ActionEnum,
    action_name: String,
}

impl NiaAction {
    pub fn new<S>(action_name: S, action: ActionEnum) -> NiaAction
    where
        S: Into<String>,
    {
        NiaAction {
            action_name: action_name.into(),
            action,
        }
    }

    pub fn get_action(&self) -> &ActionEnum {
        &self.action
    }

    pub fn get_action_name(&self) -> &str {
        &self.action_name
    }

    pub fn from_interpreter_action(
        action_name: String,
        action: nia_interpreter_core::Action,
    ) -> NiaAction {
        match action {
            nia_interpreter_core::Action::KeyPress(key_code) => NiaAction {
                action: ActionKeyPress::new(key_code).into(),
                action_name,
            },
            nia_interpreter_core::Action::KeyClick(key_code) => NiaAction {
                action: ActionKeyClick::new(key_code).into(),
                action_name,
            },
            nia_interpreter_core::Action::KeyRelease(key_code) => NiaAction {
                action: ActionKeyRelease::new(key_code).into(),
                action_name,
            },
            nia_interpreter_core::Action::MouseButtonPress(button_code) => {
                NiaAction {
                    action: ActionMouseButtonPress::new(button_code).into(),
                    action_name,
                }
            }
            nia_interpreter_core::Action::MouseButtonClick(button_code) => {
                NiaAction {
                    action: ActionMouseButtonClick::new(button_code).into(),
                    action_name,
                }
            }
            nia_interpreter_core::Action::MouseButtonRelease(button_code) => {
                NiaAction {
                    action: ActionMouseButtonRelease::new(button_code).into(),
                    action_name,
                }
            }
            nia_interpreter_core::Action::MouseAbsoluteMove(x, y) => {
                NiaAction {
                    action: ActionMouseAbsoluteMove::new(x, y).into(),
                    action_name,
                }
            }
            nia_interpreter_core::Action::MouseRelativeMove(dx, dy) => {
                NiaAction {
                    action: ActionMouseRelativeMove::new(dx, dy).into(),
                    action_name,
                }
            }
            nia_interpreter_core::Action::TextType(text_to_type) => NiaAction {
                action: ActionTextType::new(text_to_type).into(),
                action_name,
            },
            nia_interpreter_core::Action::Wait(ms) => NiaAction {
                action: ActionWait::new(ms).into(),
                action_name,
            },
            nia_interpreter_core::Action::ExecuteCode(code_to_execute) => {
                NiaAction {
                    action: ActionExecuteCode::new(code_to_execute).into(),
                    action_name,
                }
            }
            nia_interpreter_core::Action::ExecuteFunction(function_name) => {
                NiaAction {
                    action: ActionExecuteFunction::new(function_name).into(),
                    action_name,
                }
            }
            nia_interpreter_core::Action::ExecuteOSCommand(os_command) => {
                NiaAction {
                    action: ActionExecuteOSCommand::new(os_command).into(),
                    action_name,
                }
            }
        }
    }
}

impl Serializable<NiaAction, nia_protocol_rust::Action> for NiaAction {
    fn to_pb(&self) -> nia_protocol_rust::Action {
        let mut action_pb = nia_protocol_rust::Action::new();

        match &self.action {
            ActionEnum::KeyClick(action_key_click) => {
                let action_key_click_pb = action_key_click.to_pb();

                action_pb.set_action_key_click(action_key_click_pb)
            }
            ActionEnum::KeyPress(action_key_press) => {
                let action_key_press_pb = action_key_press.to_pb();

                action_pb.set_action_key_press(action_key_press_pb)
            }
            ActionEnum::KeyRelease(action_key_release) => {
                let action_key_release_pb = action_key_release.to_pb();

                action_pb.set_action_key_release(action_key_release_pb)
            }

            ActionEnum::MouseButtonClick(action_mouse_button_click) => {
                let action_mouse_button_click_pb =
                    action_mouse_button_click.to_pb();

                action_pb
                    .set_action_mouse_button_click(action_mouse_button_click_pb)
            }
            ActionEnum::MouseButtonPress(action_mouse_button_press) => {
                let action_mouse_button_press_pb =
                    action_mouse_button_press.to_pb();

                action_pb
                    .set_action_mouse_button_press(action_mouse_button_press_pb)
            }
            ActionEnum::MouseButtonRelease(action_mouse_button_release) => {
                let action_mouse_button_release_pb =
                    action_mouse_button_release.to_pb();

                action_pb.set_action_mouse_button_release(
                    action_mouse_button_release_pb,
                )
            }

            ActionEnum::MouseAbsoluteMove(action_mouse_absolute_move) => {
                let action_mouse_absolute_move_pb =
                    action_mouse_absolute_move.to_pb();

                action_pb.set_action_mouse_absolute_move(
                    action_mouse_absolute_move_pb,
                )
            }
            ActionEnum::MouseRelativeMove(action_mouse_relative_move) => {
                let action_mouse_relative_move_pb =
                    action_mouse_relative_move.to_pb();

                action_pb.set_action_mouse_relative_move(
                    action_mouse_relative_move_pb,
                )
            }

            ActionEnum::ExecuteCode(action_execute_code) => {
                let action_execute_code_pb = action_execute_code.to_pb();

                action_pb.set_action_execute_code(action_execute_code_pb)
            }
            ActionEnum::ExecuteFunction(action_execute_function) => {
                let action_execute_function_pb =
                    action_execute_function.to_pb();

                action_pb
                    .set_action_execute_function(action_execute_function_pb)
            }
            ActionEnum::ExecuteOSCommand(action_execute_os_command) => {
                let action_execute_os_command_pb =
                    action_execute_os_command.to_pb();

                action_pb
                    .set_action_execute_os_command(action_execute_os_command_pb)
            }
            ActionEnum::TextType(action_text_type) => {
                let action_text_type_pb = action_text_type.to_pb();

                action_pb.set_action_text_type(action_text_type_pb)
            }
            ActionEnum::Wait(action_wait) => {
                let action_wait_pb = action_wait.to_pb();

                action_pb.set_action_wait(action_wait_pb)
            }
        }

        action_pb
            .set_action_name(protobuf::Chars::from(self.action_name.clone()));

        action_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::Action,
    ) -> NiaServerResult<NiaAction> {
        let mut object_pb = object_pb;

        let action_enum = if object_pb.has_action_key_click() {
            let action_key_click_pb = object_pb.take_action_key_click();

            ActionKeyClick::from_pb(action_key_click_pb)?.into()
        } else if object_pb.has_action_key_press() {
            let action_key_press_pb = object_pb.take_action_key_press();

            ActionKeyPress::from_pb(action_key_press_pb)?.into()
        } else if object_pb.has_action_key_release() {
            let action_key_release_pb = object_pb.take_action_key_release();

            ActionKeyRelease::from_pb(action_key_release_pb)?.into()
        } else if object_pb.has_action_mouse_button_click() {
            let action_mouse_button_click_pb =
                object_pb.take_action_mouse_button_click();

            ActionMouseButtonClick::from_pb(action_mouse_button_click_pb)?
                .into()
        } else if object_pb.has_action_mouse_button_press() {
            let action_mouse_button_press_pb =
                object_pb.take_action_mouse_button_press();

            ActionMouseButtonPress::from_pb(action_mouse_button_press_pb)?
                .into()
        } else if object_pb.has_action_mouse_button_release() {
            let action_mouse_button_release_pb =
                object_pb.take_action_mouse_button_release();

            ActionMouseButtonRelease::from_pb(action_mouse_button_release_pb)?
                .into()
        } else if object_pb.has_action_mouse_absolute_move() {
            let action_mouse_absolute_move_pb =
                object_pb.take_action_mouse_absolute_move();

            ActionMouseAbsoluteMove::from_pb(action_mouse_absolute_move_pb)?
                .into()
        } else if object_pb.has_action_mouse_relative_move() {
            let action_mouse_relative_move_pb =
                object_pb.take_action_mouse_relative_move();

            ActionMouseRelativeMove::from_pb(action_mouse_relative_move_pb)?
                .into()
        } else if object_pb.has_action_execute_code() {
            let action_execute_code_pb = object_pb.take_action_execute_code();

            ActionExecuteCode::from_pb(action_execute_code_pb)?.into()
        } else if object_pb.has_action_execute_function() {
            let action_execute_function_pb =
                object_pb.take_action_execute_function();

            ActionExecuteFunction::from_pb(action_execute_function_pb)?.into()
        } else if object_pb.has_action_execute_os_command() {
            let action_execute_os_command_pb =
                object_pb.take_action_execute_os_command();

            ActionExecuteOSCommand::from_pb(action_execute_os_command_pb)?
                .into()
        } else if object_pb.has_action_text_type() {
            let action_text_type_pb = object_pb.take_action_text_type();

            ActionTextType::from_pb(action_text_type_pb)?.into()
        } else if object_pb.has_action_wait() {
            let action_wait_pb = object_pb.take_action_wait();

            ActionWait::from_pb(action_wait_pb)?.into()
        } else {
            return NiaServerError::deserialization_error(
                "Invalid action type.",
            )
            .into();
        };

        let action = NiaAction {
            action_name: object_pb.take_action_name().to_string(),
            action: action_enum,
        };

        Ok(action)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes_action_key_click() {
        let action_name = String::from("key-click");
        let action = ActionKeyClick::new(1).into();

        let expected = NiaAction {
            action_name,
            action,
        };

        let bytes = expected.to_bytes().unwrap();
        let actual = NiaAction::from_bytes(bytes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn serializes_and_deserializes_action_key_press() {
        let action_name = String::from("key-press");
        let action = ActionKeyPress::new(1).into();

        let expected = NiaAction {
            action_name,
            action,
        };

        let bytes = expected.to_bytes().unwrap();
        let actual = NiaAction::from_bytes(bytes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn serializes_and_deserializes_action_key_release() {
        let action_name = String::from("key-release");
        let action = ActionKeyRelease::new(1).into();

        let expected = NiaAction {
            action_name,
            action,
        };

        let bytes = expected.to_bytes().unwrap();
        let mut actual = NiaAction::from_bytes(bytes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn serializes_and_deserializes_action_mouse_button_click() {
        let action_name = String::from("mouse-button-click");
        let action = ActionMouseButtonClick::new(1).into();

        let expected = NiaAction {
            action_name,
            action,
        };

        let bytes = expected.to_bytes().unwrap();
        let actual = NiaAction::from_bytes(bytes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn serializes_and_deserializes_action_mouse_button_press() {
        let action_name = String::from("mouse-button-press");
        let action = ActionMouseButtonPress::new(1).into();

        let expected = NiaAction {
            action_name,
            action,
        };

        let bytes = expected.to_bytes().unwrap();
        let actual = NiaAction::from_bytes(bytes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn serializes_and_deserializes_action_mouse_button_release() {
        let action_name = String::from("mouse-button-release");
        let action = ActionMouseButtonRelease::new(1).into();

        let expected = NiaAction {
            action_name,
            action,
        };

        let bytes = expected.to_bytes().unwrap();
        let actual = NiaAction::from_bytes(bytes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn serializes_and_deserializes_action_mouse_absolute_move() {
        let action_name = String::from("mouse-absolute-move");
        let action = ActionMouseAbsoluteMove::new(100, 100).into();

        let expected = NiaAction {
            action_name,
            action,
        };

        let bytes = expected.to_bytes().unwrap();
        let actual = NiaAction::from_bytes(bytes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn serializes_and_deserializes_action_mouse_relative_move() {
        let action_name = String::from("mouse-relative-move");
        let action = ActionMouseRelativeMove::new(100, 100).into();

        let expected = NiaAction {
            action_name,
            action,
        };

        let bytes = expected.to_bytes().unwrap();
        let actual = NiaAction::from_bytes(bytes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn serializes_and_deserializes_action_execute_code() {
        let action_name = String::from("execute-code");
        let action = ActionExecuteCode::new("(+ 1 2)").into();

        let expected = NiaAction {
            action_name,
            action,
        };

        let bytes = expected.to_bytes().unwrap();
        let actual = NiaAction::from_bytes(bytes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn serializes_and_deserializes_action_execute_function() {
        let action_name = String::from("execute-function");
        let action = ActionExecuteFunction::new("function").into();

        let expected = NiaAction {
            action_name,
            action,
        };

        let bytes = expected.to_bytes().unwrap();
        let actual = NiaAction::from_bytes(bytes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn serializes_and_deserializes_action_execute_os_command() {
        let action_name = String::from("execute-os-command");
        let action = ActionExecuteOSCommand::new("ls").into();

        let expected = NiaAction {
            action_name,
            action,
        };

        let bytes = expected.to_bytes().unwrap();
        let actual = NiaAction::from_bytes(bytes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn serializes_and_deserializes_action_text_type() {
        let action_name = String::from("text-type");
        let action = ActionTextType::new("arst").into();

        let expected = NiaAction {
            action_name,
            action,
        };

        let bytes = expected.to_bytes().unwrap();
        let actual = NiaAction::from_bytes(bytes).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn serializes_and_deserializes_action_wait() {
        let action_name = String::from("wait");
        let action = ActionWait::new(1000).into();

        let expected = NiaAction {
            action_name,
            action,
        };

        let bytes = expected.to_bytes().unwrap();
        let actual = NiaAction::from_bytes(bytes).unwrap();

        assert_eq!(expected, actual);
    }
}
