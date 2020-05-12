use std::convert::TryFrom;

use crate::error::NiaServerError;

use crate::protocol::GetRequestType;
use crate::protocol::RequestType;

#[derive(Debug, Clone)]
pub struct NiaRemoveKeyboardByNameRequest {
    keyboard_name: String,
}

impl NiaRemoveKeyboardByNameRequest {
    pub fn new<S>(keyboard_name: S) -> NiaRemoveKeyboardByNameRequest
    where
        S: Into<String>,
    {
        NiaRemoveKeyboardByNameRequest {
            keyboard_name: keyboard_name.into(),
        }
    }

    pub fn get_keyboard_name(self) -> String {
        self.keyboard_name
    }
}

impl TryFrom<nia_protocol_rust::RemoveKeyboardByNameRequest>
    for NiaRemoveKeyboardByNameRequest
{
    type Error = NiaServerError;

    fn try_from(
        remove_keyboard_by_name_request: nia_protocol_rust::RemoveKeyboardByNameRequest,
    ) -> Result<Self, Self::Error> {
        let keyboard_name = remove_keyboard_by_name_request.get_keyboard_name();

        Ok(NiaRemoveKeyboardByNameRequest::new(keyboard_name))
    }
}
