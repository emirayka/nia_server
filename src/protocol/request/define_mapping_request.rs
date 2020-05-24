use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::NiaMapping;
use crate::protocol::Serializable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaDefineMappingRequest {
    mapping: NiaMapping,
}

impl NiaDefineMappingRequest {
    pub fn new(mapping: NiaMapping) -> NiaDefineMappingRequest {
        NiaDefineMappingRequest { mapping }
    }

    pub fn get_mapping(self) -> NiaMapping {
        self.mapping
    }
}

impl
    Serializable<
        NiaDefineMappingRequest,
        nia_protocol_rust::DefineMappingRequest,
    > for NiaDefineMappingRequest
{
    fn to_pb(&self) -> nia_protocol_rust::DefineMappingRequest {
        let mut define_device_request_pb =
            nia_protocol_rust::DefineMappingRequest::new();

        let mapping_pb = self.mapping.to_pb();
        define_device_request_pb.set_mapping(mapping_pb);

        define_device_request_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::DefineMappingRequest,
    ) -> NiaServerResult<NiaDefineMappingRequest> {
        let mut object_pb = object_pb;

        let mapping = NiaMapping::from_pb(object_pb.take_mapping())?;

        Ok(NiaDefineMappingRequest::new(mapping))
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
        let mapping = NiaMapping::new(
            vec![NiaKeyChord::new(
                vec![NiaKey::make_key_2(1, 2), NiaKey::make_key_2(1, 3)],
                NiaKey::make_key_2(1, 4),
            )],
            NiaAction::new(NiaActionEnum::MouseRelativeMove(
                ActionMouseRelativeMove::new(100, 100),
            )),
        );

        let expected = NiaDefineMappingRequest::new(mapping);

        let bytes = expected.to_bytes().unwrap();
        let result = NiaDefineMappingRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
