use diesel::table;

table! {
    products (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    product_options (id) {
        id -> Int4,
        name -> Text,
        product_id -> Int4,
    }
}

table! {
    stock_units (id) {
        id -> Int4,
        brand -> Text,
        country -> Text,
        product_id -> Int4,
        options -> Text,
    }
}

joinable!(stock_units -> products (product_id));

allow_tables_to_appear_in_same_query!(products, stock_units);
allow_tables_to_appear_in_same_query!(stock_units, product_options);
