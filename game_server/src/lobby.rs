use std::collections::HashMap;

use super::group::{Group, GroupId};

pub struct Lobby {
    groups: HashMap<GroupId, Box<Group>>,
}

impl Lobby {
    pub fn new() -> Lobby {
        Self {
            groups: HashMap::new(),
        }
    }

    pub fn add_group(&mut self, group: Box<Group>) {
        self.groups.insert(group.id(), group);
    }

    pub fn iter<'a>(&'a self) -> GroupIterator<'a> {
        GroupIterator { groups: self.groups.values() }
    }
}

pub struct GroupIterator<'a> {
    groups: std::collections::hash_map::Values<'a, GroupId, Box<Group>>
}

impl<'a> Iterator for GroupIterator<'a> {
    type Item = &'a Box<Group>;

    fn next(&mut self) -> Option<Self::Item> {
        self.groups.next()
    }
}
