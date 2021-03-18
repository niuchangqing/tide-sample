use serde::{Deserialize,Serialize};
use tide::Request;
use super::result_json;

//用户结构体
#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct User {
    pub id:i64,
    pub name:String,
    pub id_card:Option<String>,
}

impl User {
    pub fn new(id:i64,name:String) -> User{
        let user = User{id:id,name:name,id_card:Option::None};
        user
    }

    pub fn set_id_card(&mut self, id_card:String) {
        self.id_card = Option::Some(id_card);
    }
}


pub fn find_user(req:Request<()>) -> User{
    let user = User::new(1,String::from("张三"));
    user
}