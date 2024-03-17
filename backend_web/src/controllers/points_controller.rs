use axum::{debug_handler, extract::State, http::StatusCode, response::Json};
use crate::{server::AppState, services::points_service};

#[debug_handler]
pub async fn get_all_points(state: State<AppState>) -> Result<Json<Vec<i32>>, (StatusCode, String)> {
    println!("fetching points data");
    let pool = state.pool.clone();
    println!("pool: {:?}", pool);
    points_service::fetch_all_points(&pool)
        .await
        .map(|points| Json(points)) // Map Ok variant to Json
        .map_err(internal_error)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
