use crate::error::{from_protobuf_error, NiaServerError};
use crate::protocol::Serializable;
use protobuf::Message;

#[derive(Clone, Copy, Debug)]
pub struct ActionKeyRelease {
    key_code: i32,
}

impl ActionKeyRelease {
    pub fn new(key_code: i32) -> ActionKeyRelease {
        ActionKeyRelease { key_code }
    }

    pub fn get_key_code(&self) -> i32 {
        self.key_code
    }
}

impl Serializable<ActionKeyRelease, nia_protocol_rust::ActionKeyRelease>
    for ActionKeyRelease
{
    fn to_pb(&self) -> nia_protocol_rust::ActionKeyRelease {
        let mut action_key_release_pb =
            nia_protocol_rust::ActionKeyRelease::new();

        action_key_release_pb.set_key_code(self.get_key_code());

        action_key_release_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ActionKeyRelease,
    ) -> ActionKeyRelease {
        let action_key_release =
            ActionKeyRelease::new(object_pb.get_key_code());

        action_key_release
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializable_and_deserializable() {
        let expected = 123;

        let action_key_release = ActionKeyRelease::new(expected);
        let bytes = action_key_release.to_bytes().unwrap();
        let action_key_release = ActionKeyRelease::from_bytes(bytes).unwrap();

        let result = action_key_release.key_code;

        assert_eq!(expected, result)
    }
}
