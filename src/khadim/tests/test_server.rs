#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::time::Duration;
    use meta_tags::api_callback;
    use tokio::net::TcpListener;

    use crate::khadim::response::ResponseWriter;
    use crate::khadim::response::Request;
    use crate::khadim::http_status::HttpStatus;
    use crate::khadim::server::Server;
    use crate::khadim::http_header::HttpHeader;

    const START : u8 = 0;
    const EXIT : u8 = 1;

    #[api_callback]
    pub fn serve_req(_request: Request, mut writer: ResponseWriter) {
        writer.set_status(HttpStatus::Ok);
        writer.response()
    }

    #[api_callback]
    pub fn handle_query_param(request: Request, mut writer: ResponseWriter){
        writer.set_status(HttpStatus::Ok);
        assert_eq!(request.request.query_params.as_ref().unwrap().get("foo").unwrap(), "bar");
        assert_eq!(request.request.query_params.as_ref().unwrap().get("bar").unwrap(), "baz");
        writer.response()
    }

    #[api_callback]
    pub fn set_user_agent(_request: Request, mut writer: ResponseWriter) {
        writer.set_status(HttpStatus::Ok);
        writer.set_header(HttpHeader::UserAgent("Mustafa".to_string()));
        writer.response()
    }

    #[api_callback]
    pub fn trigger_panic(_request: Request, mut writer: ResponseWriter){
        Err("An error occured")
    }

    #[api_callback]
    pub fn process_multipart_form(request: Request, mut writer: ResponseWriter){
        let multipart_form = request.parse_multipart_form();
        if multipart_form.as_ref().unwrap().len() > 0{
            assert_eq!(multipart_form.as_ref().unwrap().get("field1").unwrap(), "value1");
            assert_eq!(multipart_form.as_ref().unwrap().get("field2").unwrap(), "value2");
            writer.set_status("200");
            return writer.response()
        }
        writer.set_status("500");
        writer.response()
    }

    #[api_callback]
    pub fn process_url_encoded_form(_request: Request, mut writer: ResponseWriter){
        let form = match request.parse_url_form(){
            Some(f) => f,
            None => {
                HashMap::new()
            }
        };
        writer.set_status(HttpStatus::Ok);

        if form.len() != 0{
            return writer.response();
        }
        Err("An error occured")?
    }

    #[api_callback]
    pub fn process_redirect(_request: Request, mut writer: ResponseWriter){
        let port = match request.request.body{
            Some(data) => {
                data
            },
            None => {
                String::new()
            }
        };
        let new_location = format!("http://localhost:{}/1",port);
        writer.set_header(HttpHeader::Location(new_location));
        writer.response()
    }

    #[api_callback]
    pub fn process_payload(_request: Request, mut writer: ResponseWriter){
        let payload = match request.request.body{
            Some(data) => {
                data
            },
            None => {
                String::new()
            }
        };
        assert_eq!(payload, format!("Hello"));
        writer.set_status("201");
        writer.response()
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
        let payload = match request.request.body{
            Some(data) => {
                data
            },
            None => {
                String::new()
            }
        };
        assert_eq!(payload, "{id: 1}");
        writer.set_status(HttpStatus::NoContent);
        writer.response()
    }

    #[api_callback]
    pub fn serve_delete(_request: Request, mut writer: ResponseWriter){
        writer.set_status(HttpStatus::NoContent);
        writer.response()
    }

    #[api_callback]
    pub fn serve_slow(_request: Request, mut writer: ResponseWriter){
        writer.set_status(HttpStatus::Ok);
        use std::thread;
        thread::sleep(Duration::from_secs(1));
        writer.response()
    }

    #[api_callback]
    pub fn file_not_exist(_request: Request, mut writer: ResponseWriter){
        writer.set_status(HttpStatus::Ok);
        writer.set_body_from_html("./path_that_doesnt_exist.html")?;
        writer.response()
    }

    
    #[api_callback]
    pub fn serve_json_payload(_request: Request, mut writer: ResponseWriter){
        use serde::Serialize;
        #[derive(Serialize)]
        struct MyStruct {
            field1: String,
            field2: i32,
        }
        let my_struct = MyStruct {
            field1: String::from("Hello"),
            field2: 42,
        };
        let json_string = serde_json::to_string(&my_struct).unwrap();
        writer.set_status(HttpStatus::Ok);
        writer.set_header(HttpHeader::ContentType("application/json".to_string()));
        writer.set_body(json_string);
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
    async fn test_process_payload(){
        let port = fetch_port().await;

        let mut server = init_server(port);

        server.add_route("/", "POST", process_payload);

        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });

        tokio::task::yield_now().await;

        let client = reqwest::Client::new();
        let response = client
            .post(format!("http://localhost:{}/", port))
            .body("Hello")
            .send()
            .await;

        let response = response.unwrap().status();
        assert_eq!(response.as_str(), "201")

    }

    #[tokio::test]
    async fn test_redirect(){
        let port = fetch_port().await;

        let mut server = init_server(port);

        server.add_route("/", "GET",process_redirect);
        server.add_route("/1", "GET", serve_get);

        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });

        tokio::task::yield_now().await;

        let status_code = reqwest::get(format!("http://localhost:{port}/"))
        .await
        .unwrap()
        .status();

        // I'm going to /, but the response is coming from /1

        assert_eq!(status_code.as_str(), "200");
    }

    #[tokio::test]
    async fn test_multipart_form_parsing(){
        let port = fetch_port().await;

        let mut server = init_server(port);

        server.add_route("/", "POST", process_multipart_form);

        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });

        tokio::task::yield_now().await;

        let client = reqwest::Client::new();

        let form = reqwest::multipart::Form::new()
            .text("field1", "value1")
            .text("field2", "value2");

        let response = client
            .post(format!("http://localhost:{}/", port))
            .multipart(form)
            .send()
            .await;

        let response = response.unwrap().status();
        assert_eq!(response.as_str(), "200")
    }


    #[tokio::test]
    async fn test_url_encoded_form_parsing(){
        let port = fetch_port().await;

        let mut server = init_server(port);

        server.add_route("/", "POST", process_url_encoded_form);

        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });

        tokio::task::yield_now().await;

        let client = reqwest::Client::new();

        let mut form_data = HashMap::new();
            form_data.insert("key1", "value1");
            form_data.insert("key2", "value2");

        let response = client
            .post(format!("http://localhost:{}/", port))
            .form(&form_data)
            .send()
            .await;

        let response = response.unwrap().status();
        assert_eq!(response.as_str(), "200")
    }

    #[tokio::test]
    async fn test_query_param_parse(){
        let port = fetch_port().await;

        let mut server = init_server(port);

        server.add_route("/", "GET", handle_query_param);

        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });

        tokio::task::yield_now().await;

        let status_code = reqwest::get(format!("http://localhost:{port}/?foo=bar&bar=baz"))
        .await
        .unwrap()
        .status();

        assert_eq!(status_code.as_str(), "200");
    }

    #[tokio::test]
    async fn test_user_agent(){
        let port = fetch_port().await;

        let mut server = init_server(port);

        server.add_route("/", "GET", set_user_agent);

        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });

        tokio::task::yield_now().await;

        let headers = reqwest::get(format!("http://localhost:{port}/"))
        .await
        .unwrap();

        let headers = headers.headers();
        println!("{:?}", headers);
        let user_agent = headers.get("User-Agent").unwrap();
        assert_eq!(user_agent.to_str().unwrap(), "Mustafa")
    }

    #[tokio::test]
    async fn test_file_doesnot_exist_error(){
        let port = fetch_port().await;

        let mut server = init_server(port);

        server.add_route("/", "GET", file_not_exist);

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
    async fn test_http_timeout(){
        let port = fetch_port().await;

        let mut server = init_server(port);

        server.add_route("/slow", "GET", serve_slow);

        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });

        tokio::task::yield_now().await;

        let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(600))
        .build()
        .unwrap();

        let response = client.get(format!("http://localhost:{port}/slow"))
            .send()
            .await;

        match response {
            Ok(_) => panic!("Expected a timeout error"),
            Err(err) => assert!(err.is_timeout(), "Expected a timeout error but got: {}", err),
        }

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
    async fn test_json_reponse(){
        let port = fetch_port().await;
        let mut server = init_server(port);
        server.add_route("/", "GET", serve_json_payload);
        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });
        tokio::task::yield_now().await;
        let resp = reqwest::get(format!("http://localhost:{port}/"))
        .await
        .unwrap();

        let status_code = resp.status();
        assert_eq!(status_code.as_str(), "200");
        let json = resp.text().await.unwrap();
        assert!(json.len() > 0);
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct MyStruct {
            field1: String,
            field2: i32,
        }
        let my_struct: MyStruct = serde_json::from_str(&json).unwrap();
        assert_eq!(my_struct.field1 , "Hello".to_string());
        assert_eq!(my_struct.field2 , 42);
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
        .body("{id: 1}")
        .send()
        .await
        .unwrap()
        .status();
        assert_eq!(status_code.as_str(), "204");
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
    async fn test_serve_put(){
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
        .body(r#"{id: 1}"#)
        .send()
        .await
        .unwrap()
        .status();
        assert_eq!(status_code.as_str(), "204")
    }

    #[tokio::test]
    async fn test_connection_persistence(){
        let port = fetch_port().await;

        let mut server = init_server(port);
        server.add_route("/", "GET", serve_get);
        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });
        tokio::task::yield_now().await;

        // Make the first request
        let client = reqwest::Client::builder()
        .pool_max_idle_per_host(1) // Limit pool to 1 connection
        .build().unwrap();

        // Make the first request
        let resp1 = client.get(format!("http://127.0.0.1:{}", port))
            .header("Connection","keep-alive") // Request for a persistent connection
            .send()
            .await;
        
        println!("Response 1: {}", resp1.unwrap().text().await.unwrap());

        // Make the second request
        let resp2 = client.get(format!("http://127.0.0.1:{}", port))
            .header("Connection","keep-alive") // Ensure persistent connection
            .send()
            .await;

        println!("Response 2: {}", resp2.unwrap().text().await.unwrap());
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