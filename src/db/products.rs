use crate::db::schema::{product_options, products, stock_units};
use crate::db::{DatabaseConnection, IntArray};
use crate::diesel::JoinOnDsl;
use crate::models::product::{Product, ProductOption, StockUnit, StockUnitJson};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use rocket::form::FromForm;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(FromForm, Default, Debug)]
pub struct FindProductOptions {
    pub product_id: i32,
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "stock_units"]
pub struct UpdateStockUnitData {
    pub brand: String,
    pub country: String,
    pub options: IntArray,
}

pub async fn find_product_options(
    db_connection: &DatabaseConnection,
    params: &FindProductOptions,
) -> QueryResult<Vec<ProductOption>> {
    let product_id = params.product_id;
    db_connection
        .run(move |conn| {
            product_options::table
                .select(product_options::all_columns)
                .filter(product_options::product_id.eq(product_id))
                .get_results::<ProductOption>(conn)
        })
        .await
}

pub async fn get_products(db_connection: &DatabaseConnection) -> QueryResult<Vec<StockUnitJson>> {
    let options = db_connection
        .run(move |conn| {
            product_options::table
                .select(product_options::all_columns)
                .get_results::<ProductOption>(conn)
        })
        .await?;

    let stock_units = db_connection
        .run(move |conn| {
            stock_units::table
                .inner_join(products::table)
                .select((stock_units::all_columns, products::all_columns))
                .get_results::<(StockUnit, Product)>(conn)
        })
        .await?;

    Ok(stock_units
        .into_iter()
        .map(|(stock_unit, product)| stock_unit.attach(product, &options))
        .collect())
}

pub async fn get_stock_unit(
    product_id: i32,
    db_connection: &DatabaseConnection,
) -> QueryResult<StockUnitJson> {
    let options = db_connection
        .run(move |conn| {
            product_options::table
                .select(product_options::all_columns)
                .get_results::<ProductOption>(conn)
        })
        .await?;

    let (stock_unit, product) = db_connection
        .run(move |conn| {
            stock_units::table
                .inner_join(products::table)
                .select((stock_units::all_columns, products::all_columns))
                .filter(stock_units::id.eq(product_id))
                .get_result::<(StockUnit, Product)>(conn)
        })
        .await?;

    Ok(stock_unit.attach(product, &options))
}

pub async fn get_product_options_by_stock_unit_id(
    id: i32,
    db_connection: &DatabaseConnection,
) -> QueryResult<HashSet<i32>> {
    let result = db_connection
        .run(move |conn| {
            stock_units::table
                .inner_join(
                    product_options::table
                        .on(product_options::product_id.eq(stock_units::product_id)),
                )
                .select(product_options::id)
                .filter(stock_units::id.eq(id))
                .get_results::<i32>(conn)
        })
        .await?;

    Ok(HashSet::from_iter(result))
}

pub async fn update_stock_unit(
    id: i32,
    stock_unit: UpdateStockUnitData,
    db_connection: &DatabaseConnection,
) -> QueryResult<usize> {
    db_connection
        .run(move |conn| {
            diesel::update(stock_units::table.find(id))
                .set(stock_unit)
                .execute(conn)
        })
        .await
}
