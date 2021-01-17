use async_graphql::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgDone, PgPool};

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewPersonalData {
    pub name: String,
    pub job_title: Option<String>,
}

impl<'a> NewPersonalData {
    pub fn new(name: String, job_title: Option<String>) -> Result<Self> {
        if name.len() == 0 {
            return Err(Error::from("Name is too short."));
        }
        let job_title: Option<String> = match job_title {
            Some(job) => {
                if job.len() == 0 {
                    return Err(Error::from("Job title is too short."));
                }
                Some(job)
            }
            None => None,
        };
        Ok(NewPersonalData { name, job_title })
    }

    pub async fn insert(&self, pg_pool: &PgPool, user_id: i32) -> Result<PgDone> {
        match sqlx::query!(
            "INSERT INTO user_personal_data(user_id, name, job_title) VALUES($1, $2, $3)",
            user_id,
            &self.name,
            &self.job_title: Option<String>,
        )
        .execute(pg_pool)
        .await
        {
            Ok(done) => Ok(done),
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from(
                    "Unable to insert user personal data in database.",
                ))
            }
        }
    }
}
