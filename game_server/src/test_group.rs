use super::group::{Group, GroupId};

pub struct TestGroup {
    id: GroupId,
    name: String,
}

impl Group for TestGroup {
    fn id(&self) -> GroupId {
        self.id
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn run(&self) {
        let id = self.id;
        let name = self.name.to_owned();
        std::thread::spawn(move || /*loop { println!("> group nr.{} wishes you: '{}'", id, name) }*/());
    }
}

impl TestGroup {
    pub fn new(id: GroupId, name: String) -> Self {
        TestGroup { id, name }
    }
}
