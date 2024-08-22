#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::time::Duration;
    use tokio::net::TcpListener;

    use crate::khadim::server::Server;

    const START : u8 = 0;
    const EXIT : u8 = 1;

    async fn init_server() -> u16{
        
        let address = "127.0.0.1:0";
        let listener = TcpListener::bind(&address).await.expect("Failed to bind to address");
        let port = listener.local_addr().expect("Failed to get local address").port();

        drop(listener);

        let mut server = match Server::new(&port.to_string(), "127.0.0.1") {
            Ok(server) => server,
            Err(e) => {
                panic!("Failed to create server: {}", e);
            }
        };

        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });

        tokio::time::sleep(Duration::from_secs(1)).await;

        port
    }

    #[tokio::test]
    async fn test_404(){
        let address = "127.0.0.1:0";
        let listener = TcpListener::bind(&address).await.expect("Failed to bind to address");
        let port = listener.local_addr().expect("Failed to get local address").port();

        drop(listener);

        let mut server = match Server::new(&port.to_string(), "127.0.0.1") {
            Ok(server) => server,
            Err(e) => {
                panic!("Failed to create server: {}", e);
            }
        };

        let _ = tokio::spawn(async move {
            server.serve().await.unwrap();
        });

        tokio::time::sleep(Duration::from_secs(1)).await;

        let status_code = reqwest::get(format!("http://localhost:{port}/"))
        .await
        .unwrap()
        .status();

        assert_eq!(status_code.as_str(), "404")

    }

    #[tokio::test]
    async fn test_init() {
        let port = "8080";
        let address = "127.0.0.1";
        let mut server = match Server::new(port, address) {
            Ok(server) => server,
            Err(e) => {
                eprintln!("Failed to create server: {}", e);
                return;
            }
        };

        let i = Arc::new(Mutex::new(START));

        let i_c = i.clone();
        
        // Spawn the server in a background task
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