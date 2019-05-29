use std::error::Error;

#[derive(Debug)]
pub enum WebhoggException {
}

impl Error for WebhoggException {
    fn description(&self) -> &str {
        "webhogg exception"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl std::fmt::Display for WebhoggException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "WebhoggException: {}", self.description())
    }
}
