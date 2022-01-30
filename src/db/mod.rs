pub mod products;
pub mod schema;

use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Text;
use diesel::{deserialize, serialize};
use itertools::Itertools;
use rocket_sync_db_pools::{database, diesel};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::Write;
use std::num::ParseIntError;
use std::ops::Deref;
use std::str::FromStr;

#[database("database")]
pub struct DatabaseConnection(diesel::SqliteConnection);

#[derive(AsExpression, Debug, Deserialize, Serialize, FromSqlRow, Default, Clone)]
#[sql_type = "Text"]
pub struct IntHashSet(HashSet<i32>);

#[derive(AsExpression, Debug, Deserialize, Serialize, FromSqlRow, Default, Clone)]
#[sql_type = "Text"]
pub struct IntArray(Vec<i32>);

impl Deref for IntArray {
    type Target = Vec<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<DB> FromSql<Text, DB> for IntArray
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let text = String::from_sql(bytes)?;
        let options: Result<Vec<i32>, ParseIntError> =
            text.split(",").map(|s| i32::from_str(s.trim())).collect();
        Ok(IntArray(options?))
    }
}

impl<DB> ToSql<Text, DB> for IntArray
where
    DB: Backend,
    String: ToSql<Text, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        self.0.iter().join(",").to_sql(out)
    }
}
