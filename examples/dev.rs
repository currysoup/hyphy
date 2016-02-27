#![deny(warnings)]
extern crate hyphy;

use hyphy::server::{Request, Response};

static PHRASE: &'static [u8] = b"Hello World!";

fn hello(_: Request, res: Response) {
    res.send(PHRASE).unwrap();
}

fn main() {
    hyphy::server::run_server("127.0.0.1:3000", config, hello);
    println!("Listening on http://127.0.0.1:3000");
}
