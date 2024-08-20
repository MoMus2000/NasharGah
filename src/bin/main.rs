use nashar_gah::khadim::server::Server;
use nashar_gah::khadim::response::{Request, ResponseWriter};
use nashar_gah::khadim::http_status::HttpStatus;
use nashar_gah::khadim::http_header::HttpHeader;
use nashar_gah::api_callback;


#[api_callback]
pub fn callback_function(_request: Request, mut writer: ResponseWriter) {
    writer.set_status(HttpStatus::Ok);
    writer.set_body(r#"
    {
        "key": "value"
    }
    "#.to_string());
    writer.set_header(HttpHeader::ContentType("application/json"));
    writer.response()
}

#[api_callback]
pub fn serve_homepage(_request: Request, mut writer: ResponseWriter){
    writer.set_status(HttpStatus::Ok);
    writer.set_body_from_html("/Users/mmuhammad/Desktop/projects/nashar_gah/assets/index.html");
    writer.set_header(HttpHeader::ContentType("text/html"));
    writer.response()
}

#[tokio::main]
pub async fn main() {
    let port = "8080";
    let address = "127.0.0.1";
    let mut server = Server::new(port, address).await.unwrap();
    server.add_route("/", "GET", serve_homepage);
    server.add_route("/call_back", "GET", callback_function);
    server.listen().await;
}