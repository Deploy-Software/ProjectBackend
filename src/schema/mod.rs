use crate::{AuthToken, MutationRoot, QueryRoot, SubscriptionRoot};
use async_graphql::{Context, Object, Subscription};
use futures::{stream, Stream};

mod authorization;
mod organizations;

#[Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<AuthToken>().map(|token| token.0.as_str())
    }
}

#[Object]
impl MutationRoot {
    async fn sign_up<'a>(
        &self,
        ctx: &'a Context<'_>,
        email: String,
    ) -> async_graphql::Result<&'a str> {
        authorization::sign_up(ctx, email).await
    }

    async fn new_organization<'a>(
        &self,
        ctx: &'a Context<'_>,
        name: String,
    ) -> async_graphql::Result<&'a str> {
        organizations::sign_up(ctx, name).await
    }
}

#[Subscription]
impl SubscriptionRoot {
    async fn values(&self, ctx: &Context<'_>) -> async_graphql::Result<impl Stream<Item = i32>> {
        if ctx.data_unchecked::<AuthToken>().0 != "123456" {
            return Err("Forbidden".into());
        }
        Ok(stream::once(async move { 10 }))
    }
}
