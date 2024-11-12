use std::time::Duration;
use tokio::fs;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::sleep;
use tracing::{debug, error, info};

#[derive(Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

pub struct Server {
    listener: TcpListener,
}

const GET_HELLO: &str = "GET /hello HTTP/1.1";
const GET_SLEEP: &str = "GET /sleep HTTP/1.1";
const RESPONSE_STATUS_200: &str = "HTTP/1.1 200 OK\r\n\r\n";
const RESPONSE_STATUS_404: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

const RESPONSE_BODY_HELLO: &str = "html/hello.html";
const RESPONSE_BODY_SLEEP: &str = "html/sleep.html";
const RESPONSE_BODY_404: &str = "html/404.html";

impl Server {
    pub async fn start(config: ServerConfig) -> Result<(), io::Error> {
        let server = Server::new(config).await?;

        loop {
            let (stream, _) = server.listener.accept().await?;
            server.handle_request(stream).await?
        }
    }

    async fn new(config: ServerConfig) -> Result<Self, io::Error> {
        let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;

        Ok(Self { listener })
    }

    async fn handle_request(&self, mut stream: TcpStream) -> io::Result<()> {
        info!(target: "server", "Connection established!");

        tokio::spawn(async move {
            let mut buf_reader = io::BufReader::new(&mut stream);
            let mut http_request = Vec::new();
            let mut line = String::new();

            while buf_reader.read_line(&mut line).await.unwrap_or(0) > 0 {
                let trimmed = line.trim().to_string();
                if trimmed.is_empty() {
                    break;
                }
                http_request.push(trimmed);
                line.clear(); // Clear the line buffer for the next read
            }

            debug!(target: "server", "Request: {http_request:#?}");

            let (status_line, filename) = match http_request.first() {
                Some(val) => match val.as_str() {
                    GET_HELLO => (RESPONSE_STATUS_200, RESPONSE_BODY_HELLO),
                    GET_SLEEP => {
                        sleep(Duration::from_secs(30)).await;
                        (RESPONSE_STATUS_200, RESPONSE_BODY_SLEEP)
                    }
                    _ => (RESPONSE_STATUS_404, RESPONSE_BODY_404),
                },
                None => (RESPONSE_STATUS_404, RESPONSE_BODY_404),
            };

            let contents = match fs::read_to_string(filename).await {
                Ok(contents) => contents,
                Err(err) => {
                    error!(target: "server", "Failed to read file: {}", err);
                    return;
                }
            };
            let response = format!("{status_line}{contents}");

            if let Err(err) = stream.write_all(response.as_bytes()).await {
                error!(target: "server", "Failed to write to stream: {}", err);
                return;
            }
            if let Err(err) = stream.flush().await {
                error!(target: "server", "Failed to flush stream: {}", err);
            }
        });

        Ok(())
    }
}
