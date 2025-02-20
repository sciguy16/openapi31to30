use super::UserWithNullableField;
use axum::extract::Json;

#[utoipa::path(put,
    path = "/user",
    responses((status = OK, body = UserWithNullableField)))]
pub async fn put_user(
    Json(user): Json<UserWithNullableField>,
) -> Json<UserWithNullableField> {
    Json(user)
}

#[utoipa::path(post,
    path = "/user",
    responses((status = OK, body = UserWithNullableField)))]
pub async fn post_user(
    Json(_user): Json<UserWithNullableField>,
) -> Json<UserWithNullableField> {
    Json(UserWithNullableField { id: 1, name: None })
}
