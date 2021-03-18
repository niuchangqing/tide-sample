mod user;
mod result_json;
use result_json::ResultJson;
mod user_controller;
#[async_std::main]
async fn main() ->tide::Result<()> {
    let mut app = tide::new();
    app.at("/user/:id").get(user_controller::find_user_by_id);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
