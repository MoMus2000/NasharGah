# NasharGah
All Purpose Http Server

## Setup

```rust
use nashar_gah::khadim::server::Server;
use nashar_gah::khadim::response::{Request, ResponseWriter};
use nashar_gah::khadim::http_status::HttpStatus;
use nashar_gah::api_callback;


#[api_callback]
pub fn callback_function(_request: Request, mut writer: ResponseWriter) {
    writer.set_status(HttpStatus::Ok);
    writer.set_body("<h1> Hello World </h1>".to_string());
    writer.set_content_type("text/html".to_string());
    writer.response()
}

#[api_callback]
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

### 18th August 2024 

Returning Text

```bash
projects/nashar_gah [main] $ wrk -t5 -c100 -d60s http://localhost:8080
```

| Metric             | Value        |
|--------------------|--------------|
| **Requests/sec**   | 545.98       |
| **Transfer/sec**   | 64.51K       |
| **Total Requests** | 32,805       |
| **Data Transferred** | 3.79MB      |


### 19th August 2024 

Returning a simple html page

```bash
projects/nashar_gah [main] $ wrk -t5 -c100 -d60s http://localhost:8080
```

| Metric             | Value        |
|--------------------|--------------|
| **Requests/sec**   | 122,106.62   |
| **Transfer/sec**   | 431.80M      |
| **Total Requests** | 7,334,360    |
| **Data Transferred** | 25.33GB     |


###  20th August

Load Testing, sending a html file and Benchmarking

```bash
projects/nashar_gah [main] $ wrk -t8 -c500 -d15m http://localhost:8080
```

| **Metric**                  | **NasharGah (Rust)** | **Go (std)** | **Python (Flask - Gunicorn - Gevent)** |
|-----------------------------|----------------------|--------------|----------------------------------------|
| **RPS**                     | 112,358.62           | 64,853.99    | 6,604.18                               |
| **Latency (Avg)**           | 5.26 ms              | 9.99 ms      | 116.03 ms                              |
| **Latency (Stdev)**         | 6.73 ms              | 11.78 ms     | 133.06 ms                              |
| **Latency (Max)**           | 208.31 ms            | 242.86 ms    | 2.00 s                                 |
| **Throughput**              | 397.33 MB/sec        | 234.84 MB/sec| 24.90 MB/sec                           |
| **Total Requests**          | 101,128,099          | 58,373,504   | 5,943,958                              |
| **Total Data Transferred**  | 349.23 GB            | 206.42 GB    | 21.89 GB                               |
| **Socket Errors (Read)**    | 656                  | 664          | 1,090                                  |
| **Socket Errors (Write)**   | 0                    | 0            | 153                                    |
| **Socket Errors (Timeout)** | 0                    | 0            | 94                                     |
