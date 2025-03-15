use crate::db::DbPool;
use crate::models::{NewUser, UpdateUser, User};
use crate::repository::UserRepository;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn list_users(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    match UserRepository::find_all(&pool) {
        Ok(users) => Ok(Json(users)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn get_user(
    Path(id): Path<i32>,
    State(pool): State<DbPool>,
) -> Result<Json<User>, (StatusCode, String)> {
    match UserRepository::find_by_id(id, &pool) {
        Ok(user) => Ok(Json(user)),
        Err(diesel::result::Error::NotFound) => {
            Err((StatusCode::NOT_FOUND, "Пользователь не найден".to_string()))
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn create_user(
    State(pool): State<DbPool>,
    Json(new_user): Json<NewUser>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match UserRepository::create(new_user, &pool) {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn update_user(
    Path(id): Path<i32>,
    State(pool): State<DbPool>,
    Json(user): Json<UpdateUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    match UserRepository::update(id, user, &pool) {
        Ok(user) => Ok(Json(user)),
        Err(diesel::result::Error::NotFound) => {
            Err((StatusCode::NOT_FOUND, "Пользователь не найден".to_string()))
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn delete_user(
    Path(id): Path<i32>,
    State(pool): State<DbPool>,
) -> Result<StatusCode, (StatusCode, String)> {
    match UserRepository::delete(id, &pool) {
        Ok(rows) => {
            if rows > 0 {
                Ok(StatusCode::NO_CONTENT)
            } else {
                Err((StatusCode::NOT_FOUND, "Пользователь не найден".to_string()))
            }
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
