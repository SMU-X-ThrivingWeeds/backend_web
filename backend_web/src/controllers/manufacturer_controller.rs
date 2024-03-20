use axum::{extract::State, http::StatusCode, response::Json};
use crate::{server::AppState, services::manufacturer_service};
use crate::models::manufacturer_model::Manufacturers; // Import the Manufacturers model


pub async fn get_all_manufacturers(state: State<AppState>) -> Result<Json<Vec<Manufacturers>>, (StatusCode, String)> {
    let pool = state.pool.clone();
    manufacturer_service::fetch_all_manufacturers(&pool) // Fetch manufacturers instead of users
        .await
        .map(|manufacturers| Json(manufacturers)) // Map Ok variant to Json
        .map_err(internal_error)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
