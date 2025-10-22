use crate::models::game_info_model::GameInfo;
use crate::models::list_info_model::Database;
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;
use std::sync::Mutex;

#[get("/games")]
pub async fn get_all_games(db: &State<Mutex<Database>>) -> Json<Vec<GameInfo>> {
    let db_lock = db.lock().unwrap();
    Json(db_lock.get_all().clone())
}

#[get("/games/<name>")]
pub async fn get_game_by_name(
    name: String,
    db: &State<Mutex<Database>>,
) -> Result<Json<GameInfo>, Status> {
    let db_lock = db.lock().unwrap();
    if let Some(game) = db_lock.get_by_name(&name).cloned() {
        Ok(Json(game))
    } else {
        Err(Status::NotFound)
    }
}

#[post("/games", format = "json", data = "<game>")]
pub async fn add_game(game: Json<GameInfo>, db: &State<Mutex<Database>>) -> Status {
    let mut db_lock = db.lock().unwrap();
    db_lock.add_game(game.into_inner());
    Status::Created
}

#[delete("/games/<name>")]
pub async fn delete_game(name: String, db: &State<Mutex<Database>>) -> Status {
    let mut db_lock = db.lock().unwrap();
    if db_lock.remove_by_name(&name) {
        Status::Ok
    } else {
        Status::NotFound
    }
}
