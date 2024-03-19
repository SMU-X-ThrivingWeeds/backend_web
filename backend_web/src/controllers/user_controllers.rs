use crate::{
    models::{
        points_model::Points, transaction_model::UserTransactions, user_bottles_model::UserBottles,
        user_model::User,
    },
    server::AppState,
    services::user_service,
};
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;

#[debug_handler]
pub async fn get_users(state: State<AppState>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let pool = state.pool.clone();
    user_service::fetch_users(&pool)
        .await
        .map(|users| Json(users)) // Map Ok variant to Json
        .map_err(internal_error)
}

#[debug_handler]
pub async fn get_user(
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<User>, (StatusCode, String)> {
    let pool = state.pool.clone();

    let user_uuid = match Uuid::parse_str(&id) {
        Ok(user_uuid) => {
            println!("Parsed UUID: {}", user_uuid);
            user_uuid
        }
        Err(e) => {
            eprintln!("Error parsing UUID: {}", e);
            return Err(internal_error(e));
        }
    };

    user_service::fetch_user(&pool, user_uuid)
        .await
        .map(|user| Json(user)) // Map Ok variant to Json
        .map_err(internal_error)
}

#[debug_handler]
pub async fn get_user_points(
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Points>, (StatusCode, String)> {
    let pool = state.pool.clone();

    let user_uuid = match Uuid::parse_str(&id) {
        Ok(user_uuid) => {
            println!("Parsed UUID: {}", user_uuid);
            user_uuid
        }
        Err(e) => {
            eprintln!("Error parsing UUID: {}", e);
            return Err(internal_error(e));
        }
    };

    user_service::fetch_user_points(&pool, user_uuid)
        .await
        .map(|points| Json(points)) // Map Ok variant to Json
        .map_err(internal_error)
}

#[debug_handler]
pub async fn get_user_bottles(
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<UserBottles>>, (StatusCode, String)> {
    let pool = state.pool.clone();

    let user_uuid = match Uuid::parse_str(&id) {
        Ok(user_uuid) => {
            println!("Parsed UUID: {}", user_uuid);
            user_uuid
        }
        Err(e) => {
            eprintln!("Error parsing UUID: {}", e);
            return Err(internal_error(e));
            // Convert the uuid::Error into PgDatabaseError
        }
    };

    user_service::fetch_user_bottles(&pool, user_uuid)
        .await
        .map(|user_bottles| Json(user_bottles)) // Map Ok variant to Json
        .map_err(internal_error)
}

#[debug_handler]
pub async fn get_user_transactions(
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<UserTransactions>>, (StatusCode, String)> {
    let pool = state.pool.clone();

    let user_uuid = match Uuid::parse_str(&id) {
        Ok(user_uuid) => {
            println!("Parsed UUID: {}", user_uuid);
            user_uuid
        }
        Err(e) => {
            eprintln!("Error parsing UUID: {}", e);
            return Err(internal_error(e));
        }
    };

    user_service::fetch_user_transactions(&pool, user_uuid)
        .await
        .map(|user_transactions| Json(user_transactions)) // Map Ok variant to Json
        .map_err(internal_error)
}

// #[debug_handler]
// pub async fn post_user_transaction(
//     state: State<AppState>,
//     Path(id): Path<String>,
// ) -> Result<Json<UserTransactions>, (StatusCode, String)> {
//     let pool = state.pool.clone();

//     let user_uuid = match Uuid::parse_str(&id) {
//         Ok(user_uuid) => {
//             println!("Parsed UUID: {}", user_uuid);
//             user_uuid
//         }
//         Err(e) => {
//             eprintln!("Error parsing UUID: {}", e);
//             return Err(internal_error(e));
//         }
//     };

//     user_service::create_user_transaction(&pool, user_uuid)
//         .await
//         .map(|user_transactions| Json(user_transactions)) // Map Ok variant to Json
//         .map_err(internal_error)
// }

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
