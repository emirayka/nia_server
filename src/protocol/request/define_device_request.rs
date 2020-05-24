use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::Serializable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaDefineDeviceRequest {
    device_id: i32,
}

impl NiaDefineDeviceRequest {
    pub fn new(device_id: i32) -> NiaDefineDeviceRequest {
        NiaDefineDeviceRequest { device_id }
    }

    pub fn get_device_id(self) -> i32 {
        self.device_id
    }
}

impl
    Serializable<NiaDefineDeviceRequest, nia_protocol_rust::DefineDeviceRequest>
    for NiaDefineDeviceRequest
{
    fn to_pb(&self) -> nia_protocol_rust::DefineDeviceRequest {
        let mut define_device_request_pb =
            nia_protocol_rust::DefineDeviceRequest::new();

        define_device_request_pb.set_device_id(self.device_id);

        define_device_request_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::DefineDeviceRequest,
    ) -> NiaServerResult<NiaDefineDeviceRequest> {
        let device_id = object_pb.get_device_id();

        Ok(NiaDefineDeviceRequest::new(device_id))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaDefineDeviceRequest::new(2);

        let bytes = expected.to_bytes().unwrap();
        let result = NiaDefineDeviceRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result);
    }
}
