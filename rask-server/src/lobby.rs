use std::collections::HashMap;

use crate::error::ServerError;
use crate::group::{Group, GroupId};
use crate::rask_group::RaskGroup;
use crate::scribble_group::ScribbleGroup;

use crate::server::{GameClient, UserId};

pub struct Lobby {
    groups: HashMap<GroupId, Box<dyn Group>>,
}

#[allow(dead_code)]
impl Lobby {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
        }
    }

    fn generate_group(group_type: &str, id: GroupId, name: &str) -> Option<Box<dyn Group>> {
        match group_type {
            "scribble" => Some(Box::new(ScribbleGroup::new(id, name.to_string()))),
            "rask" => Some(Box::new(RaskGroup::new(id, name.to_string()))),
            _ => None,
        }
    }

    pub fn add_group(&mut self, group: Box<dyn Group>) {
        self.groups.insert(group.id(), group);
    }

    pub fn add_client(
        &mut self,
        group_type: &str,
        group_id: GroupId,
        group_name: &str,
        user_id: UserId,
        client: GameClient,
    ) -> Result<(), ServerError> {
        if !self.groups.contains_key(&group_id) {
            let mut group = Self::generate_group(group_type, group_id, group_name).ok_or(
                ServerError::GroupCreation(format!("failed to generate '{}' group", group_type)),
            )?;
            group.run();
            self.groups.insert(group_id, group);
        }
        let group = self.groups.get_mut(&group_id).unwrap();
        group.add_client(user_id, client)
    }

    pub fn iter<'b>(&'b self) -> GroupIterator<'b> {
        GroupIterator {
            groups: self.groups.values(),
        }
    }
}

#[allow(dead_code)]
pub struct GroupIterator<'a> {
    groups: std::collections::hash_map::Values<'a, GroupId, Box<dyn Group>>,
}

impl<'a> Iterator for GroupIterator<'a> {
    type Item = &'a Box<dyn Group>;

    fn next(&mut self) -> Option<Self::Item> {
        self.groups.next()
    }
}
