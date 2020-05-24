use crate::error::{NiaServerError, NiaServerResult};
use crate::protocol::{NiaConvertable, Serializable};
use nia_interpreter_core::Key;
use nia_protocol_rust::Key_oneof_key::key_2;
use std::fs::read;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NiaKey {
    Key1(i32),
    Key2(i32, i32),
}

impl NiaKey {
    pub fn make_key_1(key_code: i32) -> NiaKey {
        NiaKey::Key1(key_code)
    }

    pub fn make_key_2(device_id: i32, key_code: i32) -> NiaKey {
        NiaKey::Key2(device_id, key_code)
    }

    pub fn get_device_id(&self) -> Option<i32> {
        match self {
            NiaKey::Key1(_) => None,
            NiaKey::Key2(device_id, _) => Some(*device_id),
        }
    }

    pub fn get_key_code(&self) -> i32 {
        match self {
            NiaKey::Key1(key_code) => *key_code,
            NiaKey::Key2(_, key_code) => *key_code,
        }
    }
}

impl NiaConvertable<NiaKey, nia_interpreter_core::Key> for NiaKey {
    fn to_interpreter_repr(&self) -> nia_interpreter_core::Key {
        match self {
            NiaKey::Key1(key_id) => {
                nia_interpreter_core::Key::new_lone_key(*key_id)
            }
            NiaKey::Key2(device_id, key_id) => {
                nia_interpreter_core::Key::new_device_key(*device_id, *key_id)
            }
        }
    }

    fn from_interpreter_repr(
        key: &nia_interpreter_core::Key,
    ) -> NiaServerResult<NiaKey> {
        let key = match key {
            Key::DeviceKey(device_key) => NiaKey::Key2(
                device_key.get_device_id(),
                device_key.get_key_id(),
            ),
            Key::LoneKey(lone_key) => NiaKey::Key1(lone_key.get_key_id()),
        };

        Ok(key)
    }
}

impl Serializable<NiaKey, nia_protocol_rust::Key> for NiaKey {
    fn to_pb(&self) -> nia_protocol_rust::Key {
        let mut key_pb = nia_protocol_rust::Key::new();

        match self {
            NiaKey::Key1(key_code) => {
                let mut key_1_pb = nia_protocol_rust::Key1::new();

                key_1_pb.set_key_code(*key_code);

                key_pb.set_key_1(key_1_pb);
            }
            NiaKey::Key2(device_id, key_code) => {
                let mut key_2_pb = nia_protocol_rust::Key2::new();

                key_2_pb.set_device_id(*device_id);
                key_2_pb.set_key_code(*key_code);

                key_pb.set_key_2(key_2_pb);
            }
        }

        key_pb
    }

    fn from_pb(object_pb: nia_protocol_rust::Key) -> NiaServerResult<NiaKey> {
        let mut object_pb = object_pb;

        let key = if object_pb.has_key_1() {
            let key_1_pb = object_pb.take_key_1();

            let key_code = key_1_pb.get_key_code();

            NiaKey::Key1(key_code)
        } else if object_pb.has_key_2() {
            let key_2_pb = object_pb.take_key_2();

            let device_id = key_2_pb.get_device_id();
            let key_code = key_2_pb.get_key_code();

            NiaKey::Key2(device_id, key_code)
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

        let key = NiaKey::make_key_2(device_id_expected, key_code_expected);
        let bytes = key.to_bytes().unwrap();
        let key = NiaKey::from_bytes(bytes).unwrap();

        let device_id_actual = key.get_device_id().unwrap();
        let key_code_actual = key.get_key_code();

        assert_eq!(device_id_expected, device_id_actual);
        assert_eq!(key_code_expected, key_code_actual);
    }

    #[test]
    fn serializes_and_deserializes_key_1() {
        let key_code_expected = 123;

        let key = NiaKey::make_key_1(key_code_expected);
        let bytes = key.to_bytes().unwrap();
        let key = NiaKey::from_bytes(bytes).unwrap();

        let key_code_actual = key.get_key_code();

        assert_eq!(key_code_expected, key_code_actual);
    }

    #[test]
    fn convertable_between_server_and_interpreter_representations_key_1() {
        let key_code_expected = 123;

        let key = NiaKey::Key1(key_code_expected);
        let interpreter_key = key.to_interpreter_repr();
        let result = NiaKey::from_interpreter_repr(&interpreter_key).unwrap();

        assert_eq!(None, result.get_device_id());
        assert_eq!(key_code_expected, result.get_key_code());
    }

    #[test]
    fn convertable_between_server_and_interpreter_representations_key_2() {
        let device_id_expected = 2;
        let key_code_expected = 123;

        let key = NiaKey::Key2(device_id_expected, key_code_expected);
        let interpreter_key = key.to_interpreter_repr();
        let result = NiaKey::from_interpreter_repr(&interpreter_key).unwrap();

        assert_eq!(Some(device_id_expected), result.get_device_id());
        assert_eq!(key_code_expected, result.get_key_code());
    }
}
