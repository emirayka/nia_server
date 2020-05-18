use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::RequestType;
use crate::protocol::{GetRequestType, Serializable};
use nia_protocol_rust::DefineModifierRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaDefineModifierRequest {
    device_id: i32,
    key_code: i32,
    modifier_alias: String,
}

impl NiaDefineModifierRequest {
    pub fn new<S>(
        device_id: i32,
        key_code: i32,
        modifier_alias: S,
    ) -> NiaDefineModifierRequest
    where
        S: Into<String>,
    {
        NiaDefineModifierRequest {
            device_id,
            key_code,
            modifier_alias: modifier_alias.into(),
        }
    }

    pub fn into_tuple(self) -> (i32, i32, String) {
        (self.device_id, self.key_code, self.modifier_alias)
    }
}

impl
    Serializable<
        NiaDefineModifierRequest,
        nia_protocol_rust::DefineModifierRequest,
    > for NiaDefineModifierRequest
{
    fn to_pb(&self) -> nia_protocol_rust::DefineModifierRequest {
        let mut define_modifier_request_pb =
            nia_protocol_rust::DefineModifierRequest::new();

        define_modifier_request_pb.set_device_id(self.device_id);
        define_modifier_request_pb.set_key_code(self.key_code);
        define_modifier_request_pb.set_modifier_alias(protobuf::Chars::from(
            self.modifier_alias.clone(),
        ));

        define_modifier_request_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::DefineModifierRequest,
    ) -> NiaServerResult<NiaDefineModifierRequest> {
        let device_id = object_pb.get_device_id();
        let key_code = object_pb.get_key_code();
        let modifier_alias = object_pb.get_modifier_alias().to_string();

        let mut define_modifier_request =
            NiaDefineModifierRequest::new(device_id, key_code, modifier_alias);

        Ok(define_modifier_request)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaDefineModifierRequest::new(2, 3, "kek");

        let bytes = expected.to_bytes().unwrap();
        let result = NiaDefineModifierRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
