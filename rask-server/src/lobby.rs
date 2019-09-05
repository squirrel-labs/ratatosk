use std::sync::mpsc::{channel, Receiver, Sender};

struct Connection<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Connection<T> {
    fn new() -> (Self, Self) {
        let (tx1, rx1) = channel();
        let (tx2, rx2) = channel();
        (
            Self {
                sender: tx1,
                receiver: rx2,
            },
            Self {
                sender: tx2,
                receiver: rx1,
            },
        )
    }
}

struct Lobby {
    connection: Connection<String>,
}

struct Listener {
    connections: Vec<Connection<String>>,
}

impl Listener {
    /// Creates a new Listener with no lobbys.
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
        }
    }

    /// Creates a new Lobby that is listened to by the Listener.
    pub fn new_lobby(&mut self) -> Lobby {
        let (c1, c2) = Connection::new();
        self.connections.push(c1);
        Lobby { connection: c2 }
    }
}
