pub type GroupId = u32;

pub trait Group {
    fn id(&self) -> GroupId;
    fn name(&self) -> String;

    fn run(&self);
}
