use crate::records::organizations::NewOrganization;
use async_graphql::{Context, Result};
use sqlx::PgPool;

pub async fn sign_up<'a>(ctx: &'a Context<'_>, name: String) -> Result<&'a str> {
    let pg_pool = ctx.data::<PgPool>()?;
    let new_organization = NewOrganization::new(&name)?;
    let organization = new_organization.insert(&pg_pool).await?;
    organization.add_user(&pg_pool, 1).await?;
    Ok("OK")
}
