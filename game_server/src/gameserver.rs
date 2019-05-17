use websocket::{OwnedMessage, sync::Server, server::NoTlsAcceptor};
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

#[derive(Debug)]
pub enum GameServerError {
    BindError(std::io::Error),
}

type ClientConnection = Result<(), GameServerError>;

pub struct GameServer {
    addr: SocketAddr,
    rec: Receiver<ClientConnection>,
}

impl GameServer {
    pub fn new<T: ToSocketAddrs>(addr: T) -> Self {
        let (s, r): (Sender<ClientConnection>, Receiver<ClientConnection>)
                     = mpsc::channel();
        let addr = addr.to_socket_addrs().unwrap().next().unwrap();
        debug!("ws address: {}", addr);
        std::thread::spawn(move || {
            let server = match Server::<NoTlsAcceptor>::bind(addr) {
                Ok(v) => v,
                Err(e) => {
                    s.send(Err(GameServerError::BindError(e))).unwrap();
                    return;
                },
            };
            info!("webserver is being launched");
            for req in server {
                //println!("{:?}", req);
                println!("gotcha");
            }
            info!("webserver is being shut down");
        });
        GameServer {
            addr,
            rec: r,
        }
    }
}
