use crate::{controllers::bottle_controllers, server::AppState};
use axum::{routing::get, Router};

pub fn bottle_type_routes() -> Router<AppState> {
    println!("bottle_type_routes");
    // This route gets all the rows for the manufacturer
    Router::new().nest(
        "/bottle_type",
        Router::new().route("/check", get(bottle_controllers::check_bottles)),
    )
}
