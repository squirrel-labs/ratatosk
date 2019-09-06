use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::collections::HashMap;
use crate::backend_connection::*;
use crate::group::*;

use ws::{listen, CloseCode, Handler, Handshake, Message, Request, Response, Result, Sender};

use crate::error::ServerError;

const PROTOCOL: &str = "tuesday";
// WebSocket connection handler for the server connection
pub struct Server<'a> {
    ws: Sender,
    group: mpsc::Sender<&'a [u8]>,
    groups: Arc<Mutex<HashMap<u32, Group<'a>>>>,
}

impl Handler for Server<'_> {
    // called when the socket connection is created
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // keep track of the number of Clients -> could be a vec of lobbys as well
        //Ok((*self.groups.lock().unwrap()) += 1)
        Ok(())
    }

    // low-level handling of requests
    fn on_request(&mut self, req: &Request) -> Result<Response> {
        let res = handshake(req);
        match res {
            (res, Ok(token)) => {
                info!("recived token: {}", token);
                match crate::backend_connection::verify_token(token) {
                    Ok(response) => {self.handle_token(response); Ok(res)},
                    Err(e) => Ok(fail_response(res, format!("{}", e).as_str())),
                }
            },
            (res, Err(err)) => {
                warn!("Client {:?}: {:?}", req.client_addr(), err);
                Ok(res)
            }
        }
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        info!("Server got message '{}'. ", msg);

        // echo it back
        self.ws.send(format!("{} +1", msg))
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        //self.ws.shutdown().unwrap()
        //The connection is going down, so we need to decrement the count
        //(*self.count.lock().unwrap()) -= 1
    }
}

impl Server<'_> {
    pub fn run<'a>(address: &str, port: &str) -> core::result::Result<JoinHandle<()>, ServerError> {
        let count = Arc::new(Mutex::new(HashMap<u32, Group<'a>>));
        let (sender, _) = mpsc::channel();
        let url = format!("{}:{}", address, port);
        thread::Builder::new()
            .name("server".to_owned())
            .spawn(move || {
                listen(url, |out| Server {
                    ws: out,
                    group: sender,
                    groups: count,
                })
                .unwrap()
            })
            .map_err(|e| ServerError::WebsocketCreation(e))
    }

    fn handle_token(&mut self, response: TokenResponse) {
        self.ws.send(format!("{:?})", response));
    }
}

fn handshake(req: &Request) -> (Response, core::result::Result<i32, ServerError>) {
    let mut res = Response::from_request(req).unwrap();
    // TODO fix 2 unwraps
    // Reject Clients that do not support the
    if let Ok(protocols) = req.protocols() {
        if protocols.iter().find(|&&pro| pro.contains(PROTOCOL)).is_some(){
            res.set_protocol(PROTOCOL)
        } else {
            return (fail_response(res, format!("does not support the {} protocol", PROTOCOL).as_str()),
            Err(ServerError::InvalidProtocol));
        }
        let token = protocols.iter().find(|&&pro| pro.starts_with("Token-"));
        match token {
            Some(token) => {
                let (_, token) = token.split_at(6);
                if let Ok(token) = token.parse::<i32>() {
                    return (res, Ok(token)) 
                } else {
                    return (fail_response(res, "token is no valid i32"), Err(ServerError::InvalidTokenFormat));
                }
            }
            None => {
                return (fail_response(res, "no token in protocols"), Err(ServerError::InvalidToken)); 
            }
        }
    } else {
        return (fail_response(res, "failed to retrive protocols"), Err(ServerError::InvalidProtocol));
    }
}

fn fail_response(mut res: Response, reason: &str) -> Response  {
    res.set_status(400);
    warn!("{}", reason);
    res
}

