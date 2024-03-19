use crate::{controllers::user_controllers, server::AppState};
use axum::{
    routing::{get, post},
    Router,
};

pub fn user_routes() -> Router<AppState> {
    println!("user_routes");
    Router::new()
        .route("/users", get(user_controllers::get_users)) // gets all the users
        .nest(
            "/user/:id",
            Router::new()
                .route("/", get(user_controllers::get_user))
                .route("/points", get(user_controllers::get_user_points)) // gets the points of the user
                .route("/bottles", get(user_controllers::get_user_bottles)) // gets all the bottles that the user has deposited
                .route(
                    "/transactions",
                    get(user_controllers::get_user_transactions),
                ), // gets all the transactions that the user has made
                   // .route("/transaction", post(user_controllers::post_user_transaction)) // posts a transaction for the user
        )
}
