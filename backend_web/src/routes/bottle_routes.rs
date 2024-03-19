use crate::{controllers::bottle_controllers, server::AppState};
use axum::{
    routing::{get, post},
    Router,
};

pub fn bottle_routes() -> Router<AppState> {
    println!("bottle_routes");
    Router::new().route("/bottle_type", post(bottle_controllers::add_bottle_type))
}
