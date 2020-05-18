use crate::error::{NiaServerError, NiaServerResult};
use crate::protocol::{KeyDescription, Serializable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceModel {
    key_descriptions: Vec<KeyDescription>,
    device_width: i32,
    device_height: i32,
}

impl DeviceModel {
    pub fn new(
        key_descriptions: Vec<KeyDescription>,
        device_width: i32,
        device_height: i32,
    ) -> DeviceModel {
        DeviceModel {
            key_descriptions,
            device_width,
            device_height,
        }
    }

    pub fn from_string(string: String) -> NiaServerResult<DeviceModel> {
        let integers: Vec<i32> = string
            .lines()
            .flat_map(|line| line.split_whitespace())
            .map(|part| part.parse().expect("Cannot convert an integer"))
            .collect();

        let mut iter = integers.into_iter().peekable();

        let device_width = iter.next().ok_or_else(|| {
            NiaServerError::deserialization_error("Invalid kbm file")
        })?;

        let device_height = iter.next().ok_or_else(|| {
            NiaServerError::deserialization_error("Invalid kbm file")
        })?;

        let mut key_descriptions = Vec::new();

        while iter.peek().is_some() {
            let x = iter.next().ok_or_else(|| {
                NiaServerError::deserialization_error("Invalid kbm file")
            })?;

            let y = iter.next().ok_or_else(|| {
                NiaServerError::deserialization_error("Invalid kbm file")
            })?;

            let width = iter.next().ok_or_else(|| {
                NiaServerError::deserialization_error("Invalid kbm file")
            })?;

            let height = iter.next().ok_or_else(|| {
                NiaServerError::deserialization_error("Invalid kbm file")
            })?;

            let key_code = iter.next().ok_or_else(|| {
                NiaServerError::deserialization_error("Invalid kbm file")
            })?;

            let key_description =
                KeyDescription::new(x, y, width, height, key_code);

            key_descriptions.push(key_description)
        }

        let device_model =
            DeviceModel::new(key_descriptions, device_width, device_height);

        Ok(device_model)
    }

    pub fn get_key_descriptions(&self) -> &Vec<KeyDescription> {
        &self.key_descriptions
    }

    pub fn get_device_width(&self) -> i32 {
        self.device_width
    }

    pub fn get_device_height(&self) -> i32 {
        self.device_height
    }
}

impl Serializable<DeviceModel, nia_protocol_rust::DeviceModel> for DeviceModel {
    fn to_pb(&self) -> nia_protocol_rust::DeviceModel {
        let mut keyboard_model_pb = nia_protocol_rust::DeviceModel::new();

        keyboard_model_pb.set_device_height(self.device_height);
        keyboard_model_pb.set_device_width(self.device_width);
        keyboard_model_pb.set_key_descriptions(
            self.key_descriptions
                .iter()
                .map(|key_description| key_description.to_pb())
                .collect(),
        );

        keyboard_model_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::DeviceModel,
    ) -> NiaServerResult<DeviceModel> {
        let mut object_pb = object_pb;

        let width = object_pb.get_device_width();
        let height = object_pb.get_device_height();
        let mut key_descriptions = Vec::new();

        for key_description_pb in object_pb.take_key_descriptions().into_iter()
        {
            let key_description = KeyDescription::from_pb(key_description_pb)?;

            key_descriptions.push(key_description);
        }

        let keyboard_model = DeviceModel::new(key_descriptions, width, height);

        Ok(keyboard_model)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected_keyboard_model = DeviceModel::new(
            vec![
                KeyDescription::new(1, 2, 3, 4, 5),
                KeyDescription::new(3, 2, 4, 1, 2),
            ],
            100,
            200,
        );

        let bytes = expected_keyboard_model.to_bytes().unwrap();
        let actual_keyboard_model = DeviceModel::from_bytes(bytes).unwrap();

        assert_eq!(expected_keyboard_model, actual_keyboard_model)
    }
}
