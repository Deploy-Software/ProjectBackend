#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::correctness)]
#![deny(clippy::style)]
#![deny(clippy::complexity)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use async_graphql::{Data, Schema};
use async_graphql_warp::{graphql_subscription_with_data, Response};
use sqlx::postgres::PgPool;
use std::convert::Infallible;
use std::env;
use warp::Filter;

mod records;
mod routes;
mod schema;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct AuthToken(String);
pub struct QueryRoot;
pub struct MutationRoot;
pub struct SubscriptionRoot;

pub async fn db_connection() -> Result<PgPool> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL NOT FOUND");
    Ok(PgPool::connect(&database_url).await?)
}

#[tokio::main]
async fn main() {
    let pg_pool: PgPool = db_connection().await.expect("Database connection failed.");
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Database migrations failed");

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(pg_pool)
        .finish();

    let graphql_post = warp::header::optional::<String>("token")
        .and(async_graphql_warp::graphql(schema.clone()))
        .and_then(
            |token,
             (schema, mut request): (
                Schema<QueryRoot, MutationRoot, SubscriptionRoot>,
                async_graphql::Request,
            )| async move {
                if let Some(token) = token {
                    request = request.data(AuthToken(token));
                }
                let resp = schema.execute(request).await;
                Ok::<_, Infallible>(Response::from(resp))
            },
        );

    let graphql_playground = warp::path::end()
        .and(warp::get())
        .map(|| routes::playground());

    let routes = graphql_subscription_with_data(
        schema,
        Some(|value| {
            #[derive(serde_derive::Deserialize)]
            struct Payload {
                token: String,
            }

            if let Ok(payload) = serde_json::from_value::<Payload>(value) {
                let mut data = Data::default();
                data.insert(AuthToken(payload.token));
                Ok(data)
            } else {
                Err("Token is required".into())
            }
        }),
    )
    .or(graphql_playground)
    .or(graphql_post);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
