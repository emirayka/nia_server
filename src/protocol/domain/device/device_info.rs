use crate::error::NiaServerResult;
use crate::protocol::{DeviceModel, Serializable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceInfo {
    device_id: i32,
    defined: bool,
    device_path: String,
    device_name: String,
    device_model: DeviceModel,
}

impl DeviceInfo {
    pub fn new<S>(
        device_id: i32,
        defined: bool,
        device_path: S,
        device_name: S,
        device_model: DeviceModel,
    ) -> DeviceInfo
    where
        S: Into<String>,
    {
        DeviceInfo {
            device_id,
            defined,
            device_path: device_path.into(),
            device_name: device_name.into(),
            device_model,
        }
    }

    pub fn get_device_id(&self) -> i32 {
        self.device_id
    }

    pub fn is_defined(&self) -> bool {
        self.defined
    }

    pub fn set_defined(&mut self, value: bool) {
        self.defined = value;
    }

    pub fn get_device_path(&self) -> &str {
        &self.device_path
    }

    pub fn get_device_name(&self) -> &str {
        &self.device_name
    }

    pub fn get_device_model(&self) -> &DeviceModel {
        &self.device_model
    }
}

impl Serializable<DeviceInfo, nia_protocol_rust::DeviceInfo> for DeviceInfo {
    fn to_pb(&self) -> nia_protocol_rust::DeviceInfo {
        let device_model_pb = self.device_model.to_pb();
        let mut device_info_pb = nia_protocol_rust::DeviceInfo::new();

        device_info_pb.set_device_id(self.get_device_id());
        device_info_pb.set_defined(self.is_defined());
        device_info_pb
            .set_device_path(protobuf::Chars::from(self.device_path.clone()));
        device_info_pb
            .set_device_name(protobuf::Chars::from(self.device_name.clone()));
        device_info_pb.set_device_model(device_model_pb);

        device_info_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::DeviceInfo,
    ) -> NiaServerResult<DeviceInfo> {
        let mut object_pb = object_pb;

        let device_id = object_pb.get_device_id();
        let defined = object_pb.get_defined();
        let device_path = object_pb.get_device_path().to_string();
        let device_name = object_pb.get_device_name().to_string();
        let device_model = DeviceModel::from_pb(object_pb.take_device_model())?;

        let device_info =
            DeviceInfo::new(device_id, defined, device_path, device_name, device_model);

        Ok(device_info)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::protocol::KeyDescription;

    #[test]
    fn serializes_and_deserializes() {
        let device_model = DeviceModel::new(
            vec![
                KeyDescription::new(1, 2, 3, 4, 5),
                KeyDescription::new(3, 2, 4, 1, 2),
            ],
            100,
            200,
        );

        let expected_device_info = DeviceInfo::new(
            1,
            true,
            String::from("/dev/input/event0"),
            String::from("Corsair Keyboard"),
            device_model,
        );

        let bytes = expected_device_info.to_bytes().unwrap();
        let actual_device_info = DeviceInfo::from_bytes(bytes).unwrap();

        assert_eq!(expected_device_info, actual_device_info);
    }
}
