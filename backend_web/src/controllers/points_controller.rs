use axum::{extract::State, http::StatusCode, response::Json};
use crate::{server::AppState, services::points_service};

pub async fn get_all_points(state: State<AppState>) -> Result<Json<Vec<i64>>, (StatusCode, String)> {
    let pool = state.pool.clone();
    let points = points_service::fetch_all_points(&pool)
        .await
        .map(|points| {
            points.iter().map(|point| point.points).collect::<Vec<i64>>() // Extract points field from Points struct
        })
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
