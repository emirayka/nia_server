use crate::error::NiaServerResult;
use crate::protocol::{NiaConvertable, NiaKey, Serializable};
use nia_interpreter_core::ModifierDescription;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NiaModifierDescription {
    key: NiaKey,
    alias: String,
}

impl NiaModifierDescription {
    pub fn new<S>(key: NiaKey, alias: S) -> NiaModifierDescription
    where
        S: Into<String>,
    {
        NiaModifierDescription {
            key,
            alias: alias.into(),
        }
    }

    pub fn get_key(&self) -> NiaKey {
        self.key
    }

    pub fn get_alias(&self) -> &String {
        &self.alias
    }
}

impl
    Serializable<NiaModifierDescription, nia_protocol_rust::ModifierDescription>
    for NiaModifierDescription
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
    ) -> NiaServerResult<NiaModifierDescription> {
        let mut object_pb = object_pb;

        let key = NiaKey::from_pb(object_pb.take_key())?;
        let alias = object_pb.take_alias().to_string();

        let modifier_description = NiaModifierDescription::new(key, alias);

        Ok(modifier_description)
    }
}

impl
    NiaConvertable<
        NiaModifierDescription,
        nia_interpreter_core::ModifierDescription,
    > for NiaModifierDescription
{
    fn to_interpreter_repr(&self) -> nia_interpreter_core::ModifierDescription {
        let interpreter_key = self.key.to_interpreter_repr();
        let alias = self.alias.clone();

        nia_interpreter_core::ModifierDescription::new(interpreter_key, alias)
    }

    fn from_interpreter_repr(
        object_pb: &nia_interpreter_core::ModifierDescription,
    ) -> NiaServerResult<NiaModifierDescription> {
        let key = NiaKey::from_interpreter_repr(&object_pb.get_key())?;
        let alias = object_pb.get_alias().clone();

        let modifier_description = (NiaModifierDescription::new(key, alias));

        Ok(modifier_description)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializes_and_deserializes() {
        let key_expected = NiaKey::make_key_2(1, 2);
        let alias_expected = "Control";

        let modifier_description =
            NiaModifierDescription::new(key_expected, alias_expected);
        let bytes = modifier_description.to_bytes().unwrap();
        let modifier_description =
            NiaModifierDescription::from_bytes(bytes).unwrap();

        let key_actual = modifier_description.get_key();
        let alias_actual = modifier_description.get_alias();

        assert_eq!(key_expected, key_actual);
        assert_eq!(alias_expected, alias_actual);
    }
}
