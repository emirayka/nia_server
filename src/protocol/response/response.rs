use std::sync::MutexGuard;

use nia_interpreter_core::EventLoopHandle;

use crate::error::{NiaServerError, NiaServerResult};

use crate::protocol::NiaHandshakeResponse;
use crate::protocol::NiaRemoveDeviceByNameResponse;
use crate::protocol::NiaRemoveDeviceByPathResponse;
use crate::protocol::NiaRemoveModifierResponse;
use crate::protocol::NiaRequest;
use crate::protocol::{
    NiaDefineActionResponse, NiaExecuteCodeResponse,
    NiaGetDefinedActionsResponse,
};
use crate::protocol::{
    NiaDefineDeviceResponse, NiaGetDefinedModifiersResponse,
};
use crate::protocol::{NiaDefineModifierResponse, Serializable};
use crate::protocol::{NiaGetDevicesResponse, NiaRemoveActionResponse};
use crate::server::Server;
use nia_protocol_rust::Response;

#[derive(Debug, Clone)]
pub enum NiaResponse {
    Handshake(NiaHandshakeResponse),
    GetDevices(NiaGetDevicesResponse),
    ExecuteCode(NiaExecuteCodeResponse),
    DefineKeyboard(NiaDefineDeviceResponse),
    RemoveKeyboardByPath(NiaRemoveDeviceByPathResponse),
    RemoveKeyboardByName(NiaRemoveDeviceByNameResponse),
    GetDefinedModifiers(NiaGetDefinedModifiersResponse),
    DefineModifier(NiaDefineModifierResponse),
    RemoveModifier(NiaRemoveModifierResponse),
    GetDefinedActions(NiaGetDefinedActionsResponse),
    DefineAction(NiaDefineActionResponse),
    RemoveAction(NiaRemoveActionResponse),
}

impl NiaResponse {
    pub fn from(
        server: &mut MutexGuard<Server>,
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
                let nia_get_devices_response = NiaGetDevicesResponse::from(
                    server,
                    nia_get_devices_request,
                );

                NiaResponse::GetDevices(nia_get_devices_response)
            }
            NiaRequest::ExecuteCode(nia_execute_code_request) => {
                let nia_execute_code_response = NiaExecuteCodeResponse::from(
                    nia_execute_code_request,
                    event_loop_handle,
                );

                NiaResponse::ExecuteCode(nia_execute_code_response)
            }
            NiaRequest::DefineDevice(nia_define_keyboard_request) => {
                let nia_define_keyboard_response =
                    NiaDefineDeviceResponse::from(
                        server,
                        nia_define_keyboard_request,
                        event_loop_handle,
                    );

                NiaResponse::DefineKeyboard(nia_define_keyboard_response)
            }
            NiaRequest::RemoveDeviceByPath(
                nia_remove_keyboard_by_path_request,
            ) => {
                let nia_remove_keyboard_by_path_response =
                    NiaRemoveDeviceByPathResponse::from(
                        server,
                        nia_remove_keyboard_by_path_request,
                        event_loop_handle,
                    );

                NiaResponse::RemoveKeyboardByPath(
                    nia_remove_keyboard_by_path_response,
                )
            }
            NiaRequest::RemoveDeviceByName(
                nia_remove_keyboard_by_name_request,
            ) => {
                let nia_remove_keyboard_by_name_response =
                    NiaRemoveDeviceByNameResponse::from(
                        server,
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
            NiaRequest::GetDefinedActions(nia_get_defined_actions_request) => {
                let nia_get_defined_actions_response =
                    NiaGetDefinedActionsResponse::from(
                        nia_get_defined_actions_request,
                        event_loop_handle,
                    );

                NiaResponse::GetDefinedActions(nia_get_defined_actions_response)
            }
            NiaRequest::DefineAction(nia_define_action_request) => {
                let nia_define_action_response = NiaDefineActionResponse::from(
                    nia_define_action_request,
                    event_loop_handle,
                );

                NiaResponse::DefineAction(nia_define_action_response)
            }
            NiaRequest::RemoveAction(nia_remove_action_request) => {
                let nia_remove_action_response = NiaRemoveActionResponse::from(
                    nia_remove_action_request,
                    event_loop_handle,
                );

                NiaResponse::RemoveAction(nia_remove_action_response)
            }
        };

        nia_response
    }
}

impl Serializable<NiaResponse, nia_protocol_rust::Response> for NiaResponse {
    fn to_pb(&self) -> Response {
        let mut response = nia_protocol_rust::Response::new();

        match &self {
            NiaResponse::Handshake(nia_handshake_response) => {
                let handshake_response = nia_handshake_response.to_pb();

                response.set_handshake_response(handshake_response);
            }
            NiaResponse::GetDevices(nia_get_devices_response) => {
                let get_devices_response = nia_get_devices_response.to_pb();

                response.set_get_devices_response(get_devices_response);
                println!("{:?}", response);
            }
            NiaResponse::ExecuteCode(nia_execute_code_response) => {
                let execute_code_response = nia_execute_code_response.to_pb();

                response.set_execute_code_response(execute_code_response);
            }
            NiaResponse::DefineKeyboard(nia_define_keyboard_response) => {
                let define_keyboard_response =
                    nia_define_keyboard_response.to_pb();

                response.set_define_device_response(define_keyboard_response);
            }
            NiaResponse::RemoveKeyboardByPath(
                nia_remove_keyboard_by_path_response,
            ) => {
                let remove_keyboard_by_path =
                    nia_remove_keyboard_by_path_response.to_pb();

                response.set_remove_device_by_path_response(
                    remove_keyboard_by_path,
                );
            }
            NiaResponse::RemoveKeyboardByName(
                nia_remove_keyboard_by_name_response,
            ) => {
                let remove_keyboard_by_name =
                    nia_remove_keyboard_by_name_response.to_pb();

                response.set_remove_device_by_name_response(
                    remove_keyboard_by_name,
                );
            }
            NiaResponse::GetDefinedModifiers(
                nia_get_defined_modifiers_response,
            ) => {
                let get_defined_modifiers =
                    nia_get_defined_modifiers_response.to_pb();

                response
                    .set_get_defined_modifiers_response(get_defined_modifiers);
            }
            NiaResponse::DefineModifier(nia_define_modifier_response) => {
                let define_modifier = nia_define_modifier_response.to_pb();

                response.set_define_modifier_response(define_modifier);
            }
            NiaResponse::RemoveModifier(nia_remove_modifier_response) => {
                let remove_modifier = nia_remove_modifier_response.to_pb();

                response.set_remove_modifier_response(remove_modifier);
            }
            NiaResponse::GetDefinedActions(get_defined_actions_response) => {
                let get_defined_actions = get_defined_actions_response.to_pb();

                response.set_get_defined_actions_response(get_defined_actions);
            }
            NiaResponse::DefineAction(define_action_response) => {
                let define_action = define_action_response.to_pb();

                response.set_define_action_response(define_action);
            }
            NiaResponse::RemoveAction(remove_action_response) => {
                let remove_action = remove_action_response.to_pb();

                response.set_remove_action_response(remove_action);
            }
        }

        response
    }

    fn from_pb(object_pb: Response) -> NiaServerResult<NiaResponse> {
        unimplemented!()
    }
}
