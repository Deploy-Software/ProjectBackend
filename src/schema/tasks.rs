use crate::{records::tasks::NewTask, records::users::SimpleUser, AuthToken};
use async_graphql::{Context, Error, Result};
use sqlx::PgPool;

pub async fn new<'a>(
    ctx: &'a Context<'_>,
    target_id: i32,
    name: String,
    about: Option<String>,
) -> Result<&'a str> {
    let pg_pool = ctx.data::<PgPool>()?;
    let token = match ctx.data_opt::<AuthToken>() {
        Some(token) => token,
        None => {
            return Err(Error::from("No session token found."));
        }
    };
    let user = SimpleUser::from_session_token(&pg_pool, &token.0).await?;
    let new_task = NewTask::make(target_id, &name, about, user.id)?;
    new_task.insert(&pg_pool).await?;
    Ok("OK")
}
