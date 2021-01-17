use crate::{records::organizations::NewOrganization, records::users::SimpleUser, AuthToken};
use async_graphql::{Context, Error, Result};
use sqlx::PgPool;

pub async fn new<'a>(ctx: &'a Context<'_>, name: String) -> Result<&'a str> {
    let pg_pool = ctx.data::<PgPool>()?;
    let token = match ctx.data_opt::<AuthToken>() {
        Some(token) => token,
        None => {
            return Err(Error::from("No session token found."));
        }
    };
    let user = SimpleUser::from_session_token(&pg_pool, &token.0).await?;
    let new_organization = NewOrganization::new(&name)?;
    let organization = new_organization.insert(&pg_pool).await?;
    organization.add_user(&pg_pool, user.id).await?;
    Ok("OK")
}
