use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::NiaKeyChord;
use crate::protocol::NiaMapping;
use crate::protocol::Serializable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaRemoveMappingRequest {
    key_chords: Vec<NiaKeyChord>,
}

impl NiaRemoveMappingRequest {
    pub fn new(key_chords: Vec<NiaKeyChord>) -> NiaRemoveMappingRequest {
        NiaRemoveMappingRequest { key_chords }
    }

    pub fn take_key_chords(self) -> Vec<NiaKeyChord> {
        self.key_chords
    }
}

impl
    Serializable<
        NiaRemoveMappingRequest,
        nia_protocol_rust::RemoveMappingRequest,
    > for NiaRemoveMappingRequest
{
    fn to_pb(&self) -> nia_protocol_rust::RemoveMappingRequest {
        let mut key_chords_pb_vector = Vec::new();

        for key_chord in &self.key_chords {
            key_chords_pb_vector.push(key_chord.to_pb());
        }

        let mut remove_mapping_request_pb =
            nia_protocol_rust::RemoveMappingRequest::new();

        remove_mapping_request_pb.set_key_chords(
            protobuf::RepeatedField::from_vec(key_chords_pb_vector),
        );

        remove_mapping_request_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::RemoveMappingRequest,
    ) -> NiaServerResult<NiaRemoveMappingRequest> {
        let mut object_pb = object_pb;
        let mut key_chords = Vec::new();

        for key_chord_pb in object_pb.take_key_chords().into_iter() {
            let key_chord = NiaKeyChord::from_pb(key_chord_pb)?;

            key_chords.push(key_chord);
        }

        let request = NiaRemoveMappingRequest::new(key_chords);

        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::protocol::NiaKey;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaRemoveMappingRequest::new(vec![
            NiaKeyChord::new(
                vec![
                    NiaKey::make_key_2(1, 2),
                    NiaKey::make_key_2(1, 3),
                    NiaKey::make_key_2(1, 4),
                ],
                NiaKey::make_key_2(1, 5),
            ),
            NiaKeyChord::new(
                vec![
                    NiaKey::make_key_2(2, 2),
                    NiaKey::make_key_2(2, 3),
                    NiaKey::make_key_2(2, 4),
                ],
                NiaKey::make_key_2(2, 5),
            ),
        ]);

        let bytes = expected.to_bytes().unwrap();
        let result = NiaRemoveMappingRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
