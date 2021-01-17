use async_graphql::{Error, Result, SimpleObject};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(sqlx::FromRow, SimpleObject, Debug, Deserialize, Serialize, Clone)]
pub struct SimpleProject {
    pub id: i32,
    pub organization_id: i32,
    pub name: String,
    pub date: DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewProject<'a> {
    pub organization_id: i32,
    pub name: &'a str,
}

impl<'a> NewProject<'a> {
    pub fn make(organization_id: i32, name: &'a str) -> Result<Self> {
        if name.len() == 0 {
            return Err(Error::from("Project name is too short."));
        }

        Ok(NewProject {
            organization_id,
            name,
        })
    }

    pub async fn insert(&self, pg_pool: &PgPool) -> Result<SimpleProject> {
        match sqlx::query_as!(
            SimpleProject,
            "INSERT INTO projects (organization_id, name) VALUES ($1, $2) RETURNING id, organization_id, name, date",
            &self.organization_id,
            &self.name
        )
        .fetch_one(pg_pool)
        .await
        {
            Ok(organization) => Ok(organization),
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from("Unable to insert organization in database."))
            }
        }
    }
}
