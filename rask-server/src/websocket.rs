use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender as ThreadOut;

use ws::{listen, CloseCode, Message, Sender, Handler, Result};

use crate::error::ServerError;
// WebSocket connection handler for the server connection
pub struct Server {
    ws: Sender,
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        info!("Server got message '{}'. ", msg);

        // echo it back
        self.ws.send(msg)
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        self.ws.shutdown().unwrap()
    }
}

impl Server {
    pub fn run(address: &str, port: &str) -> core::result::Result<JoinHandle<()>, ServerError> {
        let url = format!("{}:{}", address, port);
        thread::Builder::new().name("server".to_owned()).spawn(move || {
            listen(url, |out| {
                Server {
                    ws: out,
                }
            }).unwrap()
        }).map_err(|e| ServerError::WebsocketCreation(e))
    }
}
