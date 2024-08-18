mod khadim;
use crate::khadim::server::Server;

pub fn callback_function(){
    println!("Hey there 123");
}

#[tokio::main]
async fn main() {
    let port = "8080";
    let address = "127.0.0.1";
    let mut server = Server::new(port, address).await.unwrap();
    server.add_route("/", "GET", callback_function);
    server.listen().await;
}
