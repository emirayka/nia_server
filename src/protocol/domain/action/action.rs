use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::Serializable;
use crate::protocol::{NiaActionEnum, NiaConvertable};

use crate::protocol::domain::action::basic_actions::*;
use nia_interpreter_core::Action;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NiaAction {
    action: NiaActionEnum,
}

impl NiaAction {
    pub fn new(action: NiaActionEnum) -> NiaAction {
        NiaAction { action }
    }

    pub fn get_action(&self) -> &NiaActionEnum {
        &self.action
    }
}

impl From<NiaActionEnum> for NiaAction {
    fn from(action: NiaActionEnum) -> Self {
        NiaAction::new(action)
    }
}

impl NiaConvertable<NiaAction, nia_interpreter_core::Action> for NiaAction {
    fn to_interpreter_repr(&self) -> nia_interpreter_core::Action {
        let action = match &self.action {
            NiaActionEnum::KeyClick(action_key_click) => {
                nia_interpreter_core::Action::KeyClick(
                    action_key_click.get_key_code(),
                )
            }
            NiaActionEnum::KeyPress(action_key_press) => {
                nia_interpreter_core::Action::KeyPress(
                    action_key_press.get_key_code(),
                )
            }
            NiaActionEnum::KeyRelease(action_key_release) => {
                nia_interpreter_core::Action::KeyRelease(
                    action_key_release.get_key_code(),
                )
            }

            NiaActionEnum::MouseButtonClick(action_mouse_button_click) => {
                nia_interpreter_core::Action::MouseButtonClick(
                    action_mouse_button_click.get_button_code(),
                )
            }
            NiaActionEnum::MouseButtonPress(action_mouse_button_press) => {
                nia_interpreter_core::Action::MouseButtonPress(
                    action_mouse_button_press.get_button_code(),
                )
            }
            NiaActionEnum::MouseButtonRelease(action_mouse_button_release) => {
                nia_interpreter_core::Action::MouseButtonRelease(
                    action_mouse_button_release.get_button_code(),
                )
            }

            NiaActionEnum::ControlKeyClick(action_control_key_click) => {
                nia_interpreter_core::Action::ControlKeyClick(
                    action_control_key_click.get_key_code(),
                )
            }
            NiaActionEnum::FunctionKeyClick(action_function_key_click) => {
                nia_interpreter_core::Action::FunctionKeyClick(
                    action_function_key_click.get_key_code(),
                )
            }
            NiaActionEnum::KPKeyClick(action_kp_key_click) => {
                nia_interpreter_core::Action::KPKeyClick(
                    action_kp_key_click.get_key_code(),
                )
            }
            NiaActionEnum::MouseButtonKeyClick(
                action_mouse_button_key_click,
            ) => nia_interpreter_core::Action::MouseButtonKeyClick(
                action_mouse_button_key_click.get_key_code(),
            ),
            NiaActionEnum::MultimediaKeyClick(action_multimedia_key_click) => {
                nia_interpreter_core::Action::MultimediaKeyClick(
                    action_multimedia_key_click.get_key_code(),
                )
            }
            NiaActionEnum::NumberKeyClick(action_number_key_click) => {
                nia_interpreter_core::Action::NumberKeyClick(
                    action_number_key_click.get_key_code(),
                )
            }
            NiaActionEnum::TextKeyClick(action_text_key_click) => {
                nia_interpreter_core::Action::TextKeyClick(
                    action_text_key_click.get_key_code(),
                )
            }

            NiaActionEnum::MouseAbsoluteMove(action_mouse_absolute_move) => {
                nia_interpreter_core::Action::MouseAbsoluteMove(
                    action_mouse_absolute_move.get_x(),
                    action_mouse_absolute_move.get_y(),
                )
            }
            NiaActionEnum::MouseRelativeMove(action_mouse_relative_move) => {
                nia_interpreter_core::Action::MouseRelativeMove(
                    action_mouse_relative_move.get_dx(),
                    action_mouse_relative_move.get_dy(),
                )
            }

            NiaActionEnum::Wait(action_wait) => {
                nia_interpreter_core::Action::Wait(action_wait.get_ms())
            }
            NiaActionEnum::TextType(action_text_type) => {
                nia_interpreter_core::Action::TextType(String::from(
                    action_text_type.get_text(),
                ))
            }

            NiaActionEnum::ExecuteCode(action_execute_code) => {
                nia_interpreter_core::Action::ExecuteCode(String::from(
                    action_execute_code.get_code(),
                ))
            }
            NiaActionEnum::ExecuteFunction(action_execute_function) => {
                nia_interpreter_core::Action::ExecuteFunction(String::from(
                    action_execute_function.get_function_name(),
                ))
            }
            NiaActionEnum::ExecuteOSCommand(action_execute_os_command) => {
                nia_interpreter_core::Action::ExecuteOSCommand(String::from(
                    action_execute_os_command.get_os_command(),
                ))
            }
            NiaActionEnum::ExecuteNamedAction(action_execute_named_action) => {
                nia_interpreter_core::Action::ExecuteNamedAction(String::from(
                    action_execute_named_action.get_action_name(),
                ))
            }
        };

        action
    }

