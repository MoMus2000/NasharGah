# NasharGah
All Purpose Http Server

[Performance Log](performance-log/README.md)

## Setup

```rust
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
    writer.set_body_from_html("./assets/index.html");
    writer.set_header(HttpHeader::ContentType("text/html"));
    writer.response()
}

#[init]
fn main() -> Server{
    let port = "8080";
    let address = "127.0.0.1";
    let mut server = Server::new(port, address).unwrap();
    server.add_route("/", "GET", serve_homepage);
    server.add_route("/call_back", "GET", callback_function);
    server
}
```

## M1 Macbook Tuning

```bash
# Increase TCP Buffer
sudo sysctl -w net.inet.tcp.sendspace=262144
sudo sysctl -w net.inet.tcp.recvspace=262144

# Increase number of sockets
ulimit -n 6553
```
