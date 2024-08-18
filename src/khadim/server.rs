#![allow(dead_code)]
use std::net::{IpAddr, SocketAddr};
use http::header;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, sync::Mutex};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use anyhow::{anyhow, Error, Result};

use super::{parser::Parser, router::Router};
use super::response::create_http_response;

pub struct Server{
    pub port: u16,
    pub address: String,
    router: Router
}

impl Server{
    pub async fn new(port: &str, address: &str) -> Result<Self, Error>{
        let port = port.to_string();
        let port : u16 = port.parse()?;
        Server::validate_port(port)?;
        let address = address.parse::<IpAddr>()?;
        let address = format!("{}:{}", address, port);
        let router = Router::new();
        let server = Server{port, address, router};
        Ok(server)
    }

    pub fn add_route(&mut self, path: &'static str, method: &'static str, callback_function: fn()){
        if !self.router.add_route(path, method, callback_function){
           panic!("ERROR adding route ..");
        }
    }

    fn validate_port(port: u16) -> Result<()>{
        match port{
            0 => Err(anyhow!("Port 0 is reserved and cannot be used directly.")),
            1..1024 => Err(anyhow!("Elevated ports are not currently supported.")),
            1024..62535 => Ok(()),
            _ => Err(anyhow!("Port Number is outside the valid range.")),
        }
    }

    async fn bind(&self) -> Result<TcpListener, Error>{
        TcpListener::bind(self.address.clone()).await.map_err(Error::from)
    }

    pub async fn listen(&mut self){
        let listener = self.bind().await.unwrap();
        let listener = Arc::new(Mutex::new(listener));
        println!("Listening on {}", self.address);
        loop{
            match listener.lock().await.accept().await {
                Ok(mut conn) => {
                    let router = self.router.clone();
                    tokio::spawn(async move{
                        let parser = Server::read_request(&mut conn).await;
                        Server::handle_request(&mut conn, parser, router).await;
                    });
                }
                Err(err) => {
                    println!("An error occured getting the connection {}", err);
                }
            }
        }
    }

    async fn read_request(stream: &mut (TcpStream, SocketAddr)) -> Option<Parser>{
        let mut buffer : Vec<u8> = Vec::new();
        let mut temp_buffer = [0; 1024];
        let mut parsed_req_res : Option<Parser> = None;
        loop {
            let index = stream.0.read(&mut temp_buffer).await.unwrap();
            buffer.extend_from_slice(&temp_buffer[..index]);
            if let Some(parsed_req) = Server::check_parsed_result(&buffer){
                parsed_req_res = Some(parsed_req);
                break
            }
        }
        return parsed_req_res;
    }

    async fn handle_request(stream: &mut (TcpStream, SocketAddr), parser: Option<Parser>, router: Router){
        let response = create_http_response();
        let parser = parser.unwrap();
        router.fetch_func(&parser.path, &parser.method).unwrap()();
        stream.0.write_all(&response.as_bytes()).await.unwrap();
        stream.0.flush().await.unwrap();
    }

    fn check_parsed_result(buffer: &[u8]) -> Option<Parser>{
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);
        let parsed_result = {
            req.parse(&buffer)
        };
        match parsed_result{
            Ok(httparse::Status::Complete(_)) => {
                let parsed_req = Parser::new(req);
                return Some(parsed_req)
            }
            Ok(httparse::Status::Partial) => {
                None
            }
            Err(e) => {
                eprintln!("Failed to parse request: {}", e);
                return None
            }
        }
    }
}