    fn from_interpreter_repr(
        interpreter_action: &nia_interpreter_core::Action,
    ) -> NiaServerResult<NiaAction> {
        let action = match interpreter_action {
            nia_interpreter_core::Action::KeyPress(key_code) => NiaAction {
                action: ActionKeyPress::new(*key_code).into(),
            },
            nia_interpreter_core::Action::KeyClick(key_code) => NiaAction {
                action: ActionKeyClick::new(*key_code).into(),
            },
            nia_interpreter_core::Action::KeyRelease(key_code) => NiaAction {
                action: ActionKeyRelease::new(*key_code).into(),
            },

            nia_interpreter_core::Action::MouseButtonPress(button_code) => {
                NiaAction {
                    action: ActionMouseButtonPress::new(*button_code).into(),
                }
            }
            nia_interpreter_core::Action::MouseButtonClick(button_code) => {
                NiaAction {
                    action: ActionMouseButtonClick::new(*button_code).into(),
                }
            }
            nia_interpreter_core::Action::MouseButtonRelease(button_code) => {
                NiaAction {
                    action: ActionMouseButtonRelease::new(*button_code).into(),
                }
            }

            Action::TextKeyClick(key_code) => {
                NiaAction {
                    action: ActionTextKeyClick::new(*key_code).into(),
                }
            }
            Action::NumberKeyClick(key_code) => {
                NiaAction {
                    action: ActionNumberKeyClick::new(*key_code).into(),
                }
            }
            Action::FunctionKeyClick(key_code) => {
                NiaAction {
                    action: ActionFunctionKeyClick::new(*key_code).into(),
                }
            }
            Action::ControlKeyClick(key_code) => {
                NiaAction {
                    action: ActionControlKeyClick::new(*key_code).into(),
                }
            }
            Action::KPKeyClick(key_code) => {
                NiaAction {
                    action: ActionKPKeyClick::new(*key_code).into(),
                }
            }
            Action::MultimediaKeyClick(key_code) => {
                NiaAction {
                    action: ActionMultimediaKeyClick::new(*key_code).into(),
                }
            }
            Action::MouseButtonKeyClick(key_code) => {
                NiaAction {
                    action: ActionMouseButtonKeyClick::new(*key_code).into(),
                }
            }

            nia_interpreter_core::Action::MouseAbsoluteMove(x, y) => {
                NiaAction {
                    action: ActionMouseAbsoluteMove::new(*x, *y).into(),
                }
            }
            nia_interpreter_core::Action::MouseRelativeMove(dx, dy) => {
                NiaAction {
                    action: ActionMouseRelativeMove::new(*dx, *dy).into(),
                }
            }
            nia_interpreter_core::Action::Wait(ms) => NiaAction {
                action: ActionWait::new(*ms).into(),
            },
            nia_interpreter_core::Action::TextType(text_to_type) => NiaAction {
                action: ActionTextType::new(text_to_type).into(),
            },
            nia_interpreter_core::Action::ExecuteCode(code_to_execute) => {
                NiaAction {
                    action: ActionExecuteCode::new(code_to_execute).into(),
                }
            }
            nia_interpreter_core::Action::ExecuteFunction(function_name) => {
                NiaAction {
                    action: ActionExecuteFunction::new(function_name).into(),
                }
            }
            nia_interpreter_core::Action::ExecuteOSCommand(os_command) => {
                NiaAction {
                    action: ActionExecuteOSCommand::new(os_command).into(),
                }
            }
            nia_interpreter_core::Action::ExecuteNamedAction(action_name) => {
                NiaAction {
                    action: ActionExecuteNamedAction::new(action_name).into(),
                }
            }
            nia_interpreter_core::Action::ExecuteFunctionValue(function_value) => {
                return NiaServerError::interpreter_error(
                    "It must not be possible to serialized execute function value action directly."
                ).into()
            }
        };

        Ok(action)
    }
}

