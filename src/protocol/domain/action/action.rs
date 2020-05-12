use crate::protocol::domain::action::basic_actions::*;
use crate::protocol::Serializable;

#[derive(Clone, Debug)]
pub enum Action {
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

impl Serializable<Action, nia_protocol_rust::Action> for Action {
    fn to_pb(&self) -> nia_protocol_rust::Action {
        unimplemented!()
    }

    fn from_pb(object_pb: nia_protocol_rust::Action) -> Action {
        unimplemented!()
    }
}

macro_rules! make_from_impl {
    ($underlying_type:ident, $variant:path) => {
        impl From<$underlying_type> for Action {
            fn from(action: $underlying_type) -> Self {
                $variant(action)
            }
        }
    };
}

make_from_impl!(ActionKeyClick, Action::KeyClick);
make_from_impl!(ActionKeyPress, Action::KeyPress);
make_from_impl!(ActionKeyRelease, Action::KeyRelease);

make_from_impl!(ActionMouseButtonClick, Action::MouseButtonClick);
make_from_impl!(ActionMouseButtonPress, Action::MouseButtonPress);
make_from_impl!(ActionMouseButtonRelease, Action::MouseButtonRelease);

make_from_impl!(ActionMouseRelativeMove, Action::MouseRelativeMove);
make_from_impl!(ActionMouseAbsoluteMove, Action::MouseAbsoluteMove);

make_from_impl!(ActionExecuteCode, Action::ExecuteCode);
make_from_impl!(ActionExecuteFunction, Action::ExecuteFunction);
make_from_impl!(ActionExecuteOSCommand, Action::ExecuteOSCommand);
make_from_impl!(ActionTextType, Action::TextType);
make_from_impl!(ActionWait, Action::Wait);
