use std::sync::MutexGuard;

use nia_interpreter_core::{EventLoopHandle, NiaStopListeningCommand};
use nia_protocol_rust::Response;

use crate::error::NiaServerError;
use crate::error::NiaServerResult;

use crate::protocol::NiaGetDefinedActionsResponse;
use crate::protocol::NiaGetDefinedMappingsRequest;
use crate::protocol::NiaGetDefinedMappingsResponse;
use crate::protocol::NiaGetDefinedModifiersResponse;
use crate::protocol::NiaGetDevicesResponse;
use crate::protocol::NiaHandshakeResponse;
use crate::protocol::NiaRemoveActionResponse;
use crate::protocol::NiaRemoveDeviceByNameResponse;
use crate::protocol::NiaRemoveDeviceByPathResponse;
use crate::protocol::NiaRemoveMappingResponse;
use crate::protocol::NiaRemoveModifierResponse;
use crate::protocol::NiaRequest;
use crate::protocol::Serializable;
use crate::protocol::{NiaChangeMappingResponse, NiaDefineActionResponse};
use crate::protocol::{NiaDefineDeviceResponse, NiaStartListeningResponse};
use crate::protocol::{NiaDefineMappingResponse, NiaStopListeningResponse};
use crate::protocol::{NiaDefineModifierResponse, NiaIsListeningResponse};
use crate::protocol::{NiaExecuteCodeResponse, NiaRemoveDeviceByIdResponse};
use crate::server::Server;

#[derive(Debug, Clone)]
pub enum NiaResponse {
    Handshake(NiaHandshakeResponse),
    GetDevices(NiaGetDevicesResponse),
    ExecuteCode(NiaExecuteCodeResponse),
    DefineKeyboard(NiaDefineDeviceResponse),
    RemoveKeyboardByPath(NiaRemoveDeviceByPathResponse),
    RemoveKeyboardByName(NiaRemoveDeviceByNameResponse),
    RemoveKeyboardById(NiaRemoveDeviceByIdResponse),
    GetDefinedModifiers(NiaGetDefinedModifiersResponse),
    DefineModifier(NiaDefineModifierResponse),
    RemoveModifier(NiaRemoveModifierResponse),
    GetDefinedActions(NiaGetDefinedActionsResponse),
    DefineAction(NiaDefineActionResponse),
    RemoveAction(NiaRemoveActionResponse),
    GetDefinedMappings(NiaGetDefinedMappingsResponse),
    DefineMapping(NiaDefineMappingResponse),
    ChangeMapping(NiaChangeMappingResponse),
    RemoveMapping(NiaRemoveMappingResponse),
    IsListening(NiaIsListeningResponse),
    StartListening(NiaStartListeningResponse),
    StopListening(NiaStopListeningResponse),
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
            NiaRequest::RemoveDeviceById(nia_remove_keyboard_by_id_request) => {
                let nia_remove_keyboard_by_id_response =
                    NiaRemoveDeviceByIdResponse::from(
                        server,
                        nia_remove_keyboard_by_id_request,
                        event_loop_handle,
                    );

                NiaResponse::RemoveKeyboardById(
                    nia_remove_keyboard_by_id_response,
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
            NiaRequest::GetDefinedMappings(
                nia_get_defined_mappings_request,
            ) => {
                let nia_get_defined_mappings_response =
                    NiaGetDefinedMappingsResponse::from(
                        nia_get_defined_mappings_request,
                        event_loop_handle,
                    );

                NiaResponse::GetDefinedMappings(
                    nia_get_defined_mappings_response,
                )
            }
            NiaRequest::DefineMapping(nia_define_mapping_request) => {
                let nia_define_mapping_response =
                    NiaDefineMappingResponse::from(
                        nia_define_mapping_request,
                        event_loop_handle,
                    );

                NiaResponse::DefineMapping(nia_define_mapping_response)
            }
            NiaRequest::ChangeMapping(nia_change_mapping_request) => {
                let nia_change_mapping_response =
                    NiaChangeMappingResponse::from(
                        nia_change_mapping_request,
                        event_loop_handle,
                    );

                NiaResponse::ChangeMapping(nia_change_mapping_response)
            }
            NiaRequest::RemoveMapping(nia_remove_mapping_request) => {
                let nia_remove_mapping_response =
                    NiaRemoveMappingResponse::from(
                        nia_remove_mapping_request,
                        event_loop_handle,
                    );
                NiaResponse::RemoveMapping(nia_remove_mapping_response)
            }
            NiaRequest::IsListening(nia_is_listening_request) => {
                let nia_is_listening_response = NiaIsListeningResponse::from(
                    nia_is_listening_request,
                    event_loop_handle,
                );
                NiaResponse::IsListening(nia_is_listening_response)
            }
            NiaRequest::StartListening(nia_start_listening_request) => {
                let nia_start_listening_response =
                    NiaStartListeningResponse::from(
                        nia_start_listening_request,
                        event_loop_handle,
                    );
                NiaResponse::StartListening(nia_start_listening_response)
            }
            NiaRequest::StopListening(nia_stop_listening_request) => {
                let nia_stop_listening_response =
                    NiaStopListeningResponse::from(
                        nia_stop_listening_request,
                        event_loop_handle,
                    );
                NiaResponse::StopListening(nia_stop_listening_response)
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
            NiaResponse::RemoveKeyboardById(
                nia_remove_keyboard_by_id_response,
            ) => {
                let remove_keyboard_by_id =
                    nia_remove_keyboard_by_id_response.to_pb();

                response
                    .set_remove_device_by_id_response(remove_keyboard_by_id);
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
            NiaResponse::GetDefinedMappings(get_defined_mappings_response) => {
                let get_defined_mappings =
                    get_defined_mappings_response.to_pb();

                response
                    .set_get_defined_mappings_response(get_defined_mappings);
            }
            NiaResponse::DefineMapping(define_mapping_response) => {
                let define_mapping = define_mapping_response.to_pb();

                response.set_define_mapping_response(define_mapping);
            }
            NiaResponse::ChangeMapping(change_mapping_response) => {
                let change_mapping = change_mapping_response.to_pb();

                response.set_change_mapping_response(change_mapping);
            }
            NiaResponse::RemoveMapping(remove_mapping_response) => {
                let remove_mapping = remove_mapping_response.to_pb();

                response.set_remove_mapping_response(remove_mapping);
            }
            NiaResponse::IsListening(is_listening_response) => {
                let is_listening = is_listening_response.to_pb();

                response.set_is_listening_response(is_listening);
            }
            NiaResponse::StartListening(start_listening_response) => {
                let start_listening = start_listening_response.to_pb();

                response.set_start_listening_response(start_listening);
            }
            NiaResponse::StopListening(stop_listening_response) => {
                let stop_listening = stop_listening_response.to_pb();

                response.set_stop_listening_response(stop_listening);
            }
        }

        response
    }

    fn from_pb(object_pb: Response) -> NiaServerResult<NiaResponse> {
        unimplemented!()
    }
}
