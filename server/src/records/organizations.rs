use async_graphql::{Error, Result, SimpleObject};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgDone, PgPool};

#[derive(sqlx::FromRow, SimpleObject, Debug, Deserialize, Serialize, Clone)]
pub struct SimpleOrganization {
    pub id: i32,
    pub name: String,
    pub date: DateTime<chrono::Utc>,
}

impl<'a> SimpleOrganization {
    pub async fn add_user(&self, pg_pool: &PgPool, user_id: i32) -> Result<PgDone> {
        match sqlx::query!(
            "INSERT INTO user_organizations(organization_id, user_id) VALUES($1, $2)",
            &self.id,
            user_id
        )
        .execute(pg_pool)
        .await
        {
            Ok(done) => Ok(done),
            Err(error) => {
                println!("{}", error.to_string());
                Err(Error::from("Unable to add user to organization."))
            }
        }
    }
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewOrganization<'a> {
    pub name: &'a str,
}

impl<'a> NewOrganization<'a> {
    pub fn new(name: &'a str) -> Result<Self> {
        if name.len() == 0 {
            return Err(Error::from("Organization name is too short."));
        }

        Ok(NewOrganization { name })
    }

    pub async fn insert(&self, pg_pool: &PgPool) -> Result<SimpleOrganization> {
        match sqlx::query_as!(
            SimpleOrganization,
            "INSERT INTO organizations(name) VALUES($1) RETURNING id, name, date",
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
