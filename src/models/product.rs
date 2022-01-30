use serde::Serialize;

use crate::db::IntArray;

#[derive(Queryable, Serialize)]
pub struct Product {
    id: i32,
    name: String,
}

#[derive(Queryable, Serialize, Clone)]
pub struct ProductOption {
    id: i32,
    name: String,
    #[serde(skip_serializing)]
    _product_id: i32,
}

#[derive(Queryable, Serialize)]
pub struct StockUnit {
    id: i32,
    brand: String,
    country: String,
    product_id: i32,
    options: IntArray,
}

impl StockUnit {
    pub fn attach(self, product: Product, product_options: &Vec<ProductOption>) -> StockUnitJson {
        let options = self
            .options
            .iter()
            .filter_map(|n| product_options.iter().find(|option| option.id == *n))
            .map(|option| option.clone())
            .collect();
        StockUnitJson {
            id: self.id,
            brand: self.brand,
            country: self.country,
            product,
            options,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StockUnitJson {
    id: i32,
    brand: String,
    country: String,
    product: Product,
    options: Vec<ProductOption>,
}
