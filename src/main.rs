mod khadim;

use crate::khadim::server::Server;
use crate::khadim::response::{Request, ResponseWriter, create_http_response};

pub fn callback_function(request: Request, writer: ResponseWriter) -> String{
    println!("Hey there 123");
    create_http_response()
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
