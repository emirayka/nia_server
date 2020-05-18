use crate::error::NiaServerResult;
use crate::protocol::Serializable;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyDescription {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    key_code: i32,
}

impl KeyDescription {
    pub fn new(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        key_code: i32,
    ) -> KeyDescription {
        KeyDescription {
            x,
            y,
            width,
            height,
            key_code,
        }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_key_code(&self) -> i32 {
        self.key_code
    }
}

impl Serializable<KeyDescription, nia_protocol_rust::KeyDescription>
    for KeyDescription
{
    fn to_pb(&self) -> nia_protocol_rust::KeyDescription {
        let mut key_description_pb = nia_protocol_rust::KeyDescription::new();

        key_description_pb.set_x(self.x);
        key_description_pb.set_y(self.y);
        key_description_pb.set_width(self.width);
        key_description_pb.set_height(self.height);
        key_description_pb.set_key_code(self.key_code);

        key_description_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::KeyDescription,
    ) -> NiaServerResult<KeyDescription> {
        let mut key_description_pb = object_pb;

        let x = key_description_pb.get_x();
        let y = key_description_pb.get_y();
        let width = key_description_pb.get_width();
        let height = key_description_pb.get_height();
        let key_code = key_description_pb.get_key_code();

        let key_description =
            KeyDescription::new(x, y, width, height, key_code);

        Ok(key_description)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let expected_key_description = KeyDescription::new(1, 2, 3, 4, 5);

        let bytes = expected_key_description.to_bytes().unwrap();
        let actual_key_description = KeyDescription::from_bytes(bytes).unwrap();

        assert_eq!(expected_key_description, actual_key_description);
    }
}
