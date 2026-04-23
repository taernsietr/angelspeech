use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};
use crate::types::*;

/*
impl<'r, DB: sqlx::Database> sqlx::Decode<'r, DB> for TextGenerator {
    fn decode(value: <DB as sqlx::Database>::ValueRef<'r>) -> Result<TextGenerator, sqlx::error::BoxDynError> {
        todo!()
    }
}
*/

impl<'r> FromRow<'r, PgRow> for TextGenerator {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let name: String = row.try_get("name")?;
        let categories: Categories = serde_json::from_value(row.try_get("categories")?).unwrap();
        let patterns: Patterns = serde_json::from_value(row.try_get("patterns")?).unwrap();
        let ruleset: Ruleset = serde_json::from_value(row.try_get("ruleset")?).unwrap();

        Ok(TextGenerator::new(name, categories, patterns, ruleset))
    }
}

