use crate::{controllers::points_controller, server::AppState};
use axum::{Router, routing::get};

pub fn points_routes() -> Router<AppState> {
    println!("points_routes");
    Router::new()
        .route("/points", get(points_controller::get_all_points))
}


