use std::num::ParseIntError;

use tide::Request;

use result_json::ResultJson;

use crate::result_json;
use crate::user;

//用户API接口
pub async fn find_user_by_id(req:Request<()>) -> tide::Result {
    let m = req.param("id").unwrap().parse::<i32>();
    match m {
        Err(err) => {
            return tide::Result::Ok(result_json::ResultJson::failure(1000,"参数类型错误".to_string(),""));
        }
        _ => {}
    }
    let user = user::find_user(req);
    return tide::Result::Ok(ResultJson::success(Option::Some(user)));
}