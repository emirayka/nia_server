use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::Serializable;
use nia_protocol_rust::RemoveDeviceByPathRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaRemoveDeviceByPathRequest {
    device_path: String,
}

impl NiaRemoveDeviceByPathRequest {
    pub fn new<S>(device_path: S) -> NiaRemoveDeviceByPathRequest
    where
        S: Into<String>,
    {
        NiaRemoveDeviceByPathRequest {
            device_path: device_path.into(),
        }
    }

    pub fn get_device_path(self) -> String {
        self.device_path
    }
}

impl
    Serializable<
        NiaRemoveDeviceByPathRequest,
        nia_protocol_rust::RemoveDeviceByPathRequest,
    > for NiaRemoveDeviceByPathRequest
{
    fn to_pb(&self) -> RemoveDeviceByPathRequest {
        let mut remove_device_by_path_request_pb =
            nia_protocol_rust::RemoveDeviceByPathRequest::new();

        remove_device_by_path_request_pb
            .set_device_path(protobuf::Chars::from(self.device_path.clone()));

        remove_device_by_path_request_pb
    }

    fn from_pb(
        object_pb: RemoveDeviceByPathRequest,
    ) -> NiaServerResult<NiaRemoveDeviceByPathRequest> {
        let remove_device_by_path_request =
            NiaRemoveDeviceByPathRequest::new(object_pb.get_device_path());

        Ok(remove_device_by_path_request)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaRemoveDeviceByPathRequest::new("/dev/input/event6");

        let bytes = expected.to_bytes().unwrap();
        let result = NiaRemoveDeviceByPathRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result)
    }
}
