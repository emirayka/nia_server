use crate::protocol::*;

use crate::error::NiaServerError;
use crate::error::NiaServerResult;
use nia_protocol_rust::Request;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NiaRequest {
    Handshake(NiaHandshakeRequest),
    GetDevices(NiaGetDevicesRequest),
    ExecuteCode(NiaExecuteCodeRequest),
    DefineDevice(NiaDefineDeviceRequest),
    RemoveDeviceByPath(NiaRemoveDeviceByPathRequest),
    RemoveDeviceByName(NiaRemoveDeviceByNameRequest),
    GetDefinedModifiers(NiaGetDefinedModifiersRequest),
    DefineModifier(NiaDefineModifierRequest),
    RemoveModifier(NiaRemoveModifierRequest),
    GetDefinedActions(NiaGetDefinedActionsRequest),
    DefineAction(NiaDefineActionRequest),
    RemoveAction(NiaRemoveActionRequest),
    GetDefinedMappings(NiaGetDefinedMappingsRequest),
    DefineMapping(NiaDefineMappingRequest),
    ChangeMapping(NiaChangeMappingRequest),
    RemoveMapping(NiaRemoveMappingRequest),
    IsListening(NiaIsListeningRequest),
    StartListening(NiaStartListeningRequest),
    StopListening(NiaStopListeningRequest),
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
make_from_implementation!(NiaExecuteCodeRequest, NiaRequest::ExecuteCode);
make_from_implementation!(NiaDefineDeviceRequest, NiaRequest::DefineDevice);
make_from_implementation!(
    NiaRemoveDeviceByPathRequest,
    NiaRequest::RemoveDeviceByPath
);
make_from_implementation!(
    NiaRemoveDeviceByNameRequest,
    NiaRequest::RemoveDeviceByName
);
make_from_implementation!(
    NiaGetDefinedModifiersRequest,
    NiaRequest::GetDefinedModifiers
);
make_from_implementation!(NiaDefineModifierRequest, NiaRequest::DefineModifier);
make_from_implementation!(NiaRemoveModifierRequest, NiaRequest::RemoveModifier);
make_from_implementation!(
    NiaGetDefinedActionsRequest,
    NiaRequest::GetDefinedActions
);
make_from_implementation!(NiaDefineActionRequest, NiaRequest::DefineAction);
make_from_implementation!(NiaRemoveActionRequest, NiaRequest::RemoveAction);
make_from_implementation!(
    NiaGetDefinedMappingsRequest,
    NiaRequest::GetDefinedMappings
);
make_from_implementation!(NiaDefineMappingRequest, NiaRequest::DefineMapping);
make_from_implementation!(NiaChangeMappingRequest, NiaRequest::ChangeMapping);
make_from_implementation!(NiaRemoveMappingRequest, NiaRequest::RemoveMapping);
make_from_implementation!(NiaIsListeningRequest, NiaRequest::IsListening);
make_from_implementation!(NiaStartListeningRequest, NiaRequest::StartListening);
make_from_implementation!(NiaStopListeningRequest, NiaRequest::StopListening);

impl Serializable<NiaRequest, nia_protocol_rust::Request> for NiaRequest {
    fn to_pb(&self) -> Request {
        let mut request_pb = nia_protocol_rust::Request::new();

        match self {
            NiaRequest::Handshake(handshake_request) => {
                request_pb.set_handshake_request(handshake_request.to_pb());
            }
            NiaRequest::GetDevices(get_devices_request) => {
                request_pb.set_get_devices_request(get_devices_request.to_pb());
            }
            NiaRequest::ExecuteCode(execute_code_request) => request_pb
                .set_execute_code_request(execute_code_request.to_pb()),
            NiaRequest::DefineDevice(define_device_request) => request_pb
                .set_define_device_request(define_device_request.to_pb()),
            NiaRequest::RemoveDeviceByPath(remove_device_by_path_request) => {
                request_pb.set_remove_device_by_path_request(
                    remove_device_by_path_request.to_pb(),
                )
            }
            NiaRequest::RemoveDeviceByName(remove_device_by_name_request) => {
                request_pb.set_remove_device_by_name_request(
                    remove_device_by_name_request.to_pb(),
                )
            }
            NiaRequest::GetDefinedModifiers(get_defined_modifiers_request) => {
                request_pb.set_get_defined_modifiers_request(
                    get_defined_modifiers_request.to_pb(),
                )
            }
            NiaRequest::DefineModifier(define_modifier_request) => request_pb
                .set_define_modifier_request(define_modifier_request.to_pb()),
            NiaRequest::RemoveModifier(remove_modifier_request) => request_pb
                .set_remove_modifier_request(remove_modifier_request.to_pb()),
            NiaRequest::GetDefinedActions(get_defined_actions_request) => {
                request_pb.set_get_defined_actions_request(
                    get_defined_actions_request.to_pb(),
                )
            }
            NiaRequest::DefineAction(define_action_request) => request_pb
                .set_define_action_request(define_action_request.to_pb()),
            NiaRequest::RemoveAction(remove_action_request) => request_pb
                .set_remove_action_request(remove_action_request.to_pb()),
            NiaRequest::GetDefinedMappings(get_defined_mappings_request) => {
                request_pb.set_get_defined_mappings_request(
                    get_defined_mappings_request.to_pb(),
                )
            }
            NiaRequest::DefineMapping(define_mapping_request) => request_pb
                .set_define_mapping_request(define_mapping_request.to_pb()),
            NiaRequest::ChangeMapping(change_mapping_request) => request_pb
                .set_change_mapping_request(change_mapping_request.to_pb()),
            NiaRequest::RemoveMapping(remove_mapping_request) => request_pb
                .set_remove_mapping_request(remove_mapping_request.to_pb()),
            NiaRequest::IsListening(is_listening_request) => request_pb
                .set_is_listening_request(is_listening_request.to_pb()),
            NiaRequest::StartListening(start_listening_request) => request_pb
                .set_start_listening_request(start_listening_request.to_pb()),
            NiaRequest::StopListening(stop_listening_request) => request_pb
                .set_stop_listening_request(stop_listening_request.to_pb()),
        }

        request_pb
    }

    fn from_pb(object_pb: Request) -> NiaServerResult<NiaRequest> {
        let mut request_pb = object_pb;

        let result = if request_pb.has_handshake_request() {
            let handshake_request = NiaHandshakeRequest::from_pb(
                request_pb.take_handshake_request(),
            )?;
            NiaRequest::Handshake(handshake_request)
        } else if request_pb.has_get_devices_request() {
            let get_devices_request = NiaGetDevicesRequest::from_pb(
                request_pb.take_get_devices_request(),
            )?;
            NiaRequest::GetDevices(get_devices_request)
        } else if request_pb.has_execute_code_request() {
            let execute_code_request = NiaExecuteCodeRequest::from_pb(
                request_pb.take_execute_code_request(),
            )?;
            NiaRequest::ExecuteCode(execute_code_request)
        } else if request_pb.has_define_device_request() {
            let define_device_request = NiaDefineDeviceRequest::from_pb(
                request_pb.take_define_device_request(),
            )?;
            NiaRequest::DefineDevice(define_device_request)
        } else if request_pb.has_remove_device_by_path_request() {
            let remove_device_by_path_request =
                NiaRemoveDeviceByPathRequest::from_pb(
                    request_pb.take_remove_device_by_path_request(),
                )?;
            NiaRequest::RemoveDeviceByPath(remove_device_by_path_request)
        } else if request_pb.has_remove_device_by_name_request() {
            let remove_device_by_name_request =
                NiaRemoveDeviceByNameRequest::from_pb(
                    request_pb.take_remove_device_by_name_request(),
                )?;
            NiaRequest::RemoveDeviceByName(remove_device_by_name_request)
        } else if request_pb.has_get_defined_modifiers_request() {
            let get_defined_modifiers_request =
                NiaGetDefinedModifiersRequest::from_pb(
                    request_pb.take_get_defined_modifiers_request(),
                )?;
            NiaRequest::GetDefinedModifiers(get_defined_modifiers_request)
        } else if request_pb.has_define_modifier_request() {
            let define_modifier_request = NiaDefineModifierRequest::from_pb(
                request_pb.take_define_modifier_request(),
            )?;
            NiaRequest::DefineModifier(define_modifier_request)
        } else if request_pb.has_remove_modifier_request() {
            let remove_modifier_request = NiaRemoveModifierRequest::from_pb(
                request_pb.take_remove_modifier_request(),
            )?;
            NiaRequest::RemoveModifier(remove_modifier_request)
        } else if request_pb.has_get_defined_actions_request() {
            let get_defined_actions_request =
                NiaGetDefinedActionsRequest::from_pb(
                    request_pb.take_get_defined_actions_request(),
                )?;
            NiaRequest::GetDefinedActions(get_defined_actions_request)
        } else if request_pb.has_define_action_request() {
            let define_action_request = NiaDefineActionRequest::from_pb(
                request_pb.take_define_action_request(),
            )?;
            NiaRequest::DefineAction(define_action_request)
        } else if request_pb.has_remove_action_request() {
            let remove_action_request = NiaRemoveActionRequest::from_pb(
                request_pb.take_remove_action_request(),
            )?;
            NiaRequest::RemoveAction(remove_action_request)
        } else if request_pb.has_get_defined_mappings_request() {
            let get_defined_mappings_request =
                NiaGetDefinedMappingsRequest::from_pb(
                    request_pb.take_get_defined_mappings_request(),
                )?;
            NiaRequest::GetDefinedMappings(get_defined_mappings_request)
        } else if request_pb.has_define_mapping_request() {
            let define_mapping_request = NiaDefineMappingRequest::from_pb(
                request_pb.take_define_mapping_request(),
            )?;
            NiaRequest::DefineMapping(define_mapping_request)
        } else if request_pb.has_change_mapping_request() {
            let change_mapping_request = NiaChangeMappingRequest::from_pb(
                request_pb.take_change_mapping_request(),
            )?;
            NiaRequest::ChangeMapping(change_mapping_request)
        } else if request_pb.has_remove_mapping_request() {
            let remove_mapping_request = NiaRemoveMappingRequest::from_pb(
                request_pb.take_remove_mapping_request(),
            )?;
            NiaRequest::RemoveMapping(remove_mapping_request)
        } else if request_pb.has_is_listening_request() {
            let is_listening_request = NiaIsListeningRequest::from_pb(
                request_pb.take_is_listening_request(),
            )?;
            NiaRequest::IsListening(is_listening_request)
        } else if request_pb.has_start_listening_request() {
            let start_listening_request = NiaStartListeningRequest::from_pb(
                request_pb.take_start_listening_request(),
            )?;
            NiaRequest::StartListening(start_listening_request)
        } else if request_pb.has_stop_listening_request() {
            let stop_listening_request = NiaStopListeningRequest::from_pb(
                request_pb.take_stop_listening_request(),
            )?;
            NiaRequest::StopListening(stop_listening_request)
        } else {
            return NiaServerError::deserialization_error("Unknown request.")
                .into();
        };

        Ok(result)
    }
}
