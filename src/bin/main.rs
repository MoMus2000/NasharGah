use nashar_gah::khadim::server::Server;
use nashar_gah::khadim::response::{Request, ResponseWriter};
use nashar_gah::khadim::http_status::HttpStatus;
use nashar_gah::khadim::http_header::HttpHeader;
use nashar_gah::{api_callback, init};


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
pub fn serve_homepage(_request: Request, mut writer: ResponseWriter) {
    writer.set_status(HttpStatus::Ok);
    writer.set_body_from_html("/Users/mmuhammad/Desktop/projects/nashar_gah/assets/index.html")?;
    writer.set_header(HttpHeader::ContentType("text/html"));
    writer.response()
}

#[api_callback]
pub fn serve_slow(_request: Request, mut writer: ResponseWriter){
    writer.set_status(HttpStatus::Ok);
    use std::thread;
    use std::time::Duration;
    thread::sleep(Duration::from_secs(10));
    writer.response()
}

#[init]
fn main() -> Server{
    let port = "8080";
    let address = "127.0.0.1";
    let mut server = Server::new(port, address).unwrap();
    server.add_route("/", "GET", serve_homepage);
    server.add_route("/slow", "GET", serve_slow);
    server.add_route("/call_back", "GET", callback_function);
    server
}
