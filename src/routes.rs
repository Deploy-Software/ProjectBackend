use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use warp::http::{Error, Response};

pub fn playground() -> Result<Response<String>, Error> {
    Response::builder()
        .header("content-type", "text/html")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ))
}
