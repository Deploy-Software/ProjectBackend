use crate::{records::projects::NewProject, AuthToken};
use async_graphql::{Context, Error, Result};
use sqlx::PgPool;

pub async fn new<'a>(ctx: &'a Context<'_>, organization_id: i32, name: String) -> Result<&'a str> {
    let pg_pool = ctx.data::<PgPool>()?;
    let _token = match ctx.data_opt::<AuthToken>() {
        Some(token) => token,
        None => {
            return Err(Error::from("No session token found."));
        }
    };
    let new_project = NewProject::make(organization_id, &name)?;
    new_project.insert(&pg_pool).await?;
    Ok("OK")
}
