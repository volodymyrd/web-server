use std::fs::read_to_string;
use std::io;
use std::io::{BufRead, BufReader, Error, Write};
use std::net::{TcpListener, TcpStream};
use tracing::{debug, info};

#[derive(Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

pub struct Server {
    listener: TcpListener,
}

const GET_HELLO: &str = "GET /hello HTTP/1.1";
const RESPONSE_STATUS_200: &str = "HTTP/1.1 200 OK\r\n\r\n";
const RESPONSE_STATUS_404: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

const RESPONSE_BODY_HELLO: &str = "html/hello.html";
const RESPONSE_BODY_404: &str = "html/404.html";

impl Server {
    pub fn start(config: ServerConfig) -> Result<(), Error> {
        let server = Server::new(config)?;

        for stream in server.listener.incoming() {
            let stream = stream?;

            info!(target: "server", "Connection established!");
            server.handle_request(stream)?;
        }
        Ok(())
    }

    fn new(config: ServerConfig) -> Result<Self, Error> {
        let listener = TcpListener::bind(format!("{}:{}", config.host, config.port))?;

        Ok(Self { listener })
    }

    fn handle_request(&self, mut stream: TcpStream) -> io::Result<()> {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map_while(Result::ok)
            .take_while(|line| !line.is_empty())
            .collect();

        debug!(target: "server", "Request: {http_request:#?}");

        let (status_line, filename) = match http_request.first() {
            Some(val) => match val.as_str() {
                GET_HELLO => (RESPONSE_STATUS_200, RESPONSE_BODY_HELLO),
                _ => (RESPONSE_STATUS_404, RESPONSE_BODY_404),
            },
            None => (RESPONSE_STATUS_404, RESPONSE_BODY_404),
        };

        let contents = read_to_string(filename)?;
        let response = format!("{status_line}{contents}");

        stream.write_all(response.as_bytes()).unwrap();
        Ok(())
    }
}
