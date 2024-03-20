use crate::{controllers::rvm_controller, server::AppState};
use axum::{
    routing::{get, post},
    Router,
};

pub fn rvm_routes() -> Router<AppState> {
    println!("rvm_routes");
    // This route gets all the rows for the rvms
    Router::new().nest(
        "/rvm/location",
        Router::new()
            .route("/", post(rvm_controller::create_rvm))
            .route("/all", get(rvm_controller::get_all_rvms)),
    )
}
