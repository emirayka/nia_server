use crate::error::{from_protobuf_error, NiaServerError};
use crate::protocol::Serializable;
use protobuf::Message;

#[derive(Clone, Copy, Debug)]
pub struct ActionWait {
    ms: i32,
}

impl ActionWait {
    pub fn new(ms: i32) -> ActionWait {
        ActionWait { ms }
    }

    pub fn get_ms(&self) -> i32 {
        self.ms
    }
}

impl Serializable<ActionWait, nia_protocol_rust::ActionWait> for ActionWait {
    fn to_pb(&self) -> nia_protocol_rust::ActionWait {
        let mut action_wait_pb = nia_protocol_rust::ActionWait::new();

        action_wait_pb.set_ms(self.ms);

        action_wait_pb
    }

    fn from_pb(object_pb: nia_protocol_rust::ActionWait) -> ActionWait {
        let action_wait = ActionWait::new(object_pb.get_ms());

        action_wait
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializable_and_deserializable() {
        let expected = 1000;

        let action_wait = ActionWait::new(expected);
        let bytes = action_wait.to_bytes().unwrap();
        let action_wait = ActionWait::from_bytes(bytes).unwrap();

        let result = action_wait.ms;

        assert_eq!(expected, result)
    }
}
