use websocket::{OwnedMessage,
    sync::Server,
    client::sync::Client,
    server::{NoTlsAcceptor, InvalidConnection,
        sync::AcceptResult}};
use std::net::{SocketAddr, ToSocketAddrs, TcpStream};
use std::sync::{mpsc,
                mpsc::{Sender, Receiver},
                Arc, Mutex};
use std::collections::HashMap;
use super::lobby::Lobby;
use super::backend_connection::BackendConnection;

const PROTOCOL: &str = "tuesday";

pub type Token = u32;

#[derive(Debug)]
pub enum GameServerError {
    BindError(std::io::Error),
    HandshakeRequestError,
    InvalidProtocolError,
    AcceptError(std::io::Error)
}

pub struct GameServer {
    addr: SocketAddr,
    lobby: Lobby,
    backend: Arc<Mutex<BackendConnection>>,
    clients: Arc<Mutex<HashMap<Token, GameClient>>>,
}

pub struct GameClient {
    addr: SocketAddr,
    client: Client<TcpStream>,
}

impl GameClient {
    fn from_raw(client: Client<TcpStream>) -> Result<Self, ()> {
        let addr = client.peer_addr().map_err(|_| ())?;
        info!("got a client connection from: {}", addr);
        Ok(GameClient {
            addr,
            client,
        })
    }

    fn require_token(&mut self) -> Option<Token> {
        let message = self.client
                 .recv_message()
                 .ok()?;
        if let OwnedMessage::Text(text) = message {
            text.parse().ok()
        } else {
            None
        }
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
            lobby,
            backend: Arc::new(Mutex::new(backend)),
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn run(&self) -> Result<(), GameServerError> {
        let reader = self.read_clients();
        loop {
            let mut connection = reader.recv().unwrap()?;
            self.add_client(connection);
        }
        Ok(())
    }

    fn add_client(&self, mut client: GameClient) {
        let clients = Arc::clone(&self.clients);
        let backend = Arc::clone(&self.backend);
        std::thread::spawn(move || {
            let token = client.require_token();
            if let Some(token) = token {
                println!("Token: {}", token);
                let locked_backend = backend.lock().unwrap();
                let result = locked_backend.validate_token(&token);
                if let Err(err) = result {
                    warn!("token {} is invalid: '{:?}'", token, err);
                } else {
                    clients.lock().unwrap().insert(token, client);
                }
            } else {
                warn!("client sent invalid token");
            }
        });
    }

    fn read_clients(&self) -> Receiver<ClientConnection> {
        let (s, r): (Sender<ClientConnection>, Receiver<ClientConnection>)
                     = mpsc::channel();
        let addr = self.addr;
        std::thread::spawn(move || {
            let result = Self::handle_requests(addr, &s).or_else(|e| s.send(Err(e)));
        });
        r
    }

    fn handle_requests(addr: SocketAddr, s: &Sender<ClientConnection>) -> Result<(), GameServerError> {
        let server = match Server::<NoTlsAcceptor>::bind(addr) {
            Ok(v) => v,
            Err(e) => {
                error!("websocket binding error");
                Err(GameServerError::BindError(e))?
            },
        };
        info!("webserver is being launched");
        for req in server {
            s.send(Ok(Self::handle_request(req)?)).unwrap();
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
                        Ok(client) => {
                            match GameClient::from_raw(client) {
                                Ok(client) => Ok(client),
                                Err(_) => {
                                    error!("could not create a client");
                                    Err(GameServerError::HandshakeRequestError)
                                }
                            }
                        },
                        Err((_, e)) => {
                            warn!("client handshake failed");
                            Err(GameServerError::AcceptError(e))
                        }
                    }
                }
            },
            Err(e) => {
                warn!("invalid client request");
                Err(GameServerError::HandshakeRequestError)
            }
        }
    }
}
