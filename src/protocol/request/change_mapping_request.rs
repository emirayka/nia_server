use nia_protocol_rust::ChangeMappingRequest;
use nia_protocol_rust::KeyChord;

use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::NiaAction;
use crate::protocol::NiaKeyChord;
use crate::protocol::NiaMapping;
use crate::protocol::Serializable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaChangeMappingRequest {
    key_chords: Vec<NiaKeyChord>,
    action: NiaAction,
}

impl NiaChangeMappingRequest {
    pub fn new(
        key_chords: Vec<NiaKeyChord>,
        action: NiaAction,
    ) -> NiaChangeMappingRequest {
        NiaChangeMappingRequest { key_chords, action }
    }

    pub fn get_key_chords(&self) -> &Vec<NiaKeyChord> {
        &self.key_chords
    }

    pub fn get_action(&self) -> &NiaAction {
        &self.action
    }

    pub fn into_tuple(self) -> (Vec<NiaKeyChord>, NiaAction) {
        (self.key_chords, self.action)
    }
}

impl
    Serializable<
        NiaChangeMappingRequest,
        nia_protocol_rust::ChangeMappingRequest,
    > for NiaChangeMappingRequest
{
    fn to_pb(&self) -> nia_protocol_rust::ChangeMappingRequest {
        let mut action_pb = self.action.to_pb();
        let mut key_chords_pb = self
            .key_chords
            .iter()
            .map(|key_chord| key_chord.to_pb())
            .collect();

        let mut change_mapping_request_pb =
            nia_protocol_rust::ChangeMappingRequest::new();

        change_mapping_request_pb
            .set_key_chords(protobuf::RepeatedField::from_vec(key_chords_pb));
        change_mapping_request_pb.set_action(action_pb);

        change_mapping_request_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ChangeMappingRequest,
    ) -> NiaServerResult<NiaChangeMappingRequest> {
        let mut object_pb = object_pb;

        let mut key_chords = Vec::new();

        for key_chord_pb in object_pb.take_key_chords().into_iter() {
            let key_chord = NiaKeyChord::from_pb(key_chord_pb)?;

            key_chords.push(key_chord);
        }

        let action = NiaAction::from_pb(object_pb.take_action())?;

        Ok(NiaChangeMappingRequest::new(key_chords, action))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::protocol::{
        ActionMouseRelativeMove, NiaAction, NiaActionEnum, NiaKey, NiaKeyChord,
    };

    #[test]
    fn serializes_and_deserializes() {
        let key_chords = vec![NiaKeyChord::new(
            vec![NiaKey::make_key_2(1, 2), NiaKey::make_key_2(1, 3)],
            NiaKey::make_key_2(1, 4),
        )];

        let action = NiaAction::new(NiaActionEnum::MouseRelativeMove(
            ActionMouseRelativeMove::new(100, 100),
        ));

        let expected = NiaChangeMappingRequest::new(key_chords, action);

        let bytes = expected.to_bytes().unwrap();
        let result = NiaChangeMappingRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
