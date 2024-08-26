use tokio::net::TcpListener;
use tokio_rustls::rustls::{ClientConfig, RootCertStore};
use tokio_rustls::TlsConnector;

// https://crates.io/crates/tokio-rustls

pub struct TLS{
    pub listener : TcpListener
}

impl TLS{
    fn new(){
    }
}