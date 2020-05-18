use crate::protocol::domain::action::basic_actions::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionEnum {
    KeyClick(ActionKeyClick),
    KeyPress(ActionKeyPress),
    KeyRelease(ActionKeyRelease),

    MouseButtonClick(ActionMouseButtonClick),
    MouseButtonPress(ActionMouseButtonPress),
    MouseButtonRelease(ActionMouseButtonRelease),

    MouseAbsoluteMove(ActionMouseAbsoluteMove),
    MouseRelativeMove(ActionMouseRelativeMove),

    ExecuteCode(ActionExecuteCode),
    ExecuteFunction(ActionExecuteFunction),
    ExecuteOSCommand(ActionExecuteOSCommand),
    TextType(ActionTextType),
    Wait(ActionWait),
}

macro_rules! make_from_impl {
    ($underlying_type:ident, $variant:path) => {
        impl From<$underlying_type> for ActionEnum {
            fn from(action: $underlying_type) -> Self {
                $variant(action)
            }
        }
    };
}

make_from_impl!(ActionKeyClick, ActionEnum::KeyClick);
make_from_impl!(ActionKeyPress, ActionEnum::KeyPress);
make_from_impl!(ActionKeyRelease, ActionEnum::KeyRelease);

make_from_impl!(ActionMouseButtonClick, ActionEnum::MouseButtonClick);
make_from_impl!(ActionMouseButtonPress, ActionEnum::MouseButtonPress);
make_from_impl!(ActionMouseButtonRelease, ActionEnum::MouseButtonRelease);

make_from_impl!(ActionMouseRelativeMove, ActionEnum::MouseRelativeMove);
make_from_impl!(ActionMouseAbsoluteMove, ActionEnum::MouseAbsoluteMove);

make_from_impl!(ActionExecuteCode, ActionEnum::ExecuteCode);
make_from_impl!(ActionExecuteFunction, ActionEnum::ExecuteFunction);
make_from_impl!(ActionExecuteOSCommand, ActionEnum::ExecuteOSCommand);
make_from_impl!(ActionTextType, ActionEnum::TextType);
make_from_impl!(ActionWait, ActionEnum::Wait);
