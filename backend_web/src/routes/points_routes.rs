use crate::{controllers::points_controller, server::AppState};
use axum::{routing::get, Router};

pub fn points_routes() -> Router<AppState> {
    println!("points_routes");
    // This route gets all the points for the leaderboard
    Router::new().route("/points", get(points_controller::get_all_points))
}
