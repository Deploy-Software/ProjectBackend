use async_graphql::{Error, Result, SimpleObject};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(sqlx::FromRow, SimpleObject, Debug, Deserialize, Serialize, Clone)]
pub struct SimpleTask {
    pub id: i32,
    pub target_id: i32,
    pub name: String,
    pub about: Option<String>,
    pub created_by: i32,
    pub date: DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewTask<'a> {
    target_id: i32,
    name: &'a str,
    about: Option<String>,
    created_by: i32,
}

impl<'a> NewTask<'a> {
    pub fn make(
        target_id: i32,
        name: &'a str,
        about: Option<String>,
        created_by: i32,
    ) -> Result<Self> {
        if name.len() == 0 {
            return Err(Error::from("Target name is too short."));
        }

        Ok(NewTask {
            target_id,
            name,
            about,
            created_by,
        })
    }

    pub async fn insert(&self, pg_pool: &PgPool) -> Result<SimpleTask> {
        match sqlx::query_as!(
            SimpleTask,
            "INSERT INTO tasks (target_id, name, about, created_by) VALUES ($1, $2, $3, $4) RETURNING id, target_id, name, about, created_by, date",
            &self.target_id,
            &self.name,
            self.about: Option<String>,
            self.created_by
        )
        .fetch_one(pg_pool)
        .await
        {
            Ok(target) => Ok(target),
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from("Unable to insert task in database."))
            }
        }
    }
}
