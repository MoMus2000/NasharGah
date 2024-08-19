use nashar_gah::khadim::server::Server;
use nashar_gah::khadim::response::{Request, ResponseWriter};
use nashar_gah::khadim::http_status::HttpStatus;
use meta_tags::callback;


#[callback]
pub fn callback_function(_request: Request, mut writer: ResponseWriter) {
    writer.set_status(HttpStatus::Ok);
    writer.set_body("<h1> Hello World </h1>".to_string());
    writer.set_content_type("text/html".to_string());
    writer.response()
}

#[callback]
pub fn serve_homepage(_request: Request, mut writer: ResponseWriter){
    writer.set_status(HttpStatus::Ok);
    writer.set_body_from_html("/Users/mmuhammad/Desktop/projects/nashar_gah/assets/index.html");
    writer.response()
}

#[tokio::main]
pub async fn main() {
    let port = "8080";
    let address = "127.0.0.1";
    let mut server = Server::new(port, address).await.unwrap();
    server.add_route("/", "GET", serve_homepage);
    server.add_route("/mustafa", "GET", serve_homepage);
    server.add_route("/call_back", "GET", callback_function);
    server.listen().await;
}