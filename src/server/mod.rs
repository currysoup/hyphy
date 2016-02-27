use std::net::{SocketAddr, ToSocketAddrs};
use std::thread;

use config::Config;
use net::StatusCode;

pub struct Request {
    pub remote_addr: SocketAddr,
}

pub struct Response {
    pub status: StatusCode,
}

pub fn run_server<A, H>(addr: A, config: Config, handler: H) -> Server
    where A: ToSocketAddrs, H: Handler + 'static{
    for i in 0..config.threads {
        thread::spawn(move || {
            let event_loop = EventLoop::new().unwrap();
            let server = TcpListener::bind(&self.addr).unwrap();

            event_loop.register(&server, SERVER);
            event_loop.run(&mut HttpListener::new(server, handler));
        });
    }
}

impl<F> Handler for F where F: Fn(Request, Response<Fresh>), F: Sync + Send {
    fn handle<'a, 'k>(&'a self, req: Request<'a, 'k>, res: Response<'a, Fresh>) {
        self(req, res)
    }
}

struct Connection {
    socket: TcpStream,
    token: Token,
}

impl Connection {
    pub fn new(socket: TcpStream, token: Token) -> Connection {
        Connection {
            socket: socket,
            token: token,
        }
    }
}
