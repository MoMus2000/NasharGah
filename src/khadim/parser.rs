use std::{collections::HashMap, error::Error, net::SocketAddr};
use httparse::Request;
use url::Url;

#[derive(Debug, Clone)]
pub struct Parser{
    pub method: String,
    pub path: String,
    pub header: HashMap<String, String>,
    pub query_params: Option<HashMap<String, String>>
}

impl Parser{
    pub fn new(payload: Request, base_address: &SocketAddr) -> Self{
        let method = payload.method.unwrap().to_string();
        let path = payload.path.unwrap().to_string();

        let url_qp_tup = Parser::parse_url_and_get_query_params(&path, base_address);

        let url_qp_tup = match url_qp_tup{
            Ok(qp) => (qp.0, qp.1),
            Err(_) => panic!("Something went wrong here")
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
        Parser { method, path, header , query_params}
    }

    fn parse_url_and_get_query_params(relative_path: &str, base_address: &SocketAddr) -> Result<(String, HashMap<String, String>), Box<dyn Error>>{
        // Parse the URL
        let base = Url::parse(&format!("http://{}",&base_address))?;
        let full = base.join(relative_path)?;

        // Get the query pairs
        let query_pairs = full.query_pairs();

        let mut query_map = HashMap::new();

        for (key, value) in query_pairs{
            query_map.insert(key.to_string(), value.to_string());
        }

        Ok((full.path().to_string(), query_map))
    }

}