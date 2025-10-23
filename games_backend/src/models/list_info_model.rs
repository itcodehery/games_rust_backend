use crate::models::game_info_model::GameInfo;
use csv::Reader;
use std::{error::Error, fs::File};

#[derive(Debug, Default)]
pub struct Database {
    vec: Vec<GameInfo>,
}

#[allow(dead_code)]
impl Database {
    fn new(vec: Vec<GameInfo>) -> Self {
        Self { vec }
    }

    pub fn get_all(&self) -> &Vec<GameInfo> {
        &self.vec
    }

    pub fn get_by_name(&self, name: &str) -> Option<&GameInfo> {
        self.vec.iter().find(|game| game.name == name)
    }

    pub fn add_game(&mut self, game: GameInfo) {
        self.vec.push(game);
    }

    pub fn remove_game_by_name(&mut self, game_name: &str) -> bool {
        let old_len = self.vec.len();
        self.vec.retain(|game| game.name != game_name);
        self.vec.len() < old_len
    }
}

pub trait FromCSV<T, F> {
    fn from_csv(file: F) -> Result<T, Box<dyn Error>>;
}

impl FromCSV<Database, String> for Database {
    fn from_csv(file_path: String) -> Result<Database, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let mut rdr = Reader::from_reader(file);

        let mut new_database = Database::default();

        for result in rdr.deserialize() {
            let record: GameInfo = result?;
            println!("{}", String::from(&record));
            new_database.add_game(record);
        }

        Ok(new_database)
    }
}
