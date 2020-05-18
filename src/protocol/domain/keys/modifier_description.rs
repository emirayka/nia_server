use crate::error::NiaServerResult;
use crate::protocol::{Key, Serializable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModifierDescription {
    key: Key,
    alias: String,
}

impl ModifierDescription {
    pub fn new<S>(key: Key, alias: S) -> ModifierDescription
    where
        S: Into<String>,
    {
        ModifierDescription {
            key,
            alias: alias.into(),
        }
    }

    pub fn get_key(&self) -> Key {
        self.key
    }

    pub fn get_alias(&self) -> &String {
        &self.alias
    }
}

impl Serializable<ModifierDescription, nia_protocol_rust::ModifierDescription>
    for ModifierDescription
{
    fn to_pb(&self) -> nia_protocol_rust::ModifierDescription {
        let mut modifier_description_pb =
            nia_protocol_rust::ModifierDescription::new();

        let key_pb = self.key.to_pb();

        modifier_description_pb.set_key(key_pb);
        modifier_description_pb
            .set_alias(protobuf::Chars::from(self.alias.clone()));

        modifier_description_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ModifierDescription,
    ) -> NiaServerResult<ModifierDescription> {
        let mut object_pb = object_pb;

        let key = Key::from_pb(object_pb.take_key())?;
        let alias = object_pb.take_alias().to_string();

        let modifier_description = ModifierDescription::new(key, alias);

        Ok(modifier_description)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let key_expected = Key::make_key_2(1, 2);
        let alias_expected = "Control";

        let modifier_description =
            ModifierDescription::new(key_expected, alias_expected);
        let bytes = modifier_description.to_bytes().unwrap();
        let modifier_description =
            ModifierDescription::from_bytes(bytes).unwrap();

        let key_actual = modifier_description.get_key();
        let alias_actual = modifier_description.get_alias();

        assert_eq!(key_expected, key_actual);
        assert_eq!(alias_expected, alias_actual);
    }
}
