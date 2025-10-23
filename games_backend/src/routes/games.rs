use crate::models::game_info_model::GameInfo;
use crate::models::list_info_model::Database;
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;
use std::sync::Mutex;

// State is not passed during the request btw
#[get("/games")]
pub async fn get_all_games(db: &State<Mutex<Database>>) -> Json<Vec<GameInfo>> {
    let db_lock = db.lock().unwrap();
    Json(db_lock.get_all().clone())
}

// Mutex is a smart-pointer for Mutual Exclusion in multiple threads
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
