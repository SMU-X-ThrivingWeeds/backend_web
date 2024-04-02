use crate::{controllers::points_controller, server::AppState};
use axum::{routing::get, Json, Router};

pub fn health_routes() -> Router<AppState> {
    println!("points_routes");
    // This route gets all the points for the leaderboard
    Router::new().route("/health", get(Json("200")))
}
