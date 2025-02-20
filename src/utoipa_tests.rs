use crate::{convert, test::progenitor_test};
use axum::extract::Json;
use utoipa::openapi::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(utoipa::ToSchema, serde::Serialize)]
struct User {
    id: i32,
}

#[utoipa::path(get, path = "/user", responses((status = OK, body = User)))]
async fn get_user() -> Json<User> {
    Json(User { id: 1 })
}

#[test]
fn basic_get() {
    let (_router, api): (axum::Router, OpenApi) = OpenApiRouter::new()
        .routes(routes!(get_user))
        .split_for_parts();
    let yaml = serde_yaml::to_string(&api).unwrap();
    insta::assert_snapshot!("basic-before", &yaml);
    let converted = convert(&yaml).unwrap();
    insta::assert_snapshot!("basic-converted", &converted);
    progenitor_test(&converted);
}
