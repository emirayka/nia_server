use std::convert::TryFrom;

use crate::protocol::*;

use crate::error::NiaServerError;

#[derive(Debug, Clone)]
pub enum NiaRequest {
    Handshake(NiaHandshakeRequest),
    GetDevices(NiaGetDevicesRequest),
    GetDeviceInfo(NiaGetDeviceInfoRequest),
    ExecuteCode(NiaExecuteCodeRequest),
    DefineKeyboard(NiaDefineKeyboardRequest),
    RemoveKeyboardByPath(NiaRemoveKeyboardByPathRequest),
    RemoveKeyboardByName(NiaRemoveKeyboardByNameRequest),
    GetDefinedModifiers(NiaGetDefinedModifiersRequest),
    DefineModifier(NiaDefineModifierRequest),
    RemoveModifier(NiaRemoveModifierRequest),
}

macro_rules! make_from_implementation {
    ($type_name:ident, $variant:path) => {
        impl From<$type_name> for NiaRequest {
            fn from(v: $type_name) -> Self {
                $variant(v)
            }
        }
    };
}

make_from_implementation!(NiaHandshakeRequest, NiaRequest::Handshake);
make_from_implementation!(NiaGetDevicesRequest, NiaRequest::GetDevices);
make_from_implementation!(NiaGetDeviceInfoRequest, NiaRequest::GetDeviceInfo);
make_from_implementation!(NiaExecuteCodeRequest, NiaRequest::ExecuteCode);
make_from_implementation!(NiaDefineKeyboardRequest, NiaRequest::DefineKeyboard);
make_from_implementation!(
    NiaRemoveKeyboardByPathRequest,
    NiaRequest::RemoveKeyboardByPath
);
make_from_implementation!(
    NiaRemoveKeyboardByNameRequest,
    NiaRequest::RemoveKeyboardByName
);
make_from_implementation!(
    NiaGetDefinedModifiersRequest,
    NiaRequest::GetDefinedModifiers
);
make_from_implementation!(NiaDefineModifierRequest, NiaRequest::DefineModifier);
make_from_implementation!(NiaRemoveModifierRequest, NiaRequest::RemoveModifier);

impl TryFrom<nia_protocol_rust::Request> for NiaRequest {
    type Error = NiaServerError;

    fn try_from(
        request: nia_protocol_rust::Request,
    ) -> Result<Self, Self::Error> {
        let mut request = request;
        let request_type = request.get_request_type();

        let request = match request_type {
            RequestType::Handshake => {
                NiaHandshakeRequest::try_from(request.take_handshake_request())?
                    .into()
            }
            RequestType::GetDevices => NiaGetDevicesRequest::try_from(
                request.take_get_devices_request(),
            )?
            .into(),
            RequestType::GetDeviceInfo => NiaGetDeviceInfoRequest::try_from(
                request.take_get_device_info_request(),
            )?
            .into(),
            RequestType::ExecuteCode => NiaExecuteCodeRequest::try_from(
                request.take_execute_code_request(),
            )?
            .into(),
            RequestType::DefineKeyboard => NiaDefineKeyboardRequest::try_from(
                request.take_define_keyboard_request(),
            )?
            .into(),
            RequestType::RemoveKeyboardByPath => {
                NiaRemoveKeyboardByPathRequest::try_from(
                    request.take_remove_keyboard_by_path_request(),
                )?
                .into()
            }
            RequestType::RemoveKeyboardByName => {
                NiaRemoveKeyboardByNameRequest::try_from(
                    request.take_remove_keyboard_by_name_request(),
                )?
                .into()
            }
            RequestType::GetDefinedModifiers => {
                NiaGetDefinedModifiersRequest::try_from(
                    request.take_get_defined_modifiers_request(),
                )?
                .into()
            }
            RequestType::DefineModifier => NiaDefineModifierRequest::try_from(
                request.take_define_modifier_request(),
            )?
            .into(),
            RequestType::RemoveModifier => NiaRemoveModifierRequest::try_from(
                request.take_remove_modifier_request(),
            )?
            .into(),
            RequestType::Unknown => {
                let error = NiaServerError::invalid_request(
                    "[NiaRequest::try_from(Request)]: Unknown request was received.",
                );

                return Err(error);
            }
        };

        Ok(request)
    }
}
