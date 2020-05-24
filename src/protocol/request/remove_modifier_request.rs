use nia_protocol_rust::RemoveModifierRequest;

use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::NiaKey;
use crate::protocol::Serializable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaRemoveModifierRequest {
    key: NiaKey,
}

impl NiaRemoveModifierRequest {
    pub fn new(key: NiaKey) -> NiaRemoveModifierRequest {
        NiaRemoveModifierRequest { key }
    }

    pub fn take_key(&self) -> NiaKey {
        self.key
    }
}

impl
    Serializable<
        NiaRemoveModifierRequest,
        nia_protocol_rust::RemoveModifierRequest,
    > for NiaRemoveModifierRequest
{
    fn to_pb(&self) -> nia_protocol_rust::RemoveModifierRequest {
        let modifier_key_pb = self.key.to_pb();
        let mut remove_modifier_request_pb =
            nia_protocol_rust::RemoveModifierRequest::new();

        remove_modifier_request_pb.set_modifier_key(modifier_key_pb);

        remove_modifier_request_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::RemoveModifierRequest,
    ) -> NiaServerResult<NiaRemoveModifierRequest> {
        let mut object_pb = object_pb;

        let key = NiaKey::from_pb(object_pb.take_modifier_key())?;

        let remove_modifier_request = NiaRemoveModifierRequest::new(key);

        Ok(remove_modifier_request)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes_key_1() {
        let expected = NiaRemoveModifierRequest::new(NiaKey::Key1(1));

        let bytes = expected.to_bytes().unwrap();
        let result = NiaRemoveModifierRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn serializes_and_deserializes_key_2() {
        let expected = NiaRemoveModifierRequest::new(NiaKey::Key2(1, 2));

        let bytes = expected.to_bytes().unwrap();
        let result = NiaRemoveModifierRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
