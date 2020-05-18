use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::RequestType;
use crate::protocol::{GetRequestType, Serializable};
use nia_protocol_rust::RemoveModifierRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaRemoveModifierRequest {
    device_id: i32,
    key_code: i32,
}

impl NiaRemoveModifierRequest {
    pub fn new(device_id: i32, key_code: i32) -> NiaRemoveModifierRequest {
        NiaRemoveModifierRequest {
            device_id,
            key_code,
        }
    }

    pub fn get_device_id_and_key_code(self) -> (i32, i32) {
        (self.device_id, self.key_code)
    }
}

impl
    Serializable<
        NiaRemoveModifierRequest,
        nia_protocol_rust::RemoveModifierRequest,
    > for NiaRemoveModifierRequest
{
    fn to_pb(&self) -> nia_protocol_rust::RemoveModifierRequest {
        let mut remove_modifier_request_pb =
            nia_protocol_rust::RemoveModifierRequest::new();

        remove_modifier_request_pb.set_device_id(self.device_id);
        remove_modifier_request_pb.set_key_code(self.key_code);

        remove_modifier_request_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::RemoveModifierRequest,
    ) -> NiaServerResult<NiaRemoveModifierRequest> {
        let remove_modifier_request = NiaRemoveModifierRequest::new(
            object_pb.get_device_id(),
            object_pb.get_key_code(),
        );

        Ok(remove_modifier_request)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaRemoveModifierRequest::new(0, 12);

        let bytes = expected.to_bytes().unwrap();
        let result = NiaRemoveModifierRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
