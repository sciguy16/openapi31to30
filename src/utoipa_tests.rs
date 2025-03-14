use crate::{convert, test::progenitor_test};
use serde::{Deserialize, Serialize};
use utoipa::openapi::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

mod multipart;
mod nullable;
mod path;
mod verbs;

#[derive(utoipa::ToSchema, Deserialize, Serialize)]
struct User {
    id: i32,
    /// nullable field with docs
    number: Option<i32>,
}

#[derive(utoipa::ToSchema, Deserialize, Serialize)]
struct UserWithNullableField {
    id: i32,
    name: Option<String>,
    /// field with some documentation
    field: String,
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

#[test]
fn paths() {
    let (_router, api): (axum::Router, OpenApi) = OpenApiRouter::new()
        .routes(routes!(
            path::get_user,
            path::put_user,
            path::post_user,
            path::patch_user,
            path::delete_user
        ))
        .split_for_parts();
    let yaml = serde_yaml::to_string(&api).unwrap();
    insta::assert_snapshot!("paths-before", &yaml);
    let converted = convert(&yaml).unwrap();
    insta::assert_snapshot!("paths-converted", &converted);
    progenitor_test(&converted);
}

#[test]
fn nullable() {
    let (_router, api): (axum::Router, OpenApi) = OpenApiRouter::new()
        .routes(routes!(nullable::put_user, nullable::post_user))
        .split_for_parts();
    let yaml = serde_yaml::to_string(&api).unwrap();
    insta::assert_snapshot!("nullable-before", &yaml);
    let converted = convert(&yaml).unwrap();
    insta::assert_snapshot!("nullable-converted", &converted);
    progenitor_test(&converted);
}

#[test]
fn multipart() {
    let (_router, api): (axum::Router, OpenApi) = OpenApiRouter::new()
        .routes(routes!(multipart::post_user))
        .split_for_parts();
    let yaml = serde_yaml::to_string(&api).unwrap();
    insta::assert_snapshot!("multipart-before", &yaml);
    let converted = convert(&yaml).unwrap();
    insta::assert_snapshot!("multipart-converted", &converted);
    // https://github.com/oxidecomputer/progenitor/issues/518
    // progenitor_test(&converted);
}
