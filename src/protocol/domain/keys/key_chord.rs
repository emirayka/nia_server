use crate::error::NiaServerResult;
use crate::protocol::{Key, Serializable};

#[derive(Clone, Debug, Eq)]
pub struct KeyChord {
    modifiers: Vec<Key>,
    ordinary_key: Key,
}

impl PartialEq for KeyChord {
    fn eq(&self, other: &Self) -> bool {
        if self.ordinary_key != other.ordinary_key {
            return false;
        }

        if self.modifiers.len() != other.modifiers.len() {
            return false;
        }

        for modifier in &self.modifiers {
            if !other.modifiers.contains(modifier) {
                return false;
            }
        }

        for other_modifier in &other.modifiers {
            if !self.modifiers.contains(other_modifier) {
                return false;
            }
        }

        return true;
    }
}

impl KeyChord {
    pub fn new(modifiers: Vec<Key>, ordinary_key: Key) -> KeyChord {
        KeyChord {
            modifiers,
            ordinary_key,
        }
    }

    pub fn get_modifiers(&self) -> &Vec<Key> {
        &self.modifiers
    }

    pub fn get_key(&self) -> Key {
        self.ordinary_key
    }
}

impl Serializable<KeyChord, nia_protocol_rust::KeyChord> for KeyChord {
    fn to_pb(&self) -> nia_protocol_rust::KeyChord {
        let mut key_chord_pb = nia_protocol_rust::KeyChord::new();

        let ordinary_key_pb = self.ordinary_key.to_pb();
        let modifiers_pb_vector = self
            .modifiers
            .iter()
            .map(|key| key.to_pb())
            .collect::<Vec<nia_protocol_rust::Key>>();

        key_chord_pb
            .set_modifiers(protobuf::RepeatedField::from(modifiers_pb_vector));
        key_chord_pb.set_ordinary_key(ordinary_key_pb);

        key_chord_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::KeyChord,
    ) -> NiaServerResult<KeyChord> {
        let mut object_pb = object_pb;

        let ordinary_key = Key::from_pb(object_pb.take_ordinary_key())?;
        let mut modifiers = Vec::new();

        for modifier in object_pb.take_modifiers().into_iter() {
            let modifier = Key::from_pb(modifier)?;
            modifiers.push(modifier);
        }

        let key_chord = KeyChord::new(modifiers, ordinary_key);

        Ok(key_chord)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected_key_chord = KeyChord::new(
            vec![Key::make_key_2(1, 2), Key::make_key_1(1)],
            Key::make_key_2(2, 3),
        );

        let bytes = expected_key_chord.to_bytes().unwrap();
        let key_chord = KeyChord::from_bytes(bytes).unwrap();

        assert_eq!(expected_key_chord, key_chord)
    }
}
