pub type GroupId = u32;

pub struct Group {
    id: GroupId,
    name: String,
}

impl Group {
    pub(crate) fn new(id: GroupId, name: String) -> Group {
        Group { id, name }
    }

    pub(crate) fn get_id(&self) -> GroupId {
        self.id
    }

    pub fn run(&self) {
        let id = self.id;
        std::thread::spawn(move ||
            loop {println!("group id: {} meldet sich", id)});
    }
}
