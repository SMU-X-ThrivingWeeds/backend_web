use crate::{controllers::user_controllers, server::AppState};
use axum::{routing::get, Router};

pub fn user_routes() -> Router<AppState> {
    println!("user_routes");
    Router::new()
        .route("/users", get(user_controllers::get_users))
        .nest(
            "/user/:id",
            Router::new()
                .route("/", get(user_controllers::get_user))
                .route("/points", get(user_controllers::get_user_points))
                .route("/bottles", get(user_controllers::get_user_bottles)), // .route("/transactions", get(user_controllers::get_user_transactions))
        )
}
