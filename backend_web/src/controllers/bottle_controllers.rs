use crate::{
    models::bottle_type_model::{BottleType, NewBottleType},
    server::AppState,
    services::bottle_service,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Json as JsonResponse,
};

pub async fn add_bottle_type(
    state: State<AppState>,
    Json(payload): Json<NewBottleType>,
) -> Result<JsonResponse<BottleType>, (StatusCode, String)> {
    let pool = state.pool.clone();
    let bottle_type = bottle_service::add_bottle_type_if_not_exists(
        &pool,
        payload.manufacturer_id,
        &payload.drink_name,
        payload.points,
    )
    .await
    .map(JsonResponse)
    .map_err(internal_error)?;

    Ok(bottle_type)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
