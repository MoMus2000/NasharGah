mod khadim;

use crate::khadim::server::Server;
use crate::khadim::response::{Request, ResponseWriter, create_http_response};

pub fn callback_function(request: Request, mut writer: ResponseWriter) -> String{
    writer.set_response("Status".to_string(), "OK 200".to_string());
    writer.set_response("Content-Type".to_string(), "text/html".to_string());
    writer.set_body("<h1> Hello World </h1>".to_string());
    writer.response()
}

#[tokio::main]
async fn main() {
    let port = "8080";
    let address = "127.0.0.1";
    let mut server = Server::new(port, address).await.unwrap();
    server.add_route("/", "GET", callback_function);
    server.add_route("/mustafa", "GET", callback_function);
    server.listen().await;
}
