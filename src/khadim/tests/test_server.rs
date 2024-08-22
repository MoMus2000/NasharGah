#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread;
    use std::sync::Arc;
    use std::sync::Mutex;

    use crate::khadim::server::Server;

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

        let i = Arc::new(Mutex::new(0));

        let i_c = i.clone();
        
        // Spawn the server in a background task
        let _ = tokio::spawn(async move {
            if let Err(_) = server.serve().await {
                *i_c.lock().unwrap() = 1;
            }
        });

        tokio::task::yield_now().await;

        if *i.lock().unwrap() == 1{
            panic!("ERROR initializing the server !")
        }

        eprintln!("Exiting ..")
    }

}