pub mod models;
pub mod routes;
use crate::models::list_info_model::{Database, FromCSV};
use crate::routes::games;

#[macro_use]
extern crate rocket;

#[get("/")] // route attribute
fn index() -> &'static str {
    "Hello from the Server!!" // request handler
}

use std::sync::Mutex;

#[launch]
fn rocket() -> _ {
    let db = Database::from_csv(String::from("dataset/games_dataset/games_dataset.csv"))
        .expect("failed to load database");

    rocket::build().manage(Mutex::new(db)).mount(
        "/",
        routes![index, games::get_all_games, games::get_game_by_name,],
    )
}
