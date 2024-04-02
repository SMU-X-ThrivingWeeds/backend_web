use crate::{controllers::rvm_controller, server::AppState};
use axum::{
    routing::{get, patch, post},
    Router,
};

pub fn rvm_routes() -> Router<AppState> {
    println!("rvm_routes");
    // This route gets all the rows for the rvms
    Router::new().nest(
        "/rvm",
        Router::new()
            .route("/", post(rvm_controller::create_rvm))
            .route("/all", get(rvm_controller::get_all_rvms))
            .nest(
                "/:id",
                Router::new()
                    .route("/", get(rvm_controller::get_rvm_by_id))
                    .route("/status", patch(rvm_controller::update_rvm_status)),
            ),
    )
}
