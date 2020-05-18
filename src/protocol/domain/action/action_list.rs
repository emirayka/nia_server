use crate::error::NiaServerResult;
use crate::protocol::{NiaAction, Serializable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionList {
    actions: Vec<NiaAction>,
}

impl ActionList {
    pub fn new(actions: Vec<NiaAction>) -> ActionList {
        ActionList { actions }
    }

    pub fn get_actions(&self) -> &Vec<NiaAction> {
        &self.actions
    }
}

impl Serializable<ActionList, nia_protocol_rust::ActionList> for ActionList {
    fn to_pb(&self) -> nia_protocol_rust::ActionList {
        let actions_pb = self
            .actions
            .iter()
            .map(|action| action.to_pb())
            .collect::<Vec<nia_protocol_rust::Action>>();

        let mut action_list_pb = nia_protocol_rust::ActionList::new();

        action_list_pb.set_actions(protobuf::RepeatedField::from(actions_pb));

        action_list_pb
    }

    fn from_pb(
        object_pb: nia_protocol_rust::ActionList,
    ) -> NiaServerResult<ActionList> {
        let mut object_pb = object_pb;

        let mut actions = vec![];
        for action_pb in object_pb.take_actions().into_iter() {
            let action = NiaAction::from_pb(action_pb)?;

            actions.push(action)
        }

        Ok(ActionList::new(actions))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::protocol::{ActionExecuteOSCommand, ActionKeyRelease};

    #[test]
    fn serializes_and_deserializes_actions() {
        let expected = ActionList::new(vec![
            NiaAction::new("arst", ActionKeyRelease::new(2).into()),
            NiaAction::new(
                "arst-2",
                ActionExecuteOSCommand::new("arst-2").into(),
            ),
        ]);

        let bytes = expected.to_bytes().unwrap();
        let actual = ActionList::from_bytes(bytes).unwrap();

        assert_eq!(expected, actual);
    }
}
