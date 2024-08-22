#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::Mutex;
    use meta_tags::api_callback;
    use tokio::net::TcpListener;

    use crate::khadim::response::ResponseWriter;
    use crate::khadim::response::Request;
    use crate::khadim::http_status::HttpStatus;
    use crate::khadim::server::Server;

    const START : u8 = 0;
    const EXIT : u8 = 1;

    #[api_callback]
    pub fn serve_req(_request: Request, mut writer: ResponseWriter) {
        writer.set_status(HttpStatus::Ok);
        writer.response()
    }

    #[api_callback]
    pub fn trigger_panic(_request: Request, mut writer: ResponseWriter){
        Err("An error occured")
    }

    #[api_callback]
    pub fn serve_get(_request: Request, mut writer: ResponseWriter){
        writer.set_status(HttpStatus::Ok);
        writer.response()
    }

    #[api_callback]
    pub fn serve_post(_request: Request, mut writer: ResponseWriter){
        writer.set_status(HttpStatus::Created);
        writer.response()
    }

    #[api_callback]
    pub fn serve_put(_request: Request, mut writer: ResponseWriter){
        writer.set_status(HttpStatus::Ok);
        writer.response()
    }

    #[api_callback]
    pub fn serve_delete(_request: Request, mut writer: ResponseWriter){
        writer.set_status(HttpStatus::NoContent);
        writer.response()
    }

    async fn fetch_port() -> u16{
        let address = "127.0.0.1:0";
        let listener = TcpListener::bind(&address).await.expect("Failed to bind to address");
        let port = listener.local_addr().expect("Failed to get local address").port();
        drop(listener);
        port
    }

    fn init_server(port: u16) -> Server{
        let server = match Server::new(&port.to_string(), "127.0.0.1") {
            Ok(server) => server,
            Err(e) => {
                panic!("Failed to create server: {}", e);
            }
        };
        server
    }

    #[tokio::test]
    async fn test_get_fail_wrong_method(){
        let port = fetch_port().await;
        let mut server = init_server(port);
        server.add_route("/", "POST", serve_get);
        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });
        tokio::task::yield_now().await;
        let status_code = reqwest::get(format!("http://localhost:{port}/"))
        .await
        .unwrap()
        .status();
        assert_eq!(status_code.as_str(), "404")
    }

    #[tokio::test]
    async fn test_pass_get(){
        let port = fetch_port().await;
        let mut server = init_server(port);
        server.add_route("/", "GET", serve_get);
        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });
        tokio::task::yield_now().await;
        let status_code = reqwest::get(format!("http://localhost:{port}/"))
        .await
        .unwrap()
        .status();
        assert_eq!(status_code.as_str(), "200")
    }

    #[tokio::test]
    async fn test_multiple_methods_on_same_route(){
        let port = fetch_port().await;
        let mut server = init_server(port);
        server.add_route("/", "GET", serve_get);
        server.add_route("/", "DELETE", serve_delete);
        server.add_route("/", "PUT", serve_put);
        server.add_route("/", "POST", serve_post);
        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });
        tokio::task::yield_now().await;
        let status_code = reqwest::get(format!("http://localhost:{port}/"))
        .await
        .unwrap()
        .status();
        assert_eq!(status_code.as_str(), "200");
        let client = reqwest::Client::new();
        let status_code = client
        .post(format!("http://localhost:{port}/"))
        .send()
        .await
        .unwrap()
        .status();
        assert_eq!(status_code.as_str(), "201");
        let client = reqwest::Client::new();
        let status_code = client
        .put(format!("http://localhost:{port}/"))
        .send()
        .await
        .unwrap()
        .status();
        assert_eq!(status_code.as_str(), "200");
        let client = reqwest::Client::new();
        let status_code = client
        .delete(format!("http://localhost:{port}/"))
        .send()
        .await
        .unwrap()
        .status();
        assert_eq!(status_code.as_str(), "204");
    }

    #[tokio::test]
    async fn test_pass_post(){
        let port = fetch_port().await;
        let mut server = init_server(port);
        server.add_route("/", "POST", serve_post);
        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });
        tokio::task::yield_now().await;
        let client = reqwest::Client::new();
        let status_code = client
        .post(format!("http://localhost:{port}/"))
        .send()
        .await
        .unwrap()
        .status();
        assert_eq!(status_code.as_str(), "201")
    }

    #[tokio::test]
    async fn test_pass_put(){
        let port = fetch_port().await;
        let mut server = init_server(port);
        server.add_route("/", "PUT", serve_put);
        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });
        tokio::task::yield_now().await;
        let client = reqwest::Client::new();
        let status_code = client
        .put(format!("http://localhost:{port}/"))
        .send()
        .await
        .unwrap()
        .status();
        assert_eq!(status_code.as_str(), "200")
    }

    #[tokio::test]
    async fn test_pass_delete(){
        let port = fetch_port().await;
        let mut server = init_server(port);
        server.add_route("/", "DELETE", serve_delete);
        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });
        tokio::task::yield_now().await;
        let client = reqwest::Client::new();
        let status_code = client
        .delete(format!("http://localhost:{port}/"))
        .send()
        .await
        .unwrap()
        .status();
        assert_eq!(status_code.as_str(), "204")
    }

    #[tokio::test]
    async fn test_500(){
        let port = fetch_port().await;
        let mut server = init_server(port);
        server.add_route("/", "GET", trigger_panic);
        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });
        tokio::task::yield_now().await;
        let status_code = reqwest::get(format!("http://localhost:{port}/"))
        .await
        .unwrap()
        .status();
        assert_eq!(status_code.as_str(), "500")
    }

    #[tokio::test]
    async fn test_200(){
        let port = fetch_port().await;
        let mut server = init_server(port);
        server.add_route("/", "GET", serve_req);
        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });
        tokio::task::yield_now().await;
        let status_code = reqwest::get(format!("http://localhost:{port}/"))
        .await
        .unwrap()
        .status();
        assert_eq!(status_code.as_str(), "200")
    }

    #[tokio::test]
    async fn test_404(){
        let port = fetch_port().await;
        let mut server = init_server(port);
        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });
        tokio::task::yield_now().await;
        let status_code = reqwest::get(format!("http://localhost:{port}/"))
        .await
        .unwrap()
        .status();
        assert_eq!(status_code.as_str(), "404")

    }

    #[tokio::test]
    async fn test_init() {
        let port = 8080;
        let mut server = init_server(port);
        let i = Arc::new(Mutex::new(START));
        let i_c = i.clone();
        let _ = tokio::spawn(async move {
            if let Err(_) = server.serve().await {
                *i_c.lock().unwrap() = EXIT;
            }
        });
        tokio::task::yield_now().await;
        if *i.lock().unwrap() == EXIT{
            panic!("ERROR initializing the server !")
        }
    }

}