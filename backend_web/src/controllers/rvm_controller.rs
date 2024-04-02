use crate::models::rvm_model::{NewRVM, UpdateRVM, RVM};
use crate::{server::AppState, services::rvm_service};
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, response::Json}; // Import the Manufacturers model

pub async fn get_all_rvms(state: State<AppState>) -> Result<Json<Vec<RVM>>, (StatusCode, String)> {
    let pool = state.pool.clone();
    rvm_service::fetch_all_rvms(&pool) // Fetch rvms
        .await
        .map(|rvms| Json(rvms)) // Map Ok variant to Json
        .map_err(internal_error)
}

pub async fn create_rvm(
    state: State<AppState>,
    Json(payload): Json<NewRVM>,
) -> Result<Json<RVM>, (StatusCode, String)> {
    let pool = state.pool.clone();
    rvm_service::create_rvm_if_not_exists(
        &pool,
        &payload.location,
        &payload.latitude,
        &payload.longitude,
    )
    .await
    .map(|rvm| Json(rvm))
    .map_err(internal_error)
}

pub async fn get_rvm_by_id(
    state: State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Option<RVM>>, (StatusCode, String)> {
    let pool = state.pool.clone();
    rvm_service::get_rvm_by_id(&pool, id)
        .await
        .map(|rvm| Json(rvm))
        .map_err(internal_error)
}

pub async fn update_rvm_status(
    state: State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateRVM>,
) -> Result<Json<RVM>, (StatusCode, String)> {
    let pool = state.pool.clone();
    rvm_service::update_rvm_status(&pool, id, payload.status)
        .await
        .map(|rvm| Json(rvm))
        .map_err(internal_error)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
