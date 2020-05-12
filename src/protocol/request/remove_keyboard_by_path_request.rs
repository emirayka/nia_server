use std::convert::TryFrom;

use crate::error::NiaServerError;

use crate::protocol::GetRequestType;
use crate::protocol::RequestType;

#[derive(Debug, Clone)]
pub struct NiaRemoveKeyboardByPathRequest {
    keyboard_path: String,
}

impl NiaRemoveKeyboardByPathRequest {
    pub fn new<S>(keyboard_path: S) -> NiaRemoveKeyboardByPathRequest
    where
        S: Into<String>,
    {
        NiaRemoveKeyboardByPathRequest {
            keyboard_path: keyboard_path.into(),
        }
    }

    pub fn get_keyboard_path(self) -> String {
        self.keyboard_path
    }
}

impl TryFrom<nia_protocol_rust::RemoveKeyboardByPathRequest>
    for NiaRemoveKeyboardByPathRequest
{
    type Error = NiaServerError;

    fn try_from(
        remove_keyboard_by_path_request: nia_protocol_rust::RemoveKeyboardByPathRequest,
    ) -> Result<Self, Self::Error> {
        let keyboard_path = remove_keyboard_by_path_request.get_keyboard_path();

        Ok(NiaRemoveKeyboardByPathRequest::new(keyboard_path))
    }
}
