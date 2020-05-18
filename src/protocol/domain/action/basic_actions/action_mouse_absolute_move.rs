use crate::error::{from_protobuf_error, NiaServerError, NiaServerResult};
use crate::protocol::Serializable;
use protobuf::Message;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionMouseAbsoluteMove {
    x: i32,
    y: i32,
}

impl ActionMouseAbsoluteMove {
    pub fn new(x: i32, y: i32) -> ActionMouseAbsoluteMove {
        ActionMouseAbsoluteMove { x, y }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }
}

impl
    Serializable<
        ActionMouseAbsoluteMove,
        nia_protocol_rust::ActionMouseAbsoluteMove,
    > for ActionMouseAbsoluteMove
{
    fn to_pb(&self) -> nia_protocol_rust::ActionMouseAbsoluteMove {
        let mut action_mouse_absolute_move_pb =
            nia_protocol_rust::ActionMouseAbsoluteMove::new();

        action_mouse_absolute_move_pb.set_x(self.x);
        action_mouse_absolute_move_pb.set_y(self.y);

        action_mouse_absolute_move_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ActionMouseAbsoluteMove,
    ) -> NiaServerResult<ActionMouseAbsoluteMove> {
        let action_mouse_absolute_move =
            ActionMouseAbsoluteMove::new(object_pb.get_x(), object_pb.get_y());

        Ok(action_mouse_absolute_move)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializable_and_deserializable() {
        let x_expected = 100;
        let y_expected = 200;

        let action_mouse_absolute_move =
            ActionMouseAbsoluteMove::new(x_expected, y_expected);
        let bytes = action_mouse_absolute_move.to_bytes().unwrap();
        let action_mouse_absolute_move =
            ActionMouseAbsoluteMove::from_bytes(bytes).unwrap();

        let x_actual = action_mouse_absolute_move.x;
        let y_actual = action_mouse_absolute_move.y;

        assert_eq!(x_expected, x_actual);
        assert_eq!(y_expected, y_actual);
    }
}
