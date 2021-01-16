use async_graphql::{Error, Result, SimpleObject};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(sqlx::FromRow, SimpleObject, Debug, Deserialize, Serialize, Clone)]
pub struct SimpleTarget {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub date: DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewTarget<'a> {
    pub project_id: i32,
    pub name: &'a str,
}

impl<'a> NewTarget<'a> {
    pub fn make(project_id: i32, name: &'a str) -> Result<Self> {
        if name.len() == 0 {
            return Err(Error::from("Target name is too short."));
        }

        Ok(NewTarget { project_id, name })
    }

    pub async fn insert(&self, pg_pool: &PgPool) -> Result<SimpleTarget> {
        match sqlx::query_as!(
            SimpleTarget,
            "INSERT INTO targets (project_id, name) VALUES ($1, $2) RETURNING id, project_id, name, date",
            &self.project_id,
            &self.name,
        )
        .fetch_one(pg_pool)
        .await
        {
            Ok(target) => Ok(target),
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from("Unable to insert target in database."))
            }
        }
    }
}
