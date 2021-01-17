use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use isomorphic_app::App;
use warp::http::{Error, Response};

const HTML_PLACEHOLDER: &str = "#HTML_INSERTED_HERE_BY_SERVER#";
const STATE_PLACEHOLDER: &str = "#INITIAL_STATE_JSON#";

static INDEX_HTML: &str = include_str!("./index.html");

pub fn respond(path: String) -> Result<Response<String>, Error> {
    let app = App::new(1001, path);
    let state = app.store.borrow();

    let html = format!("{}", INDEX_HTML);
    let html = html.replacen(HTML_PLACEHOLDER, &app.render().to_string(), 1);
    let html = html.replacen(STATE_PLACEHOLDER, &state.to_json(), 1);

    Response::builder()
        .header("content-type", "text/html")
        .body(html)
}

pub fn playground() -> Result<Response<String>, Error> {
    Response::builder()
        .header("content-type", "text/html")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ))
}
