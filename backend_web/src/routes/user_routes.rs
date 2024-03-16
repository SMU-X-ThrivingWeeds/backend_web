use crate::{controllers::user_controllers, server::AppState};
use axum::{routing::get, Router};

pub fn user_routes() -> Router<AppState> {
    println!("user_routes");
    Router::new().route("/users", get(user_controllers::get_users))
}
