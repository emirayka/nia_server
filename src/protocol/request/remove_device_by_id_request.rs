use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::Serializable;
use nia_protocol_rust::RemoveDeviceByIdRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaRemoveDeviceByIdRequest {
    device_id: i32,
}

impl NiaRemoveDeviceByIdRequest {
    pub fn new(device_id: i32) -> NiaRemoveDeviceByIdRequest {
        NiaRemoveDeviceByIdRequest { device_id }
    }

    pub fn get_device_id(self) -> i32 {
        self.device_id
    }
}

impl
    Serializable<
        NiaRemoveDeviceByIdRequest,
        nia_protocol_rust::RemoveDeviceByIdRequest,
    > for NiaRemoveDeviceByIdRequest
{
    fn to_pb(&self) -> RemoveDeviceByIdRequest {
        let mut remove_device_by_id_request_pb =
            nia_protocol_rust::RemoveDeviceByIdRequest::new();

        remove_device_by_id_request_pb.set_device_id(self.device_id);

        remove_device_by_id_request_pb
    }

    fn from_pb(
        object_pb: RemoveDeviceByIdRequest,
    ) -> NiaServerResult<NiaRemoveDeviceByIdRequest> {
        let remove_device_by_name_request =
            NiaRemoveDeviceByIdRequest::new(object_pb.get_device_id());

        Ok(remove_device_by_name_request)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaRemoveDeviceByIdRequest::new(2);

        let bytes = expected.to_bytes().unwrap();
        let result = NiaRemoveDeviceByIdRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result)
    }
}
