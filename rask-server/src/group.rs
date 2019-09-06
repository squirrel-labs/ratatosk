pub type GroupId = u32;

pub trait Group {
    fn id(&self) -> GroupId;
    fn group_type(&self) -> String;
    fn name(&self) -> String;

    fn run(&mut self);

    fn add_client(&mut self) -> Result<(), crate::error::ServerError>;
}
