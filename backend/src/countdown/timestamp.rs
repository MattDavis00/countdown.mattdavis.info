use chrono::{DateTime, Utc};
use rocket::request::FromParam;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone)]
pub struct Timestamp(pub DateTime<Utc>);

impl Deref for Timestamp {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl FromSql for Timestamp {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let timestamp =
            DateTime::<Utc>::from_str(value.as_str()?).map_err(|_e| FromSqlError::InvalidType)?;
        Ok(Timestamp(timestamp))
    }
}

// Implement FromParam for Timestamp/DateTime<Utc>
impl<'r> FromParam<'r> for Timestamp {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        let date = param.parse::<DateTime<Utc>>().map_err(|_| "Invalid date")?;
        Ok(Timestamp(date))
    }
}
