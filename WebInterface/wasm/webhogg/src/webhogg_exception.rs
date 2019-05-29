use std::error::Error;

#[derive(Debug)]
pub enum WebhoggException {
    DomError(String),
    WebGlContextError(String),
}

impl WebhoggException {
    fn err_name(&self) -> &str {
        match self {
            WebhoggException::DomError(_) => "DomError",
            WebhoggException::WebGlContextError(_) => "WebGlContextError",
        }
    }
}

impl Error for WebhoggException {
    fn description(&self) -> &str {
        match self {
            WebhoggException::DomError(desc) => desc,
            WebhoggException::WebGlContextError(desc) => desc,
        }
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
        write!(f, "WebhoggException::{} {}", self.err_name(), self.description())
    }
}
