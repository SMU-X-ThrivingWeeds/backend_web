use crate::{
    models::bottle_type_model::{BottleType, CheckBottles, CheckBottlesInput, NewBottleType},
    server::AppState,
    services::{bottle_service, manufacturer_service},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};

pub async fn add_bottle_type(
    state: State<AppState>,
    Path(manufacturer_id): Path<i64>,
    Json(payload): Json<NewBottleType>,
) -> Result<Json<BottleType>, (StatusCode, String)> {
    let pool = state.pool.clone();
    // check if manufacturer exists
    manufacturer_service::fetch_manufacturer_by_id(&pool, manufacturer_id)
        .await
        .map_err(internal_error)?
        .ok_or((
            StatusCode::NOT_FOUND,
            format!("Manufacturer of id: {} is not found", manufacturer_id),
        ))?;

    let bottle_type = bottle_service::add_bottle_type_if_not_exists(
        &pool,
        manufacturer_id,
        &payload.drink_name,
        &payload.barcode,
    )
    .await
    .map(|bottle_type| Json(bottle_type))
    .map_err(internal_error)?;
    return Ok(bottle_type);
}

pub async fn check_bottles(
    state: State<AppState>,
    Json(payload): Json<CheckBottlesInput>,
) -> Result<Json<CheckBottles>, (StatusCode, String)> {
    let pool = state.pool.clone();
    let barcode_arr = payload.barcodes_to_check.clone();
    let check_bottles = bottle_service::check_bottles(&pool, barcode_arr)
        .await
        .map(|check_bottles| Json(check_bottles))
        .map_err(internal_error)?;
    return Ok(check_bottles);
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
