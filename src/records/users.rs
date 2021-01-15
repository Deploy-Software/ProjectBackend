use async_graphql::{Error, Result, SimpleObject};
use chrono::DateTime;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(sqlx::FromRow, SimpleObject, Debug, Deserialize, Serialize, Clone)]
pub struct SimpleUser {
    pub id: i32,
    pub email: String,
    pub date: DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewUser<'a> {
    pub email: &'a str,
}

impl<'a> NewUser<'a> {
    pub fn new(email: &'a str) -> Result<Self> {
        let re = match Regex::new(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)") {
            Ok(re) => re,
            Err(error) => {
                println!("{}", error.to_string());
                return Err(Error::from("Email regex could not be compiled."));
            }
        };

        if !re.is_match(email) {
            return Err(Error::from("Email is not valid."));
        }

        Ok(NewUser { email })
    }

    pub async fn insert(&self, pg_pool: &PgPool) -> Result<SimpleUser> {
        Ok(sqlx::query_as!(
            SimpleUser,
            "INSERT INTO users(email) VALUES($1) RETURNING id, email, date",
            &self.email
        )
        .fetch_one(pg_pool)
        .await?)
    }
}
