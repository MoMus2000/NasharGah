#![allow(dead_code)]
use std::collections::HashMap;
use std::pin::Pin;
use std::future::Future;

use super::response::{Request, ResponseWriter};
use super::caller::{default_404, default_500};

type AsyncReturn = Result<Pin<Box<dyn Future<Output = String> + Send>>, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
pub struct Router{
    pub router_elem_mapper: HashMap<String, RouterElement>,
    pub not_found_func: Option<fn(Request, ResponseWriter) -> AsyncReturn >,
    pub internal_server_error: Option<fn(Request, ResponseWriter) -> AsyncReturn >
}

impl Router {
    pub fn new() -> Router{
        Router{
            router_elem_mapper: HashMap::new(),
            not_found_func: Some(default_404),
            internal_server_error: Some(default_500)
        }
    }

    pub fn add_route(&mut self, path: &'static str, method: &'static str, callback_function: fn(Request, ResponseWriter) -> AsyncReturn ) -> bool{
        let element = RouterElement{
            path,
            callback_function,
            method
        };
        if self.router_elem_mapper.contains_key(path){
            return false;
        }
        else{
            self.router_elem_mapper.insert(path.to_string(), element);
        }
        true
    }

    pub fn fetch_func(&self, path: &str, method: &str) -> Option<fn(Request, ResponseWriter) -> AsyncReturn>{
        if self.router_elem_mapper.contains_key(path){
            let re = match self.router_elem_mapper.get(path){
                Some(r) => r,
                None => return None
            };
            if re.method == method {
                return Some(re.callback_function)
            }
        }
        None
    }

}

#[derive(Clone, Debug)]
pub struct RouterElement {
    pub path: &'static str,
    pub callback_function: fn(Request, ResponseWriter) -> AsyncReturn,
    pub method: &'static str
}