use super::User;
use axum::extract::Json;

#[utoipa::path(get, path = "/user", responses((status = OK, body = User)))]
pub async fn get_user() -> Json<User> {
    Json(User {
        id: 1,
        number: None,
    })
}

#[utoipa::path(put, path = "/user", responses((status = OK, body = User)))]
pub async fn put_user(Json(user): Json<User>) -> Json<User> {
    Json(user)
}

#[utoipa::path(post, path = "/user", responses((status = OK, body = User)))]
pub async fn post_user(Json(_user): Json<User>) -> Json<User> {
    Json(User {
        id: 1,
        number: None,
    })
}

#[utoipa::path(patch, path = "/user", responses((status = OK, body = User)))]
pub async fn patch_user() -> Json<User> {
    Json(User {
        id: 1,
        number: None,
    })
}

#[utoipa::path(delete, path = "/user", responses((status = OK, body = User)))]
pub async fn delete_user() -> Json<User> {
    Json(User {
        id: 1,
        number: None,
    })
}
