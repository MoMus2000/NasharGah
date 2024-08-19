use std::{collections::HashMap, io::Read, net::SocketAddr};
use tokio::net::TcpStream;
use std::fmt;

use super::parser::Parser;
use super::caller::AsyncReturn;
use std::boxed::Box;

pub struct ResponseWriter<'a>{
    pub conn: &'a TcpStream,
    pub address:  SocketAddr,
    response_map: HashMap<String, String>
}

impl<'a> ResponseWriter<'a> {
    pub fn new(conn: &'a TcpStream, address: SocketAddr) -> Self{
        let response_map = HashMap::new();
        ResponseWriter{conn, address, response_map}
    }

    fn set_response(&mut self, key: String, value: String) -> Option<String>{
        return self.response_map.insert(key, value) 
    }

    pub fn set_content_type(&mut self, value: String) -> Option<String>{
        return self.response_map.insert("Content-Type".to_string(), value);
    }

    pub fn set_body(&mut self, body: String){
        self.set_response("Content-Length".to_string(), format!("{}", body.len()));
        self.set_response("Body".to_string(), body);
    }

    pub fn set_body_from_html(&mut self, file_path: &str){
        let mut file = std::fs::File::open(file_path).unwrap();
        let mut body = String::new();
        file.read_to_string(&mut body).unwrap();
        self.set_response("Content-Length".to_string(), format!("{}", body.len()));
        self.set_response("Body".to_string(), body);
        self.set_content_type("text/html".to_string());
    }

    pub fn set_status(&mut self, status_code : impl fmt::Display) {
        self.response_map.insert("Status".to_string(), status_code.to_string());
    }

    pub fn response(&mut self) -> AsyncReturn{
        if !self.response_map.contains_key("Content-Type"){
            self.set_response(
                "Content-Type".to_string(),
                "text/plain".to_string()
            );
        }

        if !self.response_map.contains_key("Body"){
            self.set_response(
                "Body".to_string(),
                "".to_string()
            );
            self.set_response(
                "Content-Length".to_string(),
                "0".to_string()
            );
        }
        if !self.response_map.contains_key("Status"){
            self.set_response(
                "Status".to_string(),
                "200 OK".to_string()
            );
        }

        let payload = format!(
            "HTTP/1.1 {} \r\n\
            Content-Type: {}; charset=utf-8\r\n\
            Content-Length: {}\r\n\
            Connection: Close\r\n\
            \r\n\
            {}",
            self.response_map.get("Status").unwrap(),
            self.response_map.get("Content-Type").unwrap(),
            self.response_map.get("Content-Length").unwrap(),
            self.response_map.get("Body").unwrap(),
        );

        Box::pin(async move {
            payload
        })
    }

}

pub struct Request {
    pub request: Parser,
}

impl Request{
    pub fn new(request: Parser) -> Self{
        Request{request}
    }
}