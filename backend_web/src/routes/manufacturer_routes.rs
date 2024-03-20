use crate::{controllers::manufacturer_controller, server::AppState};
use axum::{
    routing::{get, post},
    Router,
};

pub fn manufacturer_routes() -> Router<AppState> {
    println!("manufacturer_routes");
    // This route gets all the rows for the manufacturer
    Router::new().nest(
        "/manufacturer",
        Router::new()
            .route("/", post(manufacturer_controller::create_manufacturer))
            .route("/all", get(manufacturer_controller::get_all_manufacturers)),
    )
}
