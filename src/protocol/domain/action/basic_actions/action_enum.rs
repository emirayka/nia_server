use crate::protocol::domain::action::basic_actions::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NiaActionEnum {
    KeyClick(ActionKeyClick),
    KeyPress(ActionKeyPress),
    KeyRelease(ActionKeyRelease),

    MouseButtonClick(ActionMouseButtonClick),
    MouseButtonPress(ActionMouseButtonPress),
    MouseButtonRelease(ActionMouseButtonRelease),

    ControlKeyClick(ActionControlKeyClick),
    FunctionKeyClick(ActionFunctionKeyClick),
    KPKeyClick(ActionKPKeyClick),
    MouseButtonKeyClick(ActionMouseButtonKeyClick),
    MultimediaKeyClick(ActionMultimediaKeyClick),
    NumberKeyClick(ActionNumberKeyClick),
    TextKeyClick(ActionTextKeyClick),

    MouseAbsoluteMove(ActionMouseAbsoluteMove),
    MouseRelativeMove(ActionMouseRelativeMove),

    Wait(ActionWait),
    TextType(ActionTextType),

    ExecuteCode(ActionExecuteCode),
    ExecuteFunction(ActionExecuteFunction),
    ExecuteOSCommand(ActionExecuteOSCommand),
    ExecuteNamedAction(ActionExecuteNamedAction),
}

macro_rules! make_from_impl {
    ($underlying_type:ident, $variant:path) => {
        impl From<$underlying_type> for NiaActionEnum {
            fn from(action: $underlying_type) -> Self {
                $variant(action)
            }
        }
    };
}

make_from_impl!(ActionKeyClick, NiaActionEnum::KeyClick);
make_from_impl!(ActionKeyPress, NiaActionEnum::KeyPress);
make_from_impl!(ActionKeyRelease, NiaActionEnum::KeyRelease);

make_from_impl!(ActionMouseButtonClick, NiaActionEnum::MouseButtonClick);
make_from_impl!(ActionMouseButtonPress, NiaActionEnum::MouseButtonPress);
make_from_impl!(ActionMouseButtonRelease, NiaActionEnum::MouseButtonRelease);

make_from_impl!(ActionControlKeyClick, NiaActionEnum::ControlKeyClick);
make_from_impl!(ActionFunctionKeyClick, NiaActionEnum::FunctionKeyClick);
make_from_impl!(ActionKPKeyClick, NiaActionEnum::KPKeyClick);
#[rustfmt::skip]
make_from_impl!(ActionMouseButtonKeyClick, NiaActionEnum::MouseButtonKeyClick);
make_from_impl!(ActionMultimediaKeyClick, NiaActionEnum::MultimediaKeyClick);
make_from_impl!(ActionNumberKeyClick, NiaActionEnum::NumberKeyClick);
make_from_impl!(ActionTextKeyClick, NiaActionEnum::TextKeyClick);

make_from_impl!(ActionMouseRelativeMove, NiaActionEnum::MouseRelativeMove);
make_from_impl!(ActionMouseAbsoluteMove, NiaActionEnum::MouseAbsoluteMove);

make_from_impl!(ActionWait, NiaActionEnum::Wait);
make_from_impl!(ActionTextType, NiaActionEnum::TextType);

make_from_impl!(ActionExecuteCode, NiaActionEnum::ExecuteCode);
make_from_impl!(ActionExecuteFunction, NiaActionEnum::ExecuteFunction);
make_from_impl!(ActionExecuteOSCommand, NiaActionEnum::ExecuteOSCommand);
make_from_impl!(ActionExecuteNamedAction, NiaActionEnum::ExecuteNamedAction);
