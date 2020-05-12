use std::convert::TryFrom;

use crate::error::NiaServerError;

use crate::protocol::GetRequestType;
use crate::protocol::RequestType;

#[derive(Debug, Clone)]
pub struct NiaDefineKeyboardRequest {
    keyboard_path: String,
    keyboard_name: String,
}

impl NiaDefineKeyboardRequest {
    pub fn new<S>(
        keyboard_path: S,
        keyboard_name: S,
    ) -> NiaDefineKeyboardRequest
    where
        S: Into<String>,
    {
        NiaDefineKeyboardRequest {
            keyboard_path: keyboard_path.into(),
            keyboard_name: keyboard_name.into(),
        }
    }

    pub fn get_path_and_name(self) -> (String, String) {
        (self.keyboard_path, self.keyboard_name)
    }
}

impl TryFrom<nia_protocol_rust::DefineKeyboardRequest>
    for NiaDefineKeyboardRequest
{
    type Error = NiaServerError;

    fn try_from(
        define_keyboard_request: nia_protocol_rust::DefineKeyboardRequest,
    ) -> Result<Self, Self::Error> {
        let keyboard_path = define_keyboard_request.get_keyboard_path();
        let keyboard_name = define_keyboard_request.get_keyboard_name();

        Ok(NiaDefineKeyboardRequest::new(keyboard_path, keyboard_name))
    }
}
