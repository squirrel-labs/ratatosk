use super::server::{UserId, GameClient};

pub type GroupId = u32;

pub trait Group {
    fn id(&self) -> GroupId;
    fn group_type(&self) -> String;
    fn name(&self) -> String;

    fn run(&mut self);

    fn add_client(&mut self, id: UserId, client: GameClient);
    fn get_client(&self, client_id: UserId) -> Option<&GameClient>;
}
