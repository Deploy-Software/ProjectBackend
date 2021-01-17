use async_graphql::Schema;
use async_graphql_warp::Response;
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
    let static_files = {
        // Development
        #[cfg(debug_assertions)]
        {
            format!("{}/../client/build", env!("CARGO_MANIFEST_DIR"))
        }

        // Production
        #[cfg(not(debug_assertions))]
        {
            format!("{}/../client/dist", env!("CARGO_MANIFEST_DIR"))
        }
    };

    let pg_pool: PgPool = db_connection().await.expect("Database connection failed.");
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Database migrations failed");

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(pg_pool)
        .finish();

    let graphql_post = warp::path("graphql")
        .and(warp::header::optional::<String>("token"))
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

    let index = warp::path::end().map(|| routes::respond("/".to_string()));
    let graphql_playground = warp::path("playground")
        .and(warp::get())
        .map(|| routes::playground());
    let examples = warp::path("static").and(warp::fs::dir(static_files));
    let catch_all = warp::path!(String).map(|path| routes::respond(path));

    let routes = index
        .or(graphql_playground)
        .or(graphql_post)
        .or(examples)
        .or(catch_all);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
