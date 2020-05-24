use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::Serializable;
use nia_protocol_rust::RemoveDeviceByNameRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NiaRemoveDeviceByNameRequest {
    device_name: String,
}

impl NiaRemoveDeviceByNameRequest {
    pub fn new<S>(device_name: S) -> NiaRemoveDeviceByNameRequest
    where
        S: Into<String>,
    {
        NiaRemoveDeviceByNameRequest {
            device_name: device_name.into(),
        }
    }

    pub fn get_device_name(self) -> String {
        self.device_name
    }
}

impl
    Serializable<
        NiaRemoveDeviceByNameRequest,
        nia_protocol_rust::RemoveDeviceByNameRequest,
    > for NiaRemoveDeviceByNameRequest
{
    fn to_pb(&self) -> RemoveDeviceByNameRequest {
        let mut remove_device_by_name_request_pb =
            nia_protocol_rust::RemoveDeviceByNameRequest::new();

        remove_device_by_name_request_pb
            .set_device_name(protobuf::Chars::from(self.device_name.clone()));

        remove_device_by_name_request_pb
    }

    fn from_pb(
        object_pb: RemoveDeviceByNameRequest,
    ) -> NiaServerResult<NiaRemoveDeviceByNameRequest> {
        let remove_device_by_name_request =
            NiaRemoveDeviceByNameRequest::new(object_pb.get_device_name());

        Ok(remove_device_by_name_request)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected = NiaRemoveDeviceByNameRequest::new("Corsair kyboard");

        let bytes = expected.to_bytes().unwrap();
        let result = NiaRemoveDeviceByNameRequest::from_bytes(bytes).unwrap();

        assert_eq!(expected, result)
    }
}
