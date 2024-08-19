use std::collections::HashMap;

use httparse::Request;

#[derive(Debug)]
pub struct Parser{
    pub method: String,
    pub path: String,
    pub header: HashMap<String, String>
}

impl Parser{
    pub fn new(payload: Request) -> Self{
        let method = payload.method.unwrap().to_string();
        let path = payload.path.unwrap().to_string();
        let headers = payload.headers;
        let mut header_map : HashMap<String, String>= HashMap::new();
        for header in headers{
            if header_map.contains_key(header.name){
            }
            else{
                let value = std::str::from_utf8(header.value).unwrap();
                header_map.insert(header.name.to_string(), value.to_string());
            }
        }
        let header = header_map;
        Parser { method, path, header }
    }
}