pub mod connection_handler {
    use std::net::TcpStream;
    use std::io::prelude::*;
    
    pub struct ConnectionHandler {
    }
    
    impl ConnectionHandler {
        pub fn new() -> ConnectionHandler {
            ConnectionHandler {}
        }

        pub fn handle_connection(&self, mut stream: TcpStream) {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
            let response = "HTTP/1.1 200 OK\r\n\r\n";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
