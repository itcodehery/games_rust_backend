pub mod models;
use crate::models::game_info_model::{GameInfo, GameType, Genre};

#[macro_use]
extern crate rocket;

#[get("/")] // route attribute
fn index() -> &'static str {
    "Hello, World!" // request handler
}

#[get("/get_all")]
fn give_all_games() -> String {
    let _ = GameInfo::new(
        String::from("Hello Kitty Video Game"),
        Genre::Puzzle,
        9.0,
        GameType::Campaign,
    );
    String::new()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, give_all_games])
}
