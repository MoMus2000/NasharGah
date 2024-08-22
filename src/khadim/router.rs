#![allow(dead_code)]
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::future::Future;

use super::response::{Request, ResponseWriter};
use super::http_method::HttpMethod;
use super::caller::{default_404, default_500};

type AsyncReturn = Result<Pin<Box<dyn Future<Output = String> + Send>>, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
pub struct Router{
    pub router_elem_mapper: HashMap<Route, RouterElement>,
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
        let path = path.to_string();

        let method = match method {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => panic!("Unexpected HTTP method"),
        };

        let route = Route::new(path, method);

        if self.router_elem_mapper.contains_key(&route){
            return false;
        }
        
        let element = RouterElement{
            callback_function,
        };

        self.router_elem_mapper.insert(route, element);

        true
    }

    pub fn fetch_func(&self, path: &str, method: &str) -> Option<fn(Request, ResponseWriter) -> AsyncReturn>{
        let path = path.to_string();

        let method = match method {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => panic!("Unexpected HTTP method"),
        };

        let route = Route::new(path, method);

        if self.router_elem_mapper.contains_key(&route){
            let re = match self.router_elem_mapper.get(&route){
                Some(r) => r,
                None => return None
            };
            return Some(re.callback_function)
        }
        None
    }

}

#[derive(Clone, Debug)]
pub struct RouterElement {
    pub callback_function: fn(Request, ResponseWriter) -> AsyncReturn,
}

// Switch hashmap string with the route struct
#[derive(Debug, Clone)]
pub struct Route{
    pub path: String,
    pub method: HttpMethod
}

impl Route{
    fn new(path: String, method: HttpMethod) -> Self{
        Self { path, method }
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Route) -> bool {
        self.path == other.path && self.method.to_string() == other.method.to_string()
    }
}

impl Eq for Route{}

impl Hash for Route {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.method.to_string().hash(state);
    }
}
