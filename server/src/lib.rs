use std::io::{BufRead, BufReader, Error, Write};
use std::net::{TcpListener, TcpStream};
use tracing::{debug, info};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn start() -> Result<(), Error> {
        let server = Server::new()?;

        for stream in server.listener.incoming() {
            let stream = stream?;

            info!(target: "server", "Connection established!");
            server.handle_request(stream);
        }
        Ok(())
    }

    fn new() -> Result<Self, Error> {
        let listener = TcpListener::bind("127.0.0.1:7878")?;

        Ok(Self { listener })
    }

    fn handle_request(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map_while(Result::ok)
            .take_while(|line| !line.is_empty())
            .collect();

        debug!(target: "server", "Request: {http_request:#?}");

        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write_all(response.as_bytes()).unwrap();
    }
}
