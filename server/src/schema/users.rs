use crate::{
    records::users::personal_data::NewPersonalData, records::users::SimpleUser, AuthToken,
};
use async_graphql::{Context, Error, Result};
use sqlx::PgPool;

pub async fn add_personal_data<'a>(
    ctx: &'a Context<'_>,
    name: String,
    job_title: Option<String>,
) -> Result<&'a str> {
    let pg_pool = ctx.data::<PgPool>()?;
    let token = match ctx.data_opt::<AuthToken>() {
        Some(token) => token,
        None => {
            return Err(Error::from("No session token found."));
        }
    };
    let user = SimpleUser::from_session_token(&pg_pool, &token.0).await?;
    let new_personal_data = NewPersonalData::new(name, job_title)?;
    new_personal_data.insert(&pg_pool, user.id).await?;
    Ok("OK")
}
