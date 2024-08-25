use std::{collections::HashMap, error::Error, net::SocketAddr};
use httparse::Request;
use url::Url;
use std::str;

#[derive(Debug, Clone)]
pub struct Parser{
    pub method: String,
    pub path: String,
    pub header: HashMap<String, String>,
    pub query_params: Option<HashMap<String, String>>,
    pub body: Option<String>
}

impl Parser{
    pub fn new(payload: Request, base_address: &SocketAddr, buffer: &[u8], parsed_len: usize) -> Result<Self, Box<dyn std::error::Error>>{
        let method = match payload.method{
            Some(verb) => verb.to_string(),
            None  => return Err("Wrong method".into())
        };

        let path = match payload.path{
            Some(p) => p.to_string(),
            None => return Err("No path found".into())
        };
        let body = Parser::parse_body(&payload, buffer, parsed_len);
        let url_qp_tup = Parser::parse_url_and_get_query_params(&path, base_address);

        let url_qp_tup = match url_qp_tup{
            Ok(qp) => (qp.0, qp.1),
            Err(e) => return Err(e.to_string().into())
        };

        let path = url_qp_tup.0;
        let query_params = Some(url_qp_tup.1);

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

        Ok(Parser { method, path, header , query_params, body})
    }

    fn parse_url_and_get_query_params(relative_path: &str, base_address: &SocketAddr) -> Result<(String, HashMap<String, String>), Box<dyn Error>>{
        let base = Url::parse(&format!("http://{}",&base_address))?;
        let full = base.join(relative_path)?;

        let query_pairs = full.query_pairs();

        let mut query_map = HashMap::new();

        for (key, value) in query_pairs{
            query_map.insert(key.to_string(), value.to_string());
        }

        Ok((full.path().to_string(), query_map))
    }

    fn parse_body(payload: &Request, buffer: &[u8], parsed_len: usize) -> Option<String> {
        match payload.method.unwrap() {
            "POST" | "PUT" | "DELETE" => {
                let body = &buffer[parsed_len..];
                let body = {
                    let b = str::from_utf8(body);
                    let b = match b {
                        Ok(valid) => valid.to_string(),
                        Err(_) => return None
                    };
                    Some(b)
                };
                return body;
            }
            _ => return None
        }
    }

}