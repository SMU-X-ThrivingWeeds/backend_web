use crate::{
    models::bottle_type_model::{BottleType, NewBottleType},
    server::AppState,
    services::{bottle_service, manufacturer_service},
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::Json as JsonResponse,
};

pub async fn add_bottle_type(
    state: State<AppState>,
    Path(manufacturer_id): Path<i64>,
    Json(payload): Json<NewBottleType>,
) -> Result<JsonResponse<BottleType>, (StatusCode, String)> {
    let pool = state.pool.clone();
    // check if manufacturer exists
    manufacturer_service::fetch_manufacturer_by_id(&pool, manufacturer_id)
        .await
        .map_err(internal_error)?
        .ok_or((StatusCode::NOT_FOUND, "Manufacturer not found".to_string()))?;

    let bottle_type = bottle_service::add_bottle_type_if_not_exists(
        &pool,
        manufacturer_id,
        &payload.drink_name,
        &payload.barcode,
    )
    .await
    .map(JsonResponse)
    .map_err(internal_error)?;
    return Ok(bottle_type);
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
