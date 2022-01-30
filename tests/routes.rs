use product_edit::rocket;
use rocket::http::Status;

#[test]
pub fn get_products() {
    use rocket::local::blocking::Client;

    let client = Client::tracked(rocket()).unwrap();
    let response = client.get("/products").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
