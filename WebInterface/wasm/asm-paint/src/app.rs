use crate::site::Site;

pub struct App {
    site: Site,
}

impl App {
    pub fn new() -> Option<Self> {
        Some(Self {
            site: Site::from_current()?,
        })
    }

    pub fn run(&self) {
    }
}
