use crate::backend_connection::TokenResponse;
use crate::error::ServerError;
use crate::group::Group;
use std::convert::TryInto;
use std::sync::mpsc;
use ws::Sender;

pub type GroupId = u32;

pub trait Game {
    fn run(&self);
}

pub struct RaskGame<'a> {
    group: &'a Group,
}

impl Game for RaskGame<'_> {
    fn run(&self) {}
}

impl<'a> RaskGame<'a> {
    pub fn new(group: &'a Group) -> Self {
        Self { group }
    }
}
