use crate::{
    records::targets::{NewActivity, NewComment, NewTarget},
    records::users::SimpleUser,
    AuthToken,
};
use async_graphql::{Context, Error, Result};
use sqlx::PgPool;

pub async fn new<'a>(
    ctx: &'a Context<'_>,
    project_id: i32,
    name: String,
    about: Option<String>,
    value: String,
) -> Result<&'a str> {
    let pg_pool = ctx.data::<PgPool>()?;
    let token = match ctx.data_opt::<AuthToken>() {
        Some(token) => token,
        None => {
            return Err(Error::from("No session token found."));
        }
    };
    let user = SimpleUser::from_session_token(&pg_pool, &token.0).await?;
    let new_target = NewTarget::make(project_id, &name, about, user.id)?;
    let target = new_target.insert(&pg_pool).await?;
    target.insert_value(&pg_pool, &value).await?;
    Ok("OK")
}

pub async fn new_comment<'a>(
    ctx: &'a Context<'_>,
    target_id: i32,
    text: String,
) -> Result<&'a str> {
    let pg_pool = ctx.data::<PgPool>()?;
    let token = match ctx.data_opt::<AuthToken>() {
        Some(token) => token,
        None => {
            return Err(Error::from("No session token found."));
        }
    };
    let user = SimpleUser::from_session_token(&pg_pool, &token.0).await?;
    let new_comment = NewComment::make(target_id, &text, user.id)?;
    new_comment.insert(&pg_pool).await?;
    Ok("OK")
}

pub async fn new_activity<'a>(
    ctx: &'a Context<'_>,
    target_id: i32,
    text: String,
) -> Result<&'a str> {
    let pg_pool = ctx.data::<PgPool>()?;
    let token = match ctx.data_opt::<AuthToken>() {
        Some(token) => token,
        None => {
            return Err(Error::from("No session token found."));
        }
    };
    let user = SimpleUser::from_session_token(&pg_pool, &token.0).await?;
    let new_comment = NewActivity::make(target_id, &text, user.id)?;
    new_comment.insert(&pg_pool).await?;
    Ok("OK")
}
