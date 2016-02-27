use mio;
use mio::tcp::TcpListener;
use mio::{EventLoop, TcpStream, Token};

struct HttpListener {
    server: TcpListener,
    slab: Slab<Connection>,
}

const SERVER: Token = Token(0);

impl HttpListener {
    pub fn new(server: TcpListener) -> HttpListener {
        HttpListener {
            server: server,
            slab: Slab::new_starting_at(Token(1), 1024),
        }
    }

    pub fn run(&mut self) {
        self.event_loop.register(&self.server, SERVER);

        self.event_loop.run(self);
    }
}

impl mio::Handler for HttpListener {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<HttpListener>, token: mio::Token, events: mio::EventSet) {
        match token {
            SERVER => {
                match self.server.accept() {
                    Ok(Some(socket)) => {
                        let token = self.connections
                                        .insert_with(|token| Connection::new(socket, token))
                                        .unwrap();

                        event_loop.register_opt(&self.connections[token].socket,
                                                token,
                                                mio::EventSet::readable(),
                                                mio::PollOpt::edge() | mio::PollOpt::oneshot()).unwrap();
                    }
                    Ok(None) => {
                    }
                    Err(e) => {
                    }
                }
            }
            _ => panic!("Received unknown token"),
        }
    }
}

pub trait Handler: Send + Sync {
    fn handle<'a, 'k>(&'a self, Request<'a, 'k>, Response<'a>);
}
