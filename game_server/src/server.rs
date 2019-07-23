use crate::backend_connection::BackendConnection;
use crate::lobby::Lobby;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
};
use websocket::{
    client::sync::Client,
    receiver, sender,
    server::{sync::AcceptResult, NoTlsAcceptor},
    stream::sync::Splittable,
    sync::Server,
    OwnedMessage,
};

pub type ClientReceiver = receiver::Reader<<TcpStream as Splittable>::Reader>;
pub type ClientSender = sender::Writer<<TcpStream as Splittable>::Writer>;

const PROTOCOL: &str = "tuesday";

pub type Token = u32;
pub type UserId = u32;

#[derive(Debug)]
pub enum GameServerError {
    BindError(std::io::Error),
    HandshakeRequestError,
    InvalidProtocolError,
    AcceptError(std::io::Error),
    GroupError(String),
    GroupCreationError(String),
}

impl std::fmt::Display for GameServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            GameServerError::BindError(e) => write!(f, "BindError: {}", e),
            GameServerError::HandshakeRequestError => write!(f, "HandshakeRequestError"),
            GameServerError::InvalidProtocolError => write!(f, "InvalidProtocolError"),
            GameServerError::AcceptError(e) => write!(f, "AcceptError: {}", e),
            GameServerError::GroupError(e) => write!(f, "GroupError: {}", e),
            GameServerError::GroupCreationError(e) => write!(f, "GroupCreationError: {}", e),
        }
    }
}

pub struct GameServer {
    addr: SocketAddr,
    lobby: Lobby,
    backend: BackendConnection,
}

pub struct GameClient {
    addr: SocketAddr,
    client: Client<TcpStream>,
}

impl GameClient {
    fn from_raw(client: Client<TcpStream>) -> Result<Self, ()> {
        let addr = client.peer_addr().map_err(|_| ())?;
        info!("got a client connection from: {}", addr);
        Ok(GameClient { addr, client })
    }

    fn require_token(&mut self) -> Option<Token> {
        let message = self.client.recv_message().ok()?;
        if let OwnedMessage::Text(text) = message {
            text.parse().ok()
        } else {
            None
        }
    }

    fn host_name(&self) -> SocketAddr {
        self.addr
    }

    pub fn split(self) -> (ClientSender, ClientReceiver) {
        let (rec, sen) = self.client.split().unwrap();
        (sen, rec)
    }
}

type ClientConnection = Result<GameClient, GameServerError>;

impl GameServer {
    pub fn new<T: ToSocketAddrs>(addr: T) -> Self {
        let addr = addr.to_socket_addrs().unwrap().next().unwrap();
        debug!("ws address: {}", addr);
        info!("create lobby");
        let lobby = Lobby::new();
        let backend = BackendConnection::new("https://kobert.dev");
        info!("got a C# backend connection");
        GameServer {
            addr,
            lobby: lobby,
            backend: backend,
        }
    }

    pub fn run(&mut self) -> Result<(), GameServerError> {
        let reader = self.read_clients();
        loop {
            let connection = reader.recv().unwrap()?;
            self.add_client(connection);
        }
    }

    fn add_client(&mut self, mut client: GameClient) {
        let token = client.require_token();
        if let Some(token) = token {
            let result = self.backend.validate_token(&token);
            match result {
                Err(err) => warn!("client's token {} is not valid: '{:?}'", token, err),
                Ok(result) => {
                    debug!("client validation was successfull");
                    let user_id = result.user_id;
                    let group_id = result.group_id;
                    let group_type = result.group_type;
                    let group_name = result.group_name;
                    debug!(
                        "add client: (id:{}, token:{}, host:{}) to \"{}\"",
                        user_id,
                        token,
                        client.host_name(),
                        group_name
                    );
                    //clients.lock().unwrap().insert(token, client);
                    self.lobby
                        .add_client(&group_type, group_id, &group_name, user_id, client)
                        .unwrap_or_else(|e| warn!("failed to add client: {}", e));
                }
            }
        } else {
            warn!("client sent invalid token");
        }
    }

    fn read_clients(&self) -> Receiver<ClientConnection> {
        let (sen, rec): (Sender<ClientConnection>, Receiver<ClientConnection>) = mpsc::channel();
        let addr = self.addr;
        std::thread::spawn(move || match Self::handle_requests(addr, &sen) {
            Err(e) => sen.send(Err(e)).unwrap(),
            _ => (),
        });
        rec
    }

    fn handle_requests(
        addr: SocketAddr,
        sen: &Sender<ClientConnection>,
    ) -> Result<(), GameServerError> {
        let server = match Server::<NoTlsAcceptor>::bind(addr) {
            Ok(v) => v,
            Err(e) => {
                error!("websocket binding error");
                Err(GameServerError::BindError(e))?
            }
        };
        info!("webserver is being launched");
        for req in server {
            sen.send(Ok(Self::handle_request(req)?)).unwrap();
        }
        info!("webserver is being shut down");
        Ok(())
    }

    fn handle_request(req: AcceptResult<TcpStream>) -> ClientConnection {
        match req {
            Ok(req) => {
                if !req.protocols().contains(&PROTOCOL.to_string()) {
                    warn!("a client tried to connect without {} protocol", PROTOCOL);
                    req.reject().unwrap();
                    Err(GameServerError::InvalidProtocolError)
                } else {
                    match req.use_protocol(PROTOCOL).accept() {
                        Ok(client) => match GameClient::from_raw(client) {
                            Ok(client) => Ok(client),
                            Err(_) => {
                                error!("could not create a client");
                                Err(GameServerError::HandshakeRequestError)
                            }
                        },
                        Err((_, e)) => {
                            warn!("client handshake failed");
                            Err(GameServerError::AcceptError(e))
                        }
                    }
                }
            }
            Err(_) => {
                warn!("invalid client request");
                Err(GameServerError::HandshakeRequestError)
            }
        }
    }
}
