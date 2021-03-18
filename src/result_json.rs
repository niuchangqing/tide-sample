use std::error::Error;
use std::ptr::null;

use serde::{Deserialize, Serialize};
use tide::{new, StatusCode};
use tide::http::Mime;
use std::str::FromStr;

#[derive(Deserialize,Serialize)]
pub struct ResultJson<T> {
    pub code:i32,
    pub message:String,
    pub data:Option<T>,
}

impl <'a,T:Serialize+Deserialize<'a>> ResultJson<T> {

    //成功
    pub fn success(data:Option<T>) -> tide::Response {
        let result = ResultJson::new(0,"成功".to_string(),data);
        let mut response = get_response();
        response.set_body(result.to_string());
        response
    }

    //失败
    pub fn failure(code:i32,message:String,data:T) ->tide::Response {
        let mut result = ResultJson::new(code,message,Option::Some(data));
        result.data = Option::None;
        let mut response = get_response();
        response.set_body(result.to_string());
        response
    }

    fn new(code:i32,message:String,data:Option<T>) -> ResultJson<T> {
        let result = ResultJson{code:code,message:message,data:data};
        return result;
    }

    pub fn to_string(&self) -> String{
        let s = serde_json::to_string(self).unwrap();
        s
    }
}

fn get_response() ->tide::Response {
    let mut response = tide::Response::new(StatusCode::Ok);
    let mime = Mime::from_str("application/json;charset=utf-8").unwrap();
    response.set_content_type(mime);
    response
}