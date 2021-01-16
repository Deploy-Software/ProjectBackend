use async_graphql::{Error, Result, SimpleObject};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgDone, PgPool};

#[derive(sqlx::FromRow, SimpleObject, Debug, Deserialize, Serialize, Clone)]
pub struct SimpleTarget {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub about: Option<String>,
    pub created_by: i32,
    pub date: DateTime<chrono::Utc>,
}

impl<'a> SimpleTarget {
    pub async fn insert_value(&self, pg_pool: &PgPool, value: &'a str) -> Result<PgDone> {
        match sqlx::query!(
            "INSERT INTO target_values (target_id, value) VALUES ($1, $2)",
            &self.id,
            &value,
        )
        .execute(pg_pool)
        .await
        {
            Ok(done) => Ok(done),
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from("Unable to insert target value in database."))
            }
        }
    }
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewTarget<'a> {
    project_id: i32,
    name: &'a str,
    about: Option<String>,
    created_by: i32,
}

impl<'a> NewTarget<'a> {
    pub fn make(
        project_id: i32,
        name: &'a str,
        about: Option<String>,
        created_by: i32,
    ) -> Result<Self> {
        if name.len() == 0 {
            return Err(Error::from("Target name is too short."));
        }

        Ok(NewTarget {
            project_id,
            name,
            about,
            created_by,
        })
    }

    pub async fn insert(&self, pg_pool: &PgPool) -> Result<SimpleTarget> {
        match sqlx::query_as!(
            SimpleTarget,
            "INSERT INTO targets (project_id, name, about, created_by) VALUES ($1, $2, $3, $4) RETURNING id, project_id, name, about, created_by, date",
            &self.project_id,
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
                Err(Error::from("Unable to insert target in database."))
            }
        }
    }
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewComment<'a> {
    target_id: i32,
    text: &'a str,
    created_by: i32,
}

impl<'a> NewComment<'a> {
    pub fn make(target_id: i32, text: &'a str, created_by: i32) -> Result<Self> {
        if text.len() == 0 {
            return Err(Error::from("Comment is too short."));
        }

        Ok(NewComment {
            target_id,
            text,
            created_by,
        })
    }

    pub async fn insert(&self, pg_pool: &PgPool) -> Result<PgDone> {
        match sqlx::query!(
            "INSERT INTO target_comments (target_id, text, created_by) VALUES ($1, $2, $3)",
            &self.target_id,
            &self.text,
            self.created_by
        )
        .execute(pg_pool)
        .await
        {
            Ok(done) => Ok(done),
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from("Unable to insert comment in database."))
            }
        }
    }
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewActivity<'a> {
    target_id: i32,
    text: &'a str,
    created_by: i32,
}

impl<'a> NewActivity<'a> {
    pub fn make(target_id: i32, text: &'a str, created_by: i32) -> Result<Self> {
        if text.len() == 0 {
            return Err(Error::from("Activity is too short."));
        }

        Ok(NewActivity {
            target_id,
            text,
            created_by,
        })
    }

    pub async fn insert(&self, pg_pool: &PgPool) -> Result<PgDone> {
        match sqlx::query!(
            "INSERT INTO target_activity (target_id, text, created_by) VALUES ($1, $2, $3)",
            &self.target_id,
            &self.text,
            self.created_by
        )
        .execute(pg_pool)
        .await
        {
            Ok(done) => Ok(done),
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from("Unable to insert activity in database."))
            }
        }
    }
}
