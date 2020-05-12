use std::sync::MutexGuard;

use nia_interpreter_core::EventLoopHandle;

use crate::error::NiaServerError;

use crate::protocol::NiaDefineModifierResponse;
use crate::protocol::NiaExecuteCodeResponse;
use crate::protocol::NiaGetDeviceInfoResponse;
use crate::protocol::NiaGetDevicesResponse;
use crate::protocol::NiaHandshakeResponse;
use crate::protocol::NiaRemoveKeyboardByNameResponse;
use crate::protocol::NiaRemoveKeyboardByPathResponse;
use crate::protocol::NiaRemoveModifierResponse;
use crate::protocol::NiaRequest;
use crate::protocol::{
    NiaDefineKeyboardResponse, NiaGetDefinedModifiersResponse,
};

#[derive(Debug, Clone)]
pub enum NiaResponse {
    Handshake(NiaHandshakeResponse),
    GetDevices(NiaGetDevicesResponse),
    GetDeviceInfo(NiaGetDeviceInfoResponse),
    ExecuteCode(NiaExecuteCodeResponse),
    DefineKeyboard(NiaDefineKeyboardResponse),
    RemoveKeyboardByPath(NiaRemoveKeyboardByPathResponse),
    RemoveKeyboardByName(NiaRemoveKeyboardByNameResponse),
    GetDefinedModifiers(NiaGetDefinedModifiersResponse),
    DefineModifier(NiaDefineModifierResponse),
    RemoveModifier(NiaRemoveModifierResponse),
}

impl NiaResponse {
    pub fn from(
        nia_request: NiaRequest,
        event_loop_handle: MutexGuard<EventLoopHandle>,
    ) -> NiaResponse {
        let nia_response = match nia_request {
            NiaRequest::Handshake(nia_handshake_request) => {
                let nia_handshake_response =
                    NiaHandshakeResponse::from(nia_handshake_request);

                NiaResponse::Handshake(nia_handshake_response)
            }
            NiaRequest::GetDevices(nia_get_devices_request) => {
                let nia_get_devices_response =
                    NiaGetDevicesResponse::from(nia_get_devices_request);

                NiaResponse::GetDevices(nia_get_devices_response)
            }
            NiaRequest::GetDeviceInfo(nia_get_device_info_request) => {
                let nia_get_device_info_response =
                    NiaGetDeviceInfoResponse::from(nia_get_device_info_request);

                NiaResponse::GetDeviceInfo(nia_get_device_info_response)
            }
            NiaRequest::ExecuteCode(nia_execute_code_request) => {
                let nia_execute_code_response = NiaExecuteCodeResponse::from(
                    nia_execute_code_request,
                    event_loop_handle,
                );

                NiaResponse::ExecuteCode(nia_execute_code_response)
            }
            NiaRequest::DefineKeyboard(nia_define_keyboard_request) => {
                let nia_define_keyboard_response =
                    NiaDefineKeyboardResponse::from(
                        nia_define_keyboard_request,
                        event_loop_handle,
                    );

                NiaResponse::DefineKeyboard(nia_define_keyboard_response)
            }
            NiaRequest::RemoveKeyboardByPath(
                nia_remove_keyboard_by_path_request,
            ) => {
                let nia_remove_keyboard_by_path_response =
                    NiaRemoveKeyboardByPathResponse::from(
                        nia_remove_keyboard_by_path_request,
                        event_loop_handle,
                    );

                NiaResponse::RemoveKeyboardByPath(
                    nia_remove_keyboard_by_path_response,
                )
            }
            NiaRequest::RemoveKeyboardByName(
                nia_remove_keyboard_by_name_request,
            ) => {
                let nia_remove_keyboard_by_name_response =
                    NiaRemoveKeyboardByNameResponse::from(
                        nia_remove_keyboard_by_name_request,
                        event_loop_handle,
                    );

                NiaResponse::RemoveKeyboardByName(
                    nia_remove_keyboard_by_name_response,
                )
            }
            NiaRequest::GetDefinedModifiers(
                nia_get_defined_modifiers_request,
            ) => {
                let nia_get_defined_modifiers_response =
                    NiaGetDefinedModifiersResponse::from(
                        nia_get_defined_modifiers_request,
                        event_loop_handle,
                    );

                NiaResponse::GetDefinedModifiers(
                    nia_get_defined_modifiers_response,
                )
            }
            NiaRequest::DefineModifier(nia_define_modifier_request) => {
                let nia_define_modifier_response =
                    NiaDefineModifierResponse::from(
                        nia_define_modifier_request,
                        event_loop_handle,
                    );

                NiaResponse::DefineModifier(nia_define_modifier_response)
            }
            NiaRequest::RemoveModifier(nia_remove_keyboard_request) => {
                let nia_remove_keyboard_response =
                    NiaRemoveModifierResponse::from(
                        nia_remove_keyboard_request,
                        event_loop_handle,
                    );

                NiaResponse::RemoveModifier(nia_remove_keyboard_response)
            }
        };

        nia_response
    }
}

impl From<NiaResponse> for nia_protocol_rust::Response {
    fn from(nia_response: NiaResponse) -> Self {
        let mut response = nia_protocol_rust::Response::new();

        match nia_response {
            NiaResponse::Handshake(nia_handshake_response) => {
                let handshake_response = nia_handshake_response.into();

                response.set_handshake_response(handshake_response);
            }
            NiaResponse::GetDevices(nia_get_devices_response) => {
                let get_devices_response = nia_get_devices_response.into();

                response.set_get_devices_response(get_devices_response);
                println!("{:?}", response);
            }
            NiaResponse::GetDeviceInfo(nia_get_device_info_response) => {
                let get_device_info_response =
                    nia_get_device_info_response.into();

                response.set_get_device_info_response(get_device_info_response);
            }
            NiaResponse::ExecuteCode(nia_execute_code_response) => {
                let execute_code_response = nia_execute_code_response.into();

                response.set_execute_code_response(execute_code_response);
            }
            NiaResponse::DefineKeyboard(nia_define_keyboard_response) => {
                let define_keyboard_response =
                    nia_define_keyboard_response.into();

                response.set_define_keyboard_response(define_keyboard_response);
            }
            NiaResponse::RemoveKeyboardByPath(
                nia_remove_keyboard_by_path_response,
            ) => {
                let remove_keyboard_by_path =
                    nia_remove_keyboard_by_path_response.into();

                response.set_remove_keyboard_by_path_response(
                    remove_keyboard_by_path,
                );
            }
            NiaResponse::RemoveKeyboardByName(
                nia_remove_keyboard_by_name_response,
            ) => {
                let remove_keyboard_by_name =
                    nia_remove_keyboard_by_name_response.into();

                response.set_remove_keyboard_by_name_response(
                    remove_keyboard_by_name,
                );
            }
            NiaResponse::GetDefinedModifiers(
                nia_get_defined_modifiers_response,
            ) => {
                let get_defined_modifiers =
                    nia_get_defined_modifiers_response.into();

                response
                    .set_get_defined_modifiers_response(get_defined_modifiers);
            }
            NiaResponse::DefineModifier(nia_define_modifier_response) => {
                let define_modifier = nia_define_modifier_response.into();

                response.set_define_modifier_response(define_modifier);
            }
            NiaResponse::RemoveModifier(nia_remove_modifier_response) => {
                let remove_modifier = nia_remove_modifier_response.into();

                response.set_remove_modifier_response(remove_modifier);
            }
        }

        response
    }
}
