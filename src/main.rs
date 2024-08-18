mod khadim;
use crate::khadim::server::Server;

#[tokio::main]
async fn main() {
    let port = "8080";
    let address = "127.0.0.1";
    let mut server = Server::new(port, address).await.unwrap();
    server.listen().await;
}
