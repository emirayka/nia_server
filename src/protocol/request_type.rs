use crate::protocol::{GetRequestType, NiaExecuteCodeRequest};
use std::convert::TryFrom;
use std::fmt::Formatter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestType {
    Handshake,
    GetDevices,
    ExecuteCode,
    DefineKeyboard,
    RemoveKeyboardByPath,
    RemoveKeyboardByName,
    GetDefinedModifiers,
    DefineModifier,
    RemoveModifier,
    // Sync,
    Unknown,
}

impl std::fmt::Display for RequestType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RequestType::Handshake => "RequestType::Handshake",
            RequestType::GetDevices => "RequestType::GetDevices",
            RequestType::ExecuteCode => "RequestType::ExecuteCode",
            RequestType::DefineKeyboard => "RequestType::DefineKeyboard",
            RequestType::RemoveKeyboardByPath => {
                "RequestType::RemoveKeyboardByPath"
            }
            RequestType::RemoveKeyboardByName => {
                "RequestType::RemoveKeyboardByName"
            }
            RequestType::GetDefinedModifiers => {
                "RequestType::GetDefinedModifiers"
            }
            RequestType::DefineModifier => "RequestType::DefineModifier",
            RequestType::RemoveModifier => "RequestType::RemoveModifier",
            // RequestType::Sync => "RequestType::Sync",
            RequestType::Unknown => "RequestType::Unknown",
        };

        write!(f, "{}", s)
    }
}

impl GetRequestType for nia_protocol_rust::Request {
    fn get_request_type(&self) -> RequestType {
        if self.has_handshake_request() {
            RequestType::Handshake
        } else if self.has_get_devices_request() {
            RequestType::GetDevices
        } else if self.has_execute_code_request() {
            RequestType::ExecuteCode
        } else if self.has_define_device_request() {
            RequestType::DefineKeyboard
        } else if self.has_remove_device_by_path_request() {
            RequestType::RemoveKeyboardByPath
        } else if self.has_remove_device_by_name_request() {
            RequestType::RemoveKeyboardByName
        } else if self.has_get_defined_modifiers_request() {
            RequestType::GetDefinedModifiers
        } else if self.has_define_modifier_request() {
            RequestType::DefineModifier
        } else if self.has_remove_modifier_request() {
            RequestType::RemoveModifier
        } else {
            RequestType::Unknown
        }
    }
}
