# NasharGah (Broadcaster)
All Purpose Http Server

## Setup

```rust
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
```

## M1 Macbook Tuning

```bash
# Increase TCP Buffer
sudo sysctl -w net.inet.tcp.sendspace=262144
sudo sysctl -w net.inet.tcp.recvspace=262144

# Increase number of sockets
ulimit -n 6553
```

## Performance Log
```bash
18th August 2024 (Returning Text)
projects/nashar_gah [main] $ wrk -t5 -c100 -d60s http://localhost:8080
Running 1m test @ http://localhost:8080
  5 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.44ms  330.79us   4.82ms   94.73%
    Req/Sec     6.06k     2.18k    7.54k    90.00%
  32805 requests in 1.00m, 3.79MB read
Requests/sec:    545.98
Transfer/sec:     64.51K
```