use std::io::prelude::*;
use std::net::TcpStream;

struct ConnectionHandler {
    stream: TcpStream
}

impl ConnectionHandler {
    fn handle_connection(&mut self)
}