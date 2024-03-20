use crate::{controllers::manufacturer_controller, server::AppState};
use axum::{routing::get, Router};

pub fn manufacturer_routes() -> Router<AppState> {
    println!("manufacturer_routes");
    // This route gets all the rows for the manufacturer
    Router::new().route("/manufacturer", get(manufacturer_controller::get_all_manufacturers))
}
