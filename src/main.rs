use dotenv::dotenv;
use product_edit;

#[rocket::main]
async fn main() {
    dotenv().ok();
    let _ = product_edit::rocket().launch().await;
}
