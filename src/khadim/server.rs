#![allow(dead_code)]
use std::net::{IpAddr, SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use anyhow::{Error, Result};

use super::{parser::Parser, response::{Request,ResponseWriter}, router::Router};

use std::pin::Pin;
use std::future::Future;
type AsyncReturn = Result<Pin<Box<dyn Future<Output = String> + Send>>, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
pub struct Server{
    pub port: u16,
    pub address: String,
    router: Router
}

impl Server{
    pub fn new(port: &str, address: &str) -> Result<Self, Error>{
        let port = port.to_string();
        let port : u16 = port.parse()?;
        Server::validate_port(port)?;
        let address = address.parse::<IpAddr>()?;
        let address = format!("{}:{}", address, port);
        let router = Router::new();
        let server = Server{port, address, router};
        Ok(server)
    }

    pub fn add_route(&mut self, path: &'static str, method: &'static str, callback_function: fn(Request, ResponseWriter) -> AsyncReturn){
        if !self.router.add_route(path, method, callback_function){
           panic!("ERROR adding route ..");
        }
    }

    fn validate_port(port: u16) -> Result<()>{
        match port{
            _ => Ok(())
        }
    }

    async fn bind(&self) -> Result<TcpListener, Error>{
        TcpListener::bind(self.address.clone()).await.map_err(Error::from)
    }
    
    pub async fn serve(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        match self.listen().await{
            Ok(_) => {},
            Err(_) => {
                return Err("Server should not have crashed".into());
            }
        };
        Ok(())
    }
    
    async fn listen(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        let listener = self.bind().await?;
        println!("Listening on {}", listener.local_addr().unwrap());
        loop{
            match listener.accept().await {
                Ok(mut conn) => {
                    let router = self.router.clone();
                    tokio::spawn(async move{
                        let mut keep_alive = true;
                        loop {
                            let parser = Server::read_request(&mut conn).await;
                            match &parser {
                                Some(pa) => {
                                    let headers = &pa.header;
                                    if let Some(connection_header) = headers.get("Connection") {
                                        if connection_header == "close" {
                                            keep_alive = false;
                                        }
                                    } 
                                }
                                None => {
                                    break
                                }
                            }
                            Server::handle_request(&mut conn, parser.clone(), &router).await;
                            if !keep_alive {
                                break
                            }
                        }
                        conn.0.shutdown().await.unwrap_or_else(|_|{})
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
            let index = match stream.0.read(&mut temp_buffer).await{
                Ok(index) => index,
                Err(_) => {
                    break
                }
            };
            buffer.extend_from_slice(&temp_buffer[..index]);
            if let Some(parsed_req) = Server::check_parsed_result(&buffer){
                if parsed_req_res.is_none(){
                    parsed_req_res = Some(parsed_req);
                }
                break
            }
        }
        return parsed_req_res;
    }

    async fn handle_request(stream: &mut (TcpStream, SocketAddr), parser: Option<Parser>, router: &Router) {
        let parser = parser.unwrap();
        let fetched_func = match router.fetch_func(&parser.path, &parser.method){
            Some(func) => func,
            None => {
                router.not_found_func.unwrap()
            }
        };
        let resp = match fetched_func(
            Request::new(parser.clone()),
            ResponseWriter::new(&stream.0, stream.1)
        ){
            Ok(result) => result,
            Err(e) => {
                println!("Error {e}");
                Box::pin(std::future::ready("Internal Server Error".to_string())) as Pin<Box<dyn Future<Output = String> + Send>>
            }
        };
        let mut resp = resp.await;
        if resp == "Internal Server Error" {
            resp = router.internal_server_error.unwrap()(
                Request::new(parser.clone()),
                ResponseWriter::new(&stream.0, stream.1)
            ).unwrap().await;
        }
        stream.0.write_all(resp.as_bytes()).await.unwrap();
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