impl Serializable<NiaAction, nia_protocol_rust::Action> for NiaAction {
    fn to_pb(&self) -> nia_protocol_rust::Action {
        let mut action_pb = nia_protocol_rust::Action::new();

        match &self.action {
            NiaActionEnum::KeyClick(action_key_click) => {
                let action_key_click_pb = action_key_click.to_pb();

                action_pb.set_action_key_click(action_key_click_pb)
            }
            NiaActionEnum::KeyPress(action_key_press) => {
                let action_key_press_pb = action_key_press.to_pb();

                action_pb.set_action_key_press(action_key_press_pb)
            }
            NiaActionEnum::KeyRelease(action_key_release) => {
                let action_key_release_pb = action_key_release.to_pb();

                action_pb.set_action_key_release(action_key_release_pb)
            }

            NiaActionEnum::MouseButtonClick(action_mouse_button_click) => {
                let action_mouse_button_click_pb =
                    action_mouse_button_click.to_pb();

                action_pb
                    .set_action_mouse_button_click(action_mouse_button_click_pb)
            }
            NiaActionEnum::MouseButtonPress(action_mouse_button_press) => {
                let action_mouse_button_press_pb =
                    action_mouse_button_press.to_pb();

                action_pb
                    .set_action_mouse_button_press(action_mouse_button_press_pb)
            }
            NiaActionEnum::MouseButtonRelease(action_mouse_button_release) => {
                let action_mouse_button_release_pb =
                    action_mouse_button_release.to_pb();

                action_pb.set_action_mouse_button_release(
                    action_mouse_button_release_pb,
                )
            }

            NiaActionEnum::ControlKeyClick(action_control_key_click) => {
                let action_control_key_click_pb =
                    action_control_key_click.to_pb();

                action_pb
                    .set_action_control_key_click(action_control_key_click_pb)
            }
            NiaActionEnum::FunctionKeyClick(action_function_key_click) => {
                let action_function_key_click_pb =
                    action_function_key_click.to_pb();

                action_pb
                    .set_action_function_key_click(action_function_key_click_pb)
            }
            NiaActionEnum::KPKeyClick(action_kp_key_click) => {
                let action_kp_key_click_pb = action_kp_key_click.to_pb();

                action_pb.set_action_kp_key_click(action_kp_key_click_pb)
            }
            NiaActionEnum::MouseButtonKeyClick(
                action_mouse_button_key_click,
            ) => {
                let action_mouse_button_key_click_pb =
                    action_mouse_button_key_click.to_pb();

                action_pb.set_action_mouse_button_key_click(
                    action_mouse_button_key_click_pb,
                )
            }
            NiaActionEnum::MultimediaKeyClick(action_multimedia_key_click) => {
                let action_multimedia_key_click_pb =
                    action_multimedia_key_click.to_pb();

                action_pb.set_action_multimedia_key_click(
                    action_multimedia_key_click_pb,
                )
            }
            NiaActionEnum::NumberKeyClick(action_number_key_click) => {
                let action_number_key_click_pb =
                    action_number_key_click.to_pb();

                action_pb
                    .set_action_number_key_click(action_number_key_click_pb)
            }
            NiaActionEnum::TextKeyClick(action_text_key_click) => {
                let action_text_key_click_pb = action_text_key_click.to_pb();

                action_pb.set_action_text_key_click(action_text_key_click_pb)
            }

            NiaActionEnum::MouseAbsoluteMove(action_mouse_absolute_move) => {
                let action_mouse_absolute_move_pb =
                    action_mouse_absolute_move.to_pb();

                action_pb.set_action_mouse_absolute_move(
                    action_mouse_absolute_move_pb,
                )
            }
            NiaActionEnum::MouseRelativeMove(action_mouse_relative_move) => {
                let action_mouse_relative_move_pb =
                    action_mouse_relative_move.to_pb();

                action_pb.set_action_mouse_relative_move(
                    action_mouse_relative_move_pb,
                )
            }

            NiaActionEnum::Wait(action_wait) => {
                let action_wait_pb = action_wait.to_pb();

                action_pb.set_action_wait(action_wait_pb)
            }
            NiaActionEnum::TextType(action_text_type) => {
                let action_text_type_pb = action_text_type.to_pb();

                action_pb.set_action_text_type(action_text_type_pb)
            }

            NiaActionEnum::ExecuteCode(action_execute_code) => {
                let action_execute_code_pb = action_execute_code.to_pb();

                action_pb.set_action_execute_code(action_execute_code_pb)
            }
            NiaActionEnum::ExecuteFunction(action_execute_function) => {
                let action_execute_function_pb =
                    action_execute_function.to_pb();

                action_pb
                    .set_action_execute_function(action_execute_function_pb)
            }
            NiaActionEnum::ExecuteOSCommand(action_execute_os_command) => {
                let action_execute_os_command_pb =
                    action_execute_os_command.to_pb();

                action_pb
                    .set_action_execute_os_command(action_execute_os_command_pb)
            }
            NiaActionEnum::ExecuteNamedAction(action_execute_named_action) => {
                let action_execute_named_action_pb =
                    action_execute_named_action.to_pb();

                action_pb.set_action_execute_named_action(
                    action_execute_named_action_pb,
                )
            }
        }

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
        } else if object_pb.has_action_text_key_click() {
            let action_text_key_click_pb =
                object_pb.take_action_text_key_click();

            ActionTextKeyClick::from_pb(action_text_key_click_pb)?.into()
        } else if object_pb.has_action_number_key_click() {
            let action_number_key_click_pb =
                object_pb.take_action_number_key_click();

            ActionNumberKeyClick::from_pb(action_number_key_click_pb)?.into()
        } else if object_pb.has_action_function_key_click() {
            let action_function_key_click_pb =
                object_pb.take_action_function_key_click();

            ActionFunctionKeyClick::from_pb(action_function_key_click_pb)?
                .into()
        } else if object_pb.has_action_control_key_click() {
            let action_control_key_click_pb =
                object_pb.take_action_control_key_click();

            ActionControlKeyClick::from_pb(action_control_key_click_pb)?.into()
        } else if object_pb.has_action_kp_key_click() {
            let action_kp_key_click_pb = object_pb.take_action_kp_key_click();

            ActionKPKeyClick::from_pb(action_kp_key_click_pb)?.into()
        } else if object_pb.has_action_multimedia_key_click() {
            let action_multimedia_key_click_pb =
                object_pb.take_action_multimedia_key_click();

            ActionMultimediaKeyClick::from_pb(action_multimedia_key_click_pb)?
                .into()
        } else if object_pb.has_action_mouse_button_key_click() {
            let action_mouse_button_key_click_pb =
                object_pb.take_action_mouse_button_key_click();

            ActionMouseButtonKeyClick::from_pb(
                action_mouse_button_key_click_pb,
            )?
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
        } else if object_pb.has_action_wait() {
            let action_wait_pb = object_pb.take_action_wait();

            ActionWait::from_pb(action_wait_pb)?.into()
        } else if object_pb.has_action_text_type() {
            let action_text_type_pb = object_pb.take_action_text_type();

            ActionTextType::from_pb(action_text_type_pb)?.into()
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
        } else if object_pb.has_action_execute_named_action() {
            let action_execute_named_action_pb =
                object_pb.take_action_execute_named_action();

            ActionExecuteNamedAction::from_pb(action_execute_named_action_pb)?
                .into()
        } else {
            return NiaServerError::deserialization_error(
                "Invalid action type.",
            )
            .into();
        };

