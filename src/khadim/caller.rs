use std::pin::Pin;
use std::future::Future;
use std::boxed::Box;

use super::response::{ResponseWriter, Request};
use super::http_status::HttpStatus;
use crate::api_callback;

pub type AsyncReturn = Result<Pin<Box<dyn Future<Output = String> + Send>>, Box<dyn std::error::Error>>;

#[api_callback]
pub fn default_404(_request: Request, mut writer: ResponseWriter){
    writer.set_status(HttpStatus::NotFound);
    writer.response()
}

#[api_callback]
pub fn default_500(_request: Request, mut writer: ResponseWriter){
    writer.set_status(HttpStatus::InternalServerError);
    writer.response()
}
