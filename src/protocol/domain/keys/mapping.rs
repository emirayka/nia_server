use crate::error::NiaServerResult;

use crate::protocol::NiaKeyChord;
use crate::protocol::Serializable;
use crate::protocol::{NiaAction, NiaConvertable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NiaMapping {
    key_chords: Vec<NiaKeyChord>,
    action: NiaAction,
}

impl NiaMapping {
    pub fn new(key_chords: Vec<NiaKeyChord>, action: NiaAction) -> NiaMapping {
        NiaMapping { key_chords, action }
    }

    pub fn get_key_chords(&self) -> &Vec<NiaKeyChord> {
        &self.key_chords
    }

    pub fn get_action(&self) -> &NiaAction {
        &self.action
    }
}

impl NiaConvertable<NiaMapping, nia_interpreter_core::Mapping> for NiaMapping {
    fn to_interpreter_repr(&self) -> nia_interpreter_core::Mapping {
        let interpreter_action = self.action.to_interpreter_repr();
        let mut interpreter_key_chords = Vec::new();

        for key_chord in &self.key_chords {
            let interpreter_key_chord = key_chord.to_interpreter_repr();

            interpreter_key_chords.push(interpreter_key_chord)
        }

        nia_interpreter_core::Mapping::new(
            interpreter_key_chords,
            interpreter_action,
        )
    }

    fn from_interpreter_repr(
        object_pb: &nia_interpreter_core::Mapping,
    ) -> NiaServerResult<NiaMapping> {
        let action = NiaAction::from_interpreter_repr(
            object_pb.get_action(), // todo: fix, looks ugly
        )?;

        let mut key_chords = object_pb
            .get_key_chords()
            .iter()
            .map(|interpreter_key_chord| {
                NiaKeyChord::from_interpreter_repr(interpreter_key_chord)
            })
            .collect::<NiaServerResult<Vec<NiaKeyChord>>>()?;

        let mapping = (NiaMapping::new(key_chords, action));

        Ok(mapping)
    }
}

impl Serializable<NiaMapping, nia_protocol_rust::Mapping> for NiaMapping {
    fn to_pb(&self) -> nia_protocol_rust::Mapping {
        let mut key_chords_pb = Vec::new();

        for key_chord in &self.key_chords {
            let key_chord_pb = key_chord.to_pb();

            key_chords_pb.push(key_chord_pb);
        }

        let action_pb = self.action.to_pb();

        let mut mapping_pb = nia_protocol_rust::Mapping::new();

        mapping_pb
            .set_key_chords(protobuf::RepeatedField::from_vec(key_chords_pb));
        mapping_pb.set_action(action_pb);

        mapping_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::Mapping,
    ) -> NiaServerResult<NiaMapping> {
        let mut object_pb = object_pb;

        let mut key_chords = Vec::new();

        for key_chord_pb in object_pb.take_key_chords().into_iter() {
            let key_chord = NiaKeyChord::from_pb(key_chord_pb)?;

            key_chords.push(key_chord);
        }

        let action = NiaAction::from_pb(object_pb.take_action())?;

        let mapping = NiaMapping::new(key_chords, action);

        Ok(mapping)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::protocol::{ActionExecuteOSCommand, NiaActionEnum, NiaKey};

    fn construct_mapping() -> NiaMapping {
        NiaMapping::new(
            vec![
                NiaKeyChord::new(
                    vec![NiaKey::make_key_2(1, 1), NiaKey::make_key_2(1, 2)],
                    NiaKey::make_key_2(1, 3),
                ),
                NiaKeyChord::new(
                    vec![NiaKey::make_key_2(2, 1), NiaKey::make_key_2(2, 2)],
                    NiaKey::make_key_2(2, 3),
                ),
            ],
            NiaAction::new(NiaActionEnum::ExecuteOSCommand(
                ActionExecuteOSCommand::new("echo catgirl"),
            )),
        )
    }

    #[cfg(test)]
    mod serialization {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn serializes_and_deserializes() {
            let expected = construct_mapping();

            let bytes = expected.to_bytes().unwrap();
            let result = NiaMapping::from_bytes(bytes).unwrap();

            assert_eq!(expected, result);
        }
    }

    #[cfg(test)]
    mod convertation {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn convertable_between_interpreter_and_server_representations() {
            let mut expected = construct_mapping(); // todo: fails when name is not empty

            let interpreter_mapping = expected.to_interpreter_repr();
            let result =
                NiaMapping::from_interpreter_repr(&interpreter_mapping)
                    .unwrap();

            assert_eq!(expected, result);
        }
    }
}
