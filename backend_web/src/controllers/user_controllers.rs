use crate::{
    models::{
        points_model::Points,
        transaction_model::{NewUserTransactions, UserTransactions},
        user_bottles_model::UserBottles,
        user_model::User,
    },
    server::AppState,
    services::{bottle_service, manufacturer_service, user_service},
};
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::json;
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

#[debug_handler]
pub async fn create_user_transaction(
    state: State<AppState>,
    Path(id): Path<String>,
    payload: Json<Vec<NewUserTransactions>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
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

    // Iterate through each UserTransaction in the payload
    for transaction in payload.iter() {
        let manufacturer_name = transaction.manufacturer_name.clone();
        let drink_name = transaction.drink_name.clone();
        let barcode = transaction.barcode.clone();

        // Create new manufacturer if it doesn't exist
        let manufacturer =
            manufacturer_service::create_manufacturer_if_not_exists(&pool, &manufacturer_name)
                .await
                .map(|manufacturer| Json(manufacturer))
                .map_err(internal_error)?;

        // Create new bottle type if it doesn't exist
        let bottle_type = bottle_service::add_bottle_type_if_not_exists(
            &pool,
            manufacturer.id,
            &drink_name,
            &barcode,
        )
        .await
        .map(|bottle_type| Json(bottle_type))
        .map_err(internal_error)?;

        // Get latest transaction
        let latest_transaction = user_service::fetch_latest_user_transaction(&pool, user_uuid)
            .await
            .map(|latest_transaction| Json(latest_transaction))
            .map_err(internal_error)?;

        let bottle_type_id = bottle_type.id;
        let quantity = transaction.bottle_quantity;
        let transaction_group_id = latest_transaction.transaction_group + 1;
        let points = bottle_type.points * quantity;

        // Insert each transaction into the database
        user_service::create_user_transaction(
            &pool,
            user_uuid,
            bottle_type_id,
            quantity,
            transaction_group_id,
            points,
        )
        .await
        .map_err(internal_error)?;

        // get current user points
        let user_points = user_service::fetch_user_points(&pool, user_uuid)
            .await
            .map_err(internal_error)?;
        let total_points = user_points.points + points;

        // Update user points
        user_service::update_user_points(&pool, user_uuid, total_points)
            .await
            .map_err(internal_error)?;
    }

    Ok(Json(
        json!({"message": "Transaction(s) created successfully"}),
    ))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
