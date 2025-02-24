use super::UserWithNullableField;
use axum::extract::{Json, Multipart};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
#[allow(unused)]
struct UserForm {
    name: String,
    #[schema(format = Binary, content_media_type = "application/octet-stream")]
    file: String,
}

#[utoipa::path(post,
    path = "/user",
    request_body(content = UserForm, content_type = "multipart/form-data"),
    responses((status = OK, body = UserWithNullableField)))]
pub async fn post_user(_: Multipart) -> Json<UserWithNullableField> {
    Json(UserWithNullableField {
        id: 1,
        name: None,
        field: String::new(),
    })
}
