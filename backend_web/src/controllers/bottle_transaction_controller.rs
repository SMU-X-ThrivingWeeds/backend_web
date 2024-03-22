    use crate::models::bottle_transaction_model::{BottleCount, BottleTransactions};
    use crate::{server::AppState, services::bottle_transaction_service};
    use axum::{extract::State, http::StatusCode, response::Json};

    pub async fn get_all_transactions(
        state: State<AppState>,
    ) -> Result<Json<Vec<BottleTransactions>>, (StatusCode, String)> {
        let pool = state.pool.clone();
        bottle_transaction_service::fetch_all_transactions(&pool) // Fetch manufacturers instead of users
            .await
            .map(|transactions| Json(transactions)) // Map Ok variant to Json
            .map_err(internal_error)
    }

    pub async fn get_drink_counts(
        state: State<AppState>,
    ) -> Result<Json<Vec<BottleCount>>, (StatusCode, String)> {
        let pool = state.pool.clone();
        bottle_transaction_service::get_drink_counts(&pool)
            .await
            .map(|counts| Json(counts))
            .map_err(internal_error)

        // Ok(counts)
    }

    fn internal_error<E>(err: E) -> (StatusCode, String)
    where
        E: std::error::Error,
    {
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
