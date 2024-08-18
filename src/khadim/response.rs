use std::{collections::HashMap, net::SocketAddr};
use tokio::net::TcpStream;

use super::parser::Parser;

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

    pub fn set_response(&mut self, key: String, value: String) -> Option<String>{
        return self.response_map.insert(key, value) 
    }

    pub fn set_body(&mut self, body: String){
        self.response_map.insert("Content-Length".to_string(), format!("{}", body.len()));
        self.response_map.insert("Body".to_string(), body);
    }

    pub fn set_status(&mut self, status_code : String) {
        self.response_map.insert("Status".to_string(), status_code);
    }

    pub fn response(&mut self) -> String {
        if !self.response_map.contains_key("Content-Type"){
            self.response_map.insert(
                "Content-Type".to_string(),
                "text/plain".to_string()
            );
        }

        if !self.response_map.contains_key("Body"){
            self.response_map.insert(
                "Body".to_string(),
                "".to_string()
            );
            self.response_map.insert(
                "Content-Length".to_string(),
                "0".to_string()
            );
        }
        if !self.response_map.contains_key("Status"){
            self.response_map.insert(
                "Status".to_string(),
                "200 OK".to_string()
            );
        }

        let payload = format!(
            "HTTP/1.1 {} \r\n\
            Content-Type: {}; charset=utf-8\r\n\
            Content-Length: {}\r\n\
            \r\n\
            {}",
            self.response_map.get("Status").unwrap(),
            self.response_map.get("Content-Type").unwrap(),
            self.response_map.get("Content-Length").unwrap(),
            self.response_map.get("Body").unwrap(),
        );

        payload
    }

}

pub struct Request {
    pub request: Parser
}

impl Request{
    pub fn new(request: Parser) -> Self{
        Request{request}
    }
}

pub fn create_http_response() -> String {
    let body = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>My Page</title>
        </head>
        <body>
            <h1>Welcome!</h1>
            <p>Hello World <b> This is mustafa Muhamamd </p>
        </body>
        </html>
    "#;

    let content_length = body.len();

    format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         \r\n\
         {}",
        content_length,
        body
    )
}