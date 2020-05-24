use crate::error::NiaServerResult;
use crate::protocol::{NiaConvertable, NiaKey, Serializable};

#[derive(Clone, Debug, Eq)]
pub struct NiaKeyChord {
    modifiers: Vec<NiaKey>,
    ordinary_key: NiaKey,
}

impl PartialEq for NiaKeyChord {
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

impl NiaKeyChord {
    pub fn new(modifiers: Vec<NiaKey>, ordinary_key: NiaKey) -> NiaKeyChord {
        NiaKeyChord {
            modifiers,
            ordinary_key,
        }
    }

    pub fn get_modifiers(&self) -> &Vec<NiaKey> {
        &self.modifiers
    }

    pub fn get_key(&self) -> NiaKey {
        self.ordinary_key
    }
}

impl NiaConvertable<NiaKeyChord, nia_interpreter_core::KeyChord>
    for NiaKeyChord
{
    fn to_interpreter_repr(&self) -> nia_interpreter_core::KeyChord {
        let mut interpreter_modifiers = Vec::new();

        for modifier in &self.modifiers {
            let interpreter_modifier = modifier.to_interpreter_repr();
            interpreter_modifiers.push(interpreter_modifier);
        }

        let interpreter_key = self.ordinary_key.to_interpreter_repr();

        nia_interpreter_core::KeyChord::new(
            interpreter_modifiers,
            interpreter_key,
        )
    }

    fn from_interpreter_repr(
        key_chord: &nia_interpreter_core::KeyChord,
    ) -> NiaServerResult<NiaKeyChord> {
        let mut modifiers = Vec::new();

        for interpreter_modifier in key_chord.get_modifiers() {
            let modifier = NiaKey::from_interpreter_repr(interpreter_modifier)?;

            modifiers.push(modifier);
        }

        let ordinary_key = NiaKey::from_interpreter_repr(&key_chord.get_key())?;

        let key_chord = NiaKeyChord::new(modifiers, ordinary_key);

        Ok(key_chord)
    }
}

impl Serializable<NiaKeyChord, nia_protocol_rust::KeyChord> for NiaKeyChord {
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
    ) -> NiaServerResult<NiaKeyChord> {
        let mut object_pb = object_pb;

        let ordinary_key = NiaKey::from_pb(object_pb.take_ordinary_key())?;
        let mut modifiers = Vec::new();

        for modifier in object_pb.take_modifiers().into_iter() {
            let modifier = NiaKey::from_pb(modifier)?;
            modifiers.push(modifier);
        }

        let key_chord = NiaKeyChord::new(modifiers, ordinary_key);

        Ok(key_chord)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(test)]
    mod serialization {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn serializes_and_deserializes() {
            let expected_key_chord = NiaKeyChord::new(
                vec![NiaKey::make_key_2(1, 2), NiaKey::make_key_1(1)],
                NiaKey::make_key_2(2, 3),
            );

            let bytes = expected_key_chord.to_bytes().unwrap();
            let key_chord = NiaKeyChord::from_bytes(bytes).unwrap();

            assert_eq!(expected_key_chord, key_chord)
        }
    }

    #[cfg(test)]
    mod convertation {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn convertable_between_server_and_interpreter_representations() {
            let expected_key_chord = NiaKeyChord::new(
                vec![NiaKey::make_key_2(1, 2), NiaKey::make_key_1(1)],
                NiaKey::make_key_2(2, 3),
            );

            let interpreter_key_chord =
                expected_key_chord.to_interpreter_repr();
            let key_chord =
                NiaKeyChord::from_interpreter_repr(&interpreter_key_chord)
                    .unwrap();

            assert_eq!(expected_key_chord, key_chord)
        }
    }
}
