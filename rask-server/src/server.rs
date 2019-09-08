use crate::backend_connection::*;
use crate::group::*;
use crate::group::{GroupId, Message as GroupMessage};
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use ws::{listen, CloseCode, Handler, Handshake, Message, Request, Response, Result, Sender};

use crate::error::ServerError;

const PROTOCOL: &str = "tuesday";
// WebSocket connection handler for the server connection
pub struct Server {
    ws: Sender,
    group: mpsc::Sender<GroupMessage>,
    groups: Arc<Mutex<HashMap<u32, Group>>>,
    ip: String,
    id: GroupId,
}

impl Handler for Server {
    // called when the socket connection is created
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        if let Ok(Some(ip)) = handshake.remote_addr() {
            self.ip = ip;
        }
        Ok(())
    }

    // low-level handling of requests
    fn on_request(&mut self, req: &Request) -> Result<Response> {
        let res = handshake(req);
        match res {
            (mut res, Ok(token)) => {
                info!("recived token: {}", token);
                match crate::backend_connection::verify_token(token) {
                    Ok(response) => {
                        if let Err(e) = self.handle_token(response) {
                            res = fail_response(res, format!("{}", e).as_str());
                        }
                        Ok(res)
                    }
                    Err(e) => Ok(fail_response(res, format!("{}", e).as_str())),
                }
            }
            (res, Err(err)) => Ok(fail_response(
                res,
                format!("Client {:?}: {:?}", req.client_addr(), err).as_str(),
            )),
        }
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        info!("Server got message '{}'. ", msg);

        self.group
            .send((self.ip.clone(), Content::data(Box::new(msg.into_data()))));
        Ok(())
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        if let Ok(mut guard) = self.groups.lock() {
            if let Some(mut group) = guard.get_mut(&self.id) {
                group.remove_client(&self.ws);
            }
        }
    }
}

impl Server {
    pub fn run(address: &str, port: &str) -> core::result::Result<JoinHandle<()>, ServerError> {
        let count = Arc::new(Mutex::new(HashMap::new()));
        let (sender, _) = mpsc::channel();
        let url = format!("{}:{}", address, port);
        thread::Builder::new()
            .name("server".to_owned())
            .spawn(move || {
                listen(url, |out| Server {
                    ws: out,
                    group: sender.clone(),
                    groups: count.clone(),
                    ip: "No ip".to_owned(),
                    id: 0,
                })
                .unwrap()
            })
            .map_err(|e| ServerError::WebsocketCreation(e))
    }

    fn handle_token(&mut self, response: TokenResponse) -> core::result::Result<(), ServerError> {
        self.ws.send(format!("{:?})", response));
        match self.groups.lock() {
            Ok(mut guard) => {
                self.id = response.group_id;
                let group_type = response.group_type.clone();
                if !guard.contains_key(&response.group_id) {
                    if let Ok(group) = Group::new(response) {
                        guard.insert(group.id(), group);
                    } else {
                        let err = format!(
                            "The client requestet a game that is not implemented: {}",
                            group_type
                        );
                        return Err(ServerError::GroupCreation(err));
                        //self.ws.close_with_reason(CloseCode::Unsupported, err);
                    };
                }

                let group = guard.get_mut(&self.id).unwrap();
                group.add_client(self.ws.clone());
            }
            Err(e) => {
                return Err(ServerError::Group(format!(
                    "cold not add client {:?}  to group {}: {}",
                    self.ws, response.group_id, e
                )));
                //self.ws.close(CloseCode::Error);
            }
        }
        Ok(())
    }
}

fn handshake(req: &Request) -> (Response, core::result::Result<i32, ServerError>) {
    let mut res = Response::from_request(req).unwrap();
    if let Ok(protocols) = req.protocols() {
        if protocols
            .iter()
            .find(|&&pro| pro.contains(PROTOCOL))
            .is_some()
        {
            res.set_protocol(PROTOCOL)
        } else {
            return (
                fail_response(
                    res,
                    format!("does not support the {} protocol", PROTOCOL).as_str(),
                ),
                Err(ServerError::InvalidProtocol),
            );
        }
        let token = protocols.iter().find(|&&pro| pro.starts_with("Token-"));
        match token {
            Some(token) => {
                let (_, token) = token.split_at(6);
                if let Ok(token) = token.parse::<i32>() {
                    return (res, Ok(token));
                } else {
                    return (
                        fail_response(res, "token is no valid i32"),
                        Err(ServerError::InvalidTokenFormat),
                    );
                }
            }
            None => {
                return (
                    fail_response(res, "no token in protocols"),
                    Err(ServerError::InvalidToken(
                        "No Token was passed as a Protocol in the Sec-WebSocket-Protocol Header"
                            .to_string(),
                    )),
                );
            }
        }
    } else {
        return (
            fail_response(res, "failed to retrive protocols"),
            Err(ServerError::InvalidProtocol),
        );
    }
}

fn fail_response(mut res: Response, reason: &str) -> Response {
    res.set_status(400);
    res.set_reason(reason);
    warn!("{}", reason);
    res
}
