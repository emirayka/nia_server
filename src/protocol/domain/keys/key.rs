use crate::error::{NiaServerError, NiaServerResult};
use crate::protocol::Serializable;
use nia_protocol_rust::Key_oneof_key::key_2;
use std::fs::read;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Key1(i32),
    Key2(i32, i32),
}

impl Key {
    pub fn make_key_1(key_code: i32) -> Key {
        Key::Key1(key_code)
    }

    pub fn make_key_2(device_id: i32, key_code: i32) -> Key {
        Key::Key2(device_id, key_code)
    }

    pub fn get_device_id(&self) -> Option<i32> {
        match self {
            Key::Key1(_) => None,
            Key::Key2(device_id, _) => Some(*device_id),
        }
    }

    pub fn get_key_code(&self) -> i32 {
        match self {
            Key::Key1(key_code) => *key_code,
            Key::Key2(_, key_code) => *key_code,
        }
    }
}

impl Serializable<Key, nia_protocol_rust::Key> for Key {
    fn to_pb(&self) -> nia_protocol_rust::Key {
        let mut key_pb = nia_protocol_rust::Key::new();

        match self {
            Key::Key1(key_code) => {
                let mut key_1_pb = nia_protocol_rust::Key1::new();

                key_1_pb.set_key_code(*key_code);

                key_pb.set_key_1(key_1_pb);
            }
            Key::Key2(device_id, key_code) => {
                let mut key_2_pb = nia_protocol_rust::Key2::new();

                key_2_pb.set_device_id(*device_id);
                key_2_pb.set_key_code(*key_code);

                key_pb.set_key_2(key_2_pb);
            }
        }

        key_pb
    }

    fn from_pb(object_pb: nia_protocol_rust::Key) -> NiaServerResult<Key> {
        let mut object_pb = object_pb;

        let key = if object_pb.has_key_1() {
            let key_1_pb = object_pb.take_key_1();

            let key_code = key_1_pb.get_key_code();

            Key::Key1(key_code)
        } else if object_pb.has_key_2() {
            let key_2_pb = object_pb.take_key_2();

            let device_id = key_2_pb.get_device_id();
            let key_code = key_2_pb.get_key_code();

            Key::Key2(device_id, key_code)
        } else {
            return NiaServerError::deserialization_error(
                "Provided neither a key1 nor key2.",
            )
            .into();
        };

        Ok(key)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes_key_2() {
        let device_id_expected = 12;
        let key_code_expected = 123;

        let key = Key::make_key_2(device_id_expected, key_code_expected);
        let bytes = key.to_bytes().unwrap();
        let key = Key::from_bytes(bytes).unwrap();

        let device_id_actual = key.get_device_id().unwrap();
        let key_code_actual = key.get_key_code();

        assert_eq!(device_id_expected, device_id_actual);
        assert_eq!(key_code_expected, key_code_actual);
    }

    #[test]
    fn serializes_and_deserializes_key_1() {
        let key_code_expected = 123;

        let key = Key::make_key_1(key_code_expected);
        let bytes = key.to_bytes().unwrap();
        let key = Key::from_bytes(bytes).unwrap();

        let key_code_actual = key.get_key_code();

        assert_eq!(key_code_expected, key_code_actual);
    }
}
