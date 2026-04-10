use std::collections::HashMap;
use sqlx::{Row, FromRow};
use sqlx::postgres::PgRow;
use sqlx::types::Json;

use crate::types::{Categories, Pattern, Rule, TextGenerator};

impl<'r> FromRow<'r, PgRow> for TextGenerator {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(TextGenerator {
            name: row.try_get("name")?,
            categories: Categories(row.try_get::<Json<HashMap<String, Vec<String>>>,_>("categories")?.0),
            patterns: row.try_get::<Json<Vec<Pattern>>,_>("patterns")?.0,
            ruleset: row.try_get::<Json<Vec<Rule>>,_>("ruleset")?.0,
        })
    }
}

