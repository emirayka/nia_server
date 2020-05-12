use std::convert::TryFrom;

use crate::error::NiaServerError;

use crate::protocol::GetRequestType;
use crate::protocol::RequestType;

#[derive(Debug, Clone)]
pub struct NiaDefineModifierRequest {
    keyboard_path: String,
    key_code: i32,
    modifier_alias: String,
}

impl NiaDefineModifierRequest {
    pub fn new<S>(
        keyboard_path: S,
        key_code: i32,
        modifier_alias: S,
    ) -> NiaDefineModifierRequest
    where
        S: Into<String>,
    {
        NiaDefineModifierRequest {
            keyboard_path: keyboard_path.into(),
            key_code,
            modifier_alias: modifier_alias.into(),
        }
    }

    pub fn into_tuple(self) -> (String, i32, String) {
        (self.keyboard_path, self.key_code, self.modifier_alias)
    }
}

impl TryFrom<nia_protocol_rust::DefineModifierRequest>
    for NiaDefineModifierRequest
{
    type Error = NiaServerError;

    fn try_from(
        define_modifier_request: nia_protocol_rust::DefineModifierRequest,
    ) -> Result<Self, Self::Error> {
        let keyboard_path = define_modifier_request.get_keyboard_path();
        let key_code = define_modifier_request.get_key_code();
        let modifier_alias = define_modifier_request.get_modifier_alias();

        Ok(NiaDefineModifierRequest::new(
            keyboard_path,
            key_code,
            modifier_alias,
        ))
    }
}
