
mod user;
mod result_json;
use result_json::ResultJson;
mod user_controller;
use lazy_static::lazy_static;
use rbatis::rbatis::Rbatis;
lazy_static! {
    static ref RB:Rbatis = Rbatis::new();
}

#[async_std::main]
async fn main() ->tide::Result<()> {
    async_std::task::block_on(async { RB.link("mysql://root:123456@localhost:3306/test").await.unwrap() });
    let mut app = tide::new();
    app.at("/user/:id").get(user_controller::find_user_by_id);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
