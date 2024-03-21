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
                .nest(
                    "/transaction",
                    Router::new()
                        .route("/", post(user_controllers::create_user_transaction)) // posts a transaction for the user
                        // .route(
                        //     ":transaction_id",
                        //     get(user_controllers::get_user_transaction),
                        // ) // gets a specific transaction that the user has made
                        .route("/all", get(user_controllers::get_user_transactions)), // gets all the transactions that the user has made
                ),
        )
}
