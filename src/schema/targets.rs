use crate::{records::targets::NewTarget, AuthToken};
use async_graphql::{Context, Error, Result};
use sqlx::PgPool;

pub async fn new<'a>(ctx: &'a Context<'_>, project_id: i32, name: String) -> Result<&'a str> {
    let pg_pool = ctx.data::<PgPool>()?;
    let _token = match ctx.data_opt::<AuthToken>() {
        Some(token) => token,
        None => {
            return Err(Error::from("No session token found."));
        }
    };
    let new_target = NewTarget::make(project_id, &name)?;
    new_target.insert(&pg_pool).await?;
    Ok("OK")
}
