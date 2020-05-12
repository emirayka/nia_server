use std::convert::TryFrom;

use crate::error::NiaServerError;

use crate::protocol::GetRequestType;
use crate::protocol::RequestType;

#[derive(Debug, Clone)]
pub struct NiaRemoveModifierRequest {
    keyboard_path: String,
    key_code: i32,
}

impl NiaRemoveModifierRequest {
    pub fn new<S>(keyboard_path: S, key_code: i32) -> NiaRemoveModifierRequest
    where
        S: Into<String>,
    {
        NiaRemoveModifierRequest {
            keyboard_path: keyboard_path.into(),
            key_code,
        }
    }

    pub fn get_path_and_key_code(self) -> (String, i32) {
        (self.keyboard_path, self.key_code)
    }
}

impl TryFrom<nia_protocol_rust::RemoveModifierRequest>
    for NiaRemoveModifierRequest
{
    type Error = NiaServerError;

    fn try_from(
        remove_modifier_request: nia_protocol_rust::RemoveModifierRequest,
    ) -> Result<Self, Self::Error> {
        let keyboard_path = remove_modifier_request.get_keyboard_path();
        let key_code = remove_modifier_request.get_key_code();

        Ok(NiaRemoveModifierRequest::new(keyboard_path, key_code))
    }
}
