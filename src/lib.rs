use crate::db::DatabaseConnection;
use diesel_migrations::embed_migrations;
use rocket::fairing::AdHoc;
use rocket::{catchers, routes, Build, Config, Rocket};

mod db;
mod models;
mod routes;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!("migrations");

    let connection = DatabaseConnection::get_one(&rocket)
        .await
        .expect("database connection");
    connection
        .run(|conn| embedded_migrations::run(conn))
        .await
        .expect("diesel migrations");

    rocket
}

pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/api",
            routes![
                routes::products::get_products,
                routes::products::get_product_options,
                routes::products::get_stock_unit,
                routes::products::update_stock_unit,
            ],
        )
        .register(
            "/",
            catchers![
                routes::catchers::not_found,
                routes::catchers::internal_error,
            ],
        )
        .attach(AdHoc::config::<Config>())
        .attach(DatabaseConnection::fairing())
        .attach(AdHoc::on_ignite("Database Migrations", run_migrations))
}
