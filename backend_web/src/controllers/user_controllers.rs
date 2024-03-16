use crate::{models::user_model::User, server::AppState, services::user_service};
use axum::{debug_handler, extract::State, http::StatusCode, response::Json};

#[debug_handler]
pub async fn get_users(state: State<AppState>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    println!("get_users");
    let pool = state.pool.clone();
    println!("pool: {:?}", pool);
    user_service::fetch_users(&pool)
        .await
        .map(|users| Json(users)) // Map Ok variant to Json
        .map_err(internal_error)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
