mod khadim;

use crate::khadim::server::Server;
use crate::khadim::response::{Request, ResponseWriter};
use crate::khadim::http_status::HttpStatus;

use std::pin::Pin;
use std::future::Future;
use std::boxed::Box;

type AsyncReturn = Pin<Box<dyn Future<Output = String> + Send>>;

pub fn callback_function<'a>(_request: Request, mut writer: ResponseWriter<'a>) -> AsyncReturn{
    writer.set_status(HttpStatus::Ok);
    writer.set_body("<h1> Hello World </h1>".to_string());
    writer.set_content_type("text/html".to_string());
    writer.response()
}

pub fn serve_homepage<'a>(_request: Request, mut writer: ResponseWriter<'a>) -> AsyncReturn{
    writer.set_status(HttpStatus::Ok);
    writer.set_body_from_html("/Users/mmuhammad/Desktop/projects/nashar_gah/assets/index.html");
    writer.response()
}

#[tokio::main]
async fn main() {
    let port = "8080";
    let address = "127.0.0.1";
    let mut server = Server::new(port, address).await.unwrap();
    server.add_route("/", "GET", serve_homepage);
    server.add_route("/call_back", "GET", callback_function);
    server.listen().await;
}
