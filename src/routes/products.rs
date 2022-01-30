use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket::{get, put};
use serde_json::{json, Value};
use std::collections::HashSet;

use crate::db;
use crate::db::products::{FindProductOptions, UpdateStockUnitData};
use crate::db::DatabaseConnection;
use crate::models::product::{ProductOption, StockUnitJson};

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/products")]
pub async fn get_products(db_connection: DatabaseConnection) -> Result<Json<Vec<StockUnitJson>>> {
    let products = db::products::get_products(&db_connection).await?;
    Ok(Json(products))
}

#[get("/products/<product_id>")]
pub async fn get_stock_unit(
    product_id: i32,
    db_connection: DatabaseConnection,
) -> Result<Json<StockUnitJson>> {
    let products = db::products::get_stock_unit(product_id as i32, &db_connection).await?;
    Ok(Json(products))
}

#[put("/products/<product_id>", format = "json", data = "<stock_unit>")]
pub async fn update_stock_unit(
    product_id: i32,
    stock_unit: Json<UpdateStockUnitData>,
    db_connection: DatabaseConnection,
) -> Result<Value> {
    let options =
        db::products::get_product_options_by_stock_unit_id(product_id, &db_connection).await?;
    let request_options: HashSet<i32> = stock_unit.options.iter().cloned().collect();

    if !request_options.is_subset(&options) {
        return Ok(json!({
            "status": "error",
            "reason": "Wrong options"
        }));
    }

    let result = db::products::update_stock_unit(product_id, stock_unit.0, &db_connection).await?;
    Ok(json!({
        "status": "success",
        "changed": result
    }))
}

#[get("/product_options?<params..>")]
pub async fn get_product_options(
    params: FindProductOptions,
    db_connection: DatabaseConnection,
) -> Result<Json<Vec<ProductOption>>> {
    let options = db::products::find_product_options(&db_connection, &params).await?;
    Ok(Json(options))
}
