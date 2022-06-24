pub mod connection_handler {
    use std::fs;
    use std::net::TcpStream;
    use std::io::prelude::*;
    use std::thread;
    use std::time::Duration;

    pub struct ConnectionHandler {}
    
    impl ConnectionHandler {
        pub fn new() -> ConnectionHandler {
            ConnectionHandler {}
        }

        fn request_handler(&self, buffer: &[u8]) -> String {
            let get = b"GET / HTTP/1.1\r\n";
            let sleep = b"GET /sleep HTTP/1.1\r\n";

            let (status_line, filename) = if buffer.starts_with(get) {
                ("HTTP/1.1 200 OK", "hello.html")
            } else if buffer.starts_with(sleep) {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "hello.html")
            } else {
                ("HTTP/1.1 404 Not Found", "404.html")
            };
            
            let contents = fs::read_to_string(filename).unwrap();
            let response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}", 
                status_line, contents.len(), contents,
            );

            response
        }

        pub fn handle_connection(&self, mut stream: TcpStream) {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

            // request handler
            let response = self.request_handler(&buffer[..]);
            
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}


pub mod thread_pool {
    use std::thread;

    pub struct Worker {
        id: usize,
        thread: thread::JoinHandle<()>
    }

    impl Worker {
        fn new(id: usize) -> Worker {
            let thread = thread::spawn(|| {});

            Worker { 
                id,
                thread,
            }
        }
    }
    pub struct ThreadPool {
        workers: Vec<Worker>,
    }

    impl ThreadPool {
        /// Create a ThreadPool
        /// 
        /// Size is number of threads in pool
        /// 
        /// # Panics
        /// 
        /// The `new` function will panic if the size is zero
        pub fn new (size: usize) -> ThreadPool {
            assert!(size > 0);

            let mut workers = Vec::with_capacity(size);
            for id in 0..size {
                workers.push(Worker::new(id));
            }

            ThreadPool {
                workers,
            }
        }

        pub fn execute<F>(&self, f: F)
            where
                F: FnOnce() + Send + 'static {
                }
    }
}