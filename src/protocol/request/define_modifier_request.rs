use nia_protocol_rust::DefineModifierRequest;
use nia_protocol_rust::ModifierDescription;

use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::NiaModifierDescription;
use crate::protocol::Serializable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaDefineModifierRequest {
    modifier: NiaModifierDescription,
}

impl NiaDefineModifierRequest {
    pub fn new(modifier: NiaModifierDescription) -> NiaDefineModifierRequest {
        NiaDefineModifierRequest { modifier }
    }

    pub fn take_modifier(self) -> NiaModifierDescription {
        self.modifier
    }
}

impl
    Serializable<
        NiaDefineModifierRequest,
        nia_protocol_rust::DefineModifierRequest,
    > for NiaDefineModifierRequest
{
    fn to_pb(&self) -> nia_protocol_rust::DefineModifierRequest {
        let modifier_pb = self.modifier.to_pb();

        let mut define_modifier_request_pb =
            nia_protocol_rust::DefineModifierRequest::new();

        define_modifier_request_pb.set_modifier(modifier_pb);

        define_modifier_request_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::DefineModifierRequest,
    ) -> NiaServerResult<NiaDefineModifierRequest> {
        let mut object_pb = object_pb;
        let modifier_pb = object_pb.take_modifier();

        let modifier = NiaModifierDescription::from_pb(modifier_pb)?;

        let mut define_modifier_request =
            NiaDefineModifierRequest::new(modifier);

        Ok(define_modifier_request)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::protocol::NiaKey;

    #[test]
    fn serializes_and_deserializes() {
        let key = NiaKey::Key2(2, 3);
        let alias = String::from("test");

        let expected = NiaDefineModifierRequest::new(
            NiaModifierDescription::new(key, alias),
        );

        let bytes = expected.to_bytes().unwrap();
        let result = NiaDefineModifierRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
