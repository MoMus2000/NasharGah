use std::pin::Pin;
use std::future::Future;
use std::boxed::Box;

pub type AsyncReturn = Pin<Box<dyn Future<Output = String> + Send>>;

