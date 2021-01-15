use crate::records::users::NewUser;
use async_graphql::{Context, Result};
use sqlx::PgPool;

pub async fn sign_up<'a>(ctx: &'a Context<'_>, email: String) -> Result<&'a str> {
    let pg_pool = ctx.data::<PgPool>()?;
    let new_user = NewUser::new(&email)?;
    new_user.insert(&pg_pool).await?;
    Ok("OK")
}
