use crate::{controllers::bottle_transaction_controller, server::AppState};
use axum::{
    routing::get,
    Router,
};

pub fn bottle_transaction_routes() -> Router<AppState> {
    println!("bottle_transaction_routes");
    // This route gets all the rows for the manufacturer
    Router::new().nest(
        "/transactions",
        Router::new()
            .route("/all", get(bottle_transaction_controller::get_all_transactions))
            .route("/count", get(bottle_transaction_controller::get_drink_counts))

    )
}
