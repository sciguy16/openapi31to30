use super::User;
use axum::extract::{Json, Path};

#[utoipa::path(get, path = "/user/{id}", responses((status = OK, body = User)))]
pub async fn get_user(Path(_id): Path<i64>) -> Json<User> {
    Json(User {
        id: 1,
        number: None,
    })
}

#[utoipa::path(put, path = "/user/{id}", responses((status = OK, body = User)))]
pub async fn put_user(
    Path(_id): Path<i64>,
    Json(user): Json<User>,
) -> Json<User> {
    Json(user)
}

#[utoipa::path(post, path = "/user/{id}", responses((status = OK, body = User)))]
pub async fn post_user(
    Path(_id): Path<i64>,
    Json(_user): Json<User>,
) -> Json<User> {
    Json(User {
        id: 1,
        number: None,
    })
}

#[utoipa::path(patch, path = "/user/{id}", responses((status = OK, body = User)))]
pub async fn patch_user(Path(_id): Path<i64>) -> Json<User> {
    Json(User {
        id: 1,
        number: None,
    })
}

#[utoipa::path(delete, path = "/user/{id}", responses((status = OK, body = User)))]
pub async fn delete_user(Path(_id): Path<i64>) -> Json<User> {
    Json(User {
        id: 1,
        number: None,
    })
}
