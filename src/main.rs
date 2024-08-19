mod khadim;

use crate::khadim::server::Server;
use crate::khadim::response::{Request, ResponseWriter};
use crate::khadim::http_status::HttpStatus;

use std::pin::Pin;
use std::future::Future;
type AsyncReturn = Pin<Box<dyn Future<Output = String> + Send>>;
use std::boxed::Box;

pub fn callback_function<'a>(_request: Request, mut writer: ResponseWriter<'a>) -> AsyncReturn{
    writer.set_status(HttpStatus::Ok);
    writer.set_body("<h1> Hello World </h1>".to_string());
    writer.set_content_type("text/html".to_string());
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
