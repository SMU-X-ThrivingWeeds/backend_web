use axum::{extract::State, http::StatusCode, response::Json};
use crate::{server::AppState, services::points_service};
use crate::models::points_model::Points;

pub async fn get_all_points(state: State<AppState>) -> Result<Json<Vec<Points>>, (StatusCode, String)> {
    let pool = state.pool.clone();
    let points = points_service::fetch_all_points(&pool)
        .await
        .map(|points| Json(points)) // Convert to JSON
        .map_err(internal_error)?;

    Ok(points)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
