use crate::{convert, test::progenitor_test};
use serde::{Deserialize, Serialize};
use utoipa::openapi::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

mod verbs;

#[derive(utoipa::ToSchema, Deserialize, Serialize)]
struct User {
    id: i32,
}

#[test]
fn basic_get() {
    let (_router, api): (axum::Router, OpenApi) = OpenApiRouter::new()
        .routes(routes!(verbs::get_user))
        .split_for_parts();
    let yaml = serde_yaml::to_string(&api).unwrap();
    insta::assert_snapshot!("basic-before", &yaml);
    let converted = convert(&yaml).unwrap();
    insta::assert_snapshot!("basic-converted", &converted);
    progenitor_test(&converted);
}

#[test]
fn basic_verbs() {
    let (_router, api): (axum::Router, OpenApi) = OpenApiRouter::new()
        .routes(routes!(
            verbs::get_user,
            verbs::put_user,
            verbs::post_user,
            verbs::patch_user,
            verbs::delete_user
        ))
        .split_for_parts();
    let yaml = serde_yaml::to_string(&api).unwrap();
    insta::assert_snapshot!("verbs-before", &yaml);
    let converted = convert(&yaml).unwrap();
    insta::assert_snapshot!("verbs-converted", &converted);
    progenitor_test(&converted);
}
