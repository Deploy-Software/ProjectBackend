use crate::{MutationRoot, MyToken, QueryRoot, SubscriptionRoot};
use async_graphql::{Context, Object, Subscription};
use futures::{stream, Stream};

#[Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<MyToken>().map(|token| token.0.as_str())
    }
}

#[Object]
impl MutationRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<MyToken>().map(|token| token.0.as_str())
    }
}

#[Subscription]
impl SubscriptionRoot {
    async fn values(&self, ctx: &Context<'_>) -> async_graphql::Result<impl Stream<Item = i32>> {
        if ctx.data_unchecked::<MyToken>().0 != "123456" {
            return Err("Forbidden".into());
        }
        Ok(stream::once(async move { 10 }))
    }
}