        let action = NiaAction {
            action: action_enum,
        };

        Ok(action)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(test)]
    mod serialization {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn serializes_and_deserializes_action_key_click() {
            let action = ActionKeyClick::new(1).into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_key_press() {
            let action = ActionKeyPress::new(1).into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_key_release() {
            let action = ActionKeyRelease::new(1).into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let mut actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_mouse_button_click() {
            let action = ActionMouseButtonClick::new(1).into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_mouse_button_press() {
            let action = ActionMouseButtonPress::new(1).into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_mouse_button_release() {
            let action = ActionMouseButtonRelease::new(1).into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_text_key_click() {
            let action = ActionTextKeyClick::new(1).into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }
        #[test]
        fn serializes_and_deserializes_action_number_key_click() {
            let action = ActionNumberKeyClick::new(1).into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }
        #[test]
        fn serializes_and_deserializes_action_function_key_click() {
            let action = ActionFunctionKeyClick::new(1).into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }
        #[test]
        fn serializes_and_deserializes_action_control_key_click() {
            let action = ActionControlKeyClick::new(1).into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }
        #[test]
        fn serializes_and_deserializes_action_kp_key_click() {
            let action = ActionKPKeyClick::new(1).into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }
        #[test]
        fn serializes_and_deserializes_action_multimedia_key_click() {
            let action = ActionMultimediaKeyClick::new(1).into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }
        #[test]
        fn serializes_and_deserializes_action_mouse_button_key_click() {
            let action = ActionMouseButtonKeyClick::new(1).into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_mouse_absolute_move() {
            let action = ActionMouseAbsoluteMove::new(100, 100).into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_mouse_relative_move() {
            let action = ActionMouseRelativeMove::new(100, 100).into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_wait() {
            let action = ActionWait::new(1000).into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }
        #[test]
        fn serializes_and_deserializes_action_text_type() {
            let action = ActionTextType::new("arst").into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_execute_code() {
            let action = ActionExecuteCode::new("(+ 1 2)").into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_execute_function() {
            let action = ActionExecuteFunction::new("function").into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_execute_os_command() {
            let action = ActionExecuteOSCommand::new("ls").into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_execute_named_action() {
            let action = ActionExecuteNamedAction::new("print-nya").into();

            let expected = NiaAction { action };

            let bytes = expected.to_bytes().unwrap();
            let actual = NiaAction::from_bytes(bytes).unwrap();

            assert_eq!(expected, actual);
        }
    }

    #[cfg(test)]
    mod convertable {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn serializes_and_deserializes_action_key_click() {
            let action = ActionKeyClick::new(1).into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_key_press() {
            let action = ActionKeyPress::new(1).into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_key_release() {
            let action = ActionKeyRelease::new(1).into();

            let expected = NiaAction { action };

            let bytes = expected.to_interpreter_repr();
            let mut actual = NiaAction::from_interpreter_repr(&bytes).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_mouse_button_click() {
            let action = ActionMouseButtonClick::new(1).into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_mouse_button_press() {
            let action = ActionMouseButtonPress::new(1).into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_mouse_button_release() {
            let action = ActionMouseButtonRelease::new(1).into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_text_key_click() {
            let action = ActionTextKeyClick::new(1).into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }
        #[test]
        fn serializes_and_deserializes_action_number_key_click() {
            let action = ActionNumberKeyClick::new(1).into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }
        #[test]
        fn serializes_and_deserializes_action_function_key_click() {
            let action = ActionFunctionKeyClick::new(1).into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }
        #[test]
        fn serializes_and_deserializes_action_control_key_click() {
            let action = ActionControlKeyClick::new(1).into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }
        #[test]
        fn serializes_and_deserializes_action_kp_key_click() {
            let action = ActionKPKeyClick::new(1).into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }
        #[test]
        fn serializes_and_deserializes_action_multimedia_key_click() {
            let action = ActionMultimediaKeyClick::new(1).into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }
        #[test]
        fn serializes_and_deserializes_action_mouse_button_key_click() {
            let action = ActionMouseButtonKeyClick::new(1).into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_mouse_absolute_move() {
            let action = ActionMouseAbsoluteMove::new(100, 100).into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_mouse_relative_move() {
            let action = ActionMouseRelativeMove::new(100, 100).into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_wait() {
            let action = ActionWait::new(1000).into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }
        #[test]
        fn serializes_and_deserializes_action_text_type() {
            let action = ActionTextType::new("arst").into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }
        #[test]
        fn serializes_and_deserializes_action_execute_code() {
            let action = ActionExecuteCode::new("(+ 1 2)").into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_execute_function() {
            let action = ActionExecuteFunction::new("function").into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_execute_os_command() {
            let action = ActionExecuteOSCommand::new("ls").into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }

        #[test]
        fn serializes_and_deserializes_action_execute_named_action() {
            let action =
                ActionExecuteNamedAction::new(String::from("print-nya")).into();

            let expected = NiaAction { action };

            let interpreter_action = expected.to_interpreter_repr();
            let actual =
                NiaAction::from_interpreter_repr(&interpreter_action).unwrap();

            assert_eq!(expected, actual);
        }
    }
}
