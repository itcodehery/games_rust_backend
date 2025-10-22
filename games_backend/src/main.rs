pub mod models;
pub mod routes;
use crate::models::list_info_model::{Database, FromCSV};
use crate::routes::games;

#[macro_use]
extern crate rocket;

#[get("/")] // route attribute
fn index() -> &'static str {
    "Hello, World!" // request handler
}

// #[get("/get_all")]
// fn give_all_games() -> String {
//     let info = GameInfo::new(
//         String::from("Halo Infinite 2"),
//         Genre::SciFiMilitaryFPS,
//         9.0,
//         GameType::Campaign,
//     );
//     String::from(&info)
// }

#[launch]
fn rocket() -> _ {
    let _ = Database::from_csv(String::from("../dataset/games_dataset/games_dataset.csv"));
    rocket::build().mount(
        "/",
        routes![
            games::get_all_games,
            games::get_game_by_name,
            games::add_game,
            games::delete_game,
        ],
    )
}
