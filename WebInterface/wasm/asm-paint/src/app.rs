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

    pub fn run(&mut self) {
        let mut canvas = self.site.create_canvas().unwrap();
        canvas.init().unwrap();
        info!("canvas initialisation was succuessfull");
    }
}
