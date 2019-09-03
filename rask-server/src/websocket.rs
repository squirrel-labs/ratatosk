use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc, Mutex};

use ws::{listen, CloseCode, Message, Sender, Handler, Handshake, Result, Request, Response};

use crate::error::ServerError;
// WebSocket connection handler for the server connection
pub struct Server {
    ws: Sender,
    count: Arc<Mutex<u32>>,
}

impl Handler for Server {
    // `on_open` will be called only after the WebSocket handshake is successful
    // so at this point we know that the connection is ready to send/receive messages.
    // We ignore the `Handshake` for now, but you could also use this method to setup
    // Handler state or reject the connection based on the details of the Request
    // or Response, such as by checking cookies or Auth headers.
    fn on_open(&mut self, _: Handshake) -> Result<()> {
		Ok((*self.count.lock().unwrap()) += 1)
    }

	fn on_request(&mut self, req: &Request) -> Result<Response> {
		let mut res = Response::from_request(req).unwrap();
		//TODO fix 2 unwraps
        if req.protocols().unwrap().iter().find(|&&pro| pro.contains("tuesday")).is_some() {
                res.set_protocol("tuesday")
        }
        Ok(res)
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        info!("Server got message '{}'. ", msg);

        // echo it back
        self.ws.send(msg)
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        //self.ws.shutdown().unwrap()

        //The connection is going down, so we need to decrement the count
		(*self.count.lock().unwrap()) -= 1
    }
}

impl Server {
    pub fn run(address: &str, port: &str) -> core::result::Result<JoinHandle<()>, ServerError> {
        let count = Arc::new(Mutex::new(0));
        let url = format!("{}:{}", address, port);
        thread::Builder::new().name("server".to_owned()).spawn(move || {
            listen(url, |out| {
                Server {
                    ws: out,
					count: count.clone(),
                }
            }).unwrap()
        }).map_err(|e| ServerError::WebsocketCreation(e))
    }
}
