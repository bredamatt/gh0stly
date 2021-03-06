use std::net::TcpListener;
use basic::thread_pool::ThreadPool;
use basic::connection_handler::ConnectionHandler;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute( || {
            let connection_handler: ConnectionHandler = ConnectionHandler::new();
            connection_handler.handle_connection(stream);
        });
    }
}
