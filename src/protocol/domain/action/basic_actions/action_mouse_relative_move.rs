use crate::error::{from_protobuf_error, NiaServerError, NiaServerResult};
use crate::protocol::Serializable;
use protobuf::Message;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionMouseRelativeMove {
    dx: i32,
    dy: i32,
}

impl ActionMouseRelativeMove {
    pub fn new(dx: i32, dy: i32) -> ActionMouseRelativeMove {
        ActionMouseRelativeMove { dx, dy }
    }

    pub fn get_dx(&self) -> i32 {
        self.dx
    }

    pub fn get_dy(&self) -> i32 {
        self.dy
    }
}

impl
    Serializable<
        ActionMouseRelativeMove,
        nia_protocol_rust::ActionMouseRelativeMove,
    > for ActionMouseRelativeMove
{
    fn to_pb(&self) -> nia_protocol_rust::ActionMouseRelativeMove {
        let mut action_mouse_absolute_move_pb =
            nia_protocol_rust::ActionMouseRelativeMove::new();

        action_mouse_absolute_move_pb.set_dx(self.dx);
        action_mouse_absolute_move_pb.set_dy(self.dy);

        action_mouse_absolute_move_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ActionMouseRelativeMove,
    ) -> NiaServerResult<ActionMouseRelativeMove> {
        let action_mouse_absolute_move = ActionMouseRelativeMove::new(
            object_pb.get_dx(),
            object_pb.get_dy(),
        );

        Ok(action_mouse_absolute_move)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializable_and_deserializable() {
        let dx_expected = 100;
        let dy_expected = 200;

        let action_mouse_relative_move =
            ActionMouseRelativeMove::new(dx_expected, dy_expected);
        let bytes = action_mouse_relative_move.to_bytes().unwrap();
        let action_mouse_relative_move =
            ActionMouseRelativeMove::from_bytes(bytes).unwrap();

        let dx_actual = action_mouse_relative_move.dx;
        let dy_actual = action_mouse_relative_move.dy;

        assert_eq!(dx_expected, dx_actual);
        assert_eq!(dy_expected, dy_actual);
    }
}
