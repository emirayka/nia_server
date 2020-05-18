use crate::error::{from_protobuf_error, NiaServerError, NiaServerResult};
use crate::protocol::Serializable;
use protobuf::Message;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionExecuteOSCommand {
    os_command: String,
}

impl ActionExecuteOSCommand {
    pub fn new<S>(os_command: S) -> ActionExecuteOSCommand
    where
        S: Into<String>,
    {
        ActionExecuteOSCommand {
            os_command: os_command.into(),
        }
    }

    pub fn get_os_command(&self) -> &str {
        &self.os_command
    }
}

impl
    Serializable<
        ActionExecuteOSCommand,
        nia_protocol_rust::ActionExecuteOSCommand,
    > for ActionExecuteOSCommand
{
    fn to_pb(&self) -> nia_protocol_rust::ActionExecuteOSCommand {
        let mut action_execute_os_command_pb =
            nia_protocol_rust::ActionExecuteOSCommand::new();

        action_execute_os_command_pb
            .set_os_command(protobuf::Chars::from(self.os_command.clone()));

        action_execute_os_command_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ActionExecuteOSCommand,
    ) -> NiaServerResult<ActionExecuteOSCommand> {
        let action_execute_os_command = ActionExecuteOSCommand::new(
            String::from(object_pb.get_os_command()),
        );

        Ok(action_execute_os_command)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn serializable_and_deserializable() {
        let expected = "let a = 1;";

        let action_execute_os_command = ActionExecuteOSCommand::new(expected);
        let bytes = action_execute_os_command.to_bytes().unwrap();
        let action_execute_os_command =
            ActionExecuteOSCommand::from_bytes(bytes).unwrap();

        let result = action_execute_os_command.os_command;

        assert_eq!(expected, result)
    }
}
