use std::error::Error;
use std::{collections::HashMap, io::Read, net::SocketAddr};
use tokio::net::TcpStream;
use std::fmt;

use super::parser::Parser;
use super::caller::AsyncReturn;
use super::http_header::HttpHeader;
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

    pub fn set_header(&mut self, header: HttpHeader) -> Option<String>{
        let parsed_header = header.as_str();
        let key = parsed_header.0;
        let value = parsed_header.1;
        return self.response_map.insert(key.to_string(), value.to_string());
    }

    pub fn set_body(&mut self, body: String){
        self.set_response("Content-Length".to_string(), format!("{}", body.len()));
        self.set_response("Body".to_string(), body);
    }

    pub fn set_body_from_html(&mut self, file_path: &str) -> Result<(), Box<dyn Error>>{
        let mut file = std::fs::File::open(file_path)?;
        let mut body = String::new();
        file.read_to_string(&mut body)?;
        self.set_response("Content-Length".to_string(), format!("{}", body.len()));
        self.set_response("Body".to_string(), body);
        self.set_content_type("text/html".to_string());
        Ok(())
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

        let mut header_string = String::new();
        for header in self.response_map.keys(){
            if header.contains("Status") || header.contains("Content-Type") || 
                header.contains("Content-Length") || header.contains("Body") { continue }
            header_string.push_str(&format!("{}: {}\r\n", header, self.response_map.get(header).unwrap()));
        }
        header_string.push_str("\r\n");

        let payload = format!(
            "HTTP/1.1 {} \r\n\
            Content-Type: {}; charset=utf-8\r\n\
            Content-Length: {}\r\n\
            {}\
            {}",
            self.response_map.get("Status").unwrap(),
            self.response_map.get("Content-Type").unwrap(),
            self.response_map.get("Content-Length").unwrap(),
            header_string,
            self.response_map.get("Body").unwrap(),
        );

        println!("{payload}");

        Ok(Box::pin(async move {
            payload
        }))
    }

}

pub struct Request {
    pub request: Parser,
}

pub struct MultiForm {
    pub generic_value : Option<String>,
    pub file: Option<Vec<u8>>
}

impl Request{
    pub fn new(request: Parser) -> Self{
        Request{request}
    }

    pub fn parse_multipart_form(&self) -> Option<HashMap<String, MultiForm>>{
        use multipart::server::Multipart;
        use std::io::{Cursor, Read};

        let mut form_fields = HashMap::new();
        let content_type = &self.request.header.get("content-type").unwrap();
        let boundary_prefix = "boundary=";
        let boundary = content_type
            .split(';')
            .find(|&part| part.trim().starts_with(boundary_prefix))?
            .trim()[boundary_prefix.len()..]
            .to_string();

        let cursor = Cursor::new(self.request.clone().body.unwrap());
        let mut multipart = Multipart::with_body(cursor, boundary);

        while let Ok(field) = multipart.read_entry() {
            if field.is_none() {
                break;
            }
            if field.as_ref().is_some(){
                let header = field.as_ref().unwrap().headers.name.to_string();
                if header == "file" {
                    let mut file_content = Vec::new();
                    let name = field.as_ref().unwrap().headers.name.to_string();
                    field.unwrap().data.read_to_end(&mut file_content).unwrap();
                    let mf = MultiForm{generic_value: None, file: Some(file_content)};
                    form_fields.insert(name, mf);
                }
                else{
                    let mut field_value = String::new();
                    let name = field.as_ref().unwrap().headers.name.to_string();
                    field.unwrap().data.read_to_string(&mut field_value).unwrap();
                    let mf = MultiForm{generic_value: Some(field_value), file: None};
                    form_fields.insert(name, mf);
                 }
            }
        }

        Some(form_fields)
    }

    pub fn parse_url_form(&self) -> Option<HashMap<String, String>>{
        use url::form_urlencoded;

        let body  = match &self.request.body{
            Some(b) => b,
            None => return None
        };

         // Parse the form-urlencoded data into a Vec of tuples
        let parsed: Vec<(String, String)> = form_urlencoded::parse(body.as_bytes())
        .into_owned()
        .collect();

        // Create a HashMap and insert the key-value pairs
        let mut map = HashMap::new();
        for (key, value) in parsed {
            map.insert(key, value);
        }

        Some(map)
    }

}