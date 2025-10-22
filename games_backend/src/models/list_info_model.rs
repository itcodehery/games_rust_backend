use crate::models::game_info_model::GameInfo;
use csv::Reader;
use std::{error::Error, fs::File};

#[derive(Debug, Default)]
pub struct Database {
    vec: Vec<GameInfo>,
}

#[allow(dead_code)]
impl Database {
    #[allow(dead_code)]
    fn new(vec: Vec<GameInfo>) -> Self {
        Self { vec }
    }

    #[allow(dead_code)]
    fn add_game(&mut self, game: GameInfo) {
        self.vec.push(game);
    }

    #[allow(dead_code)]
    fn remove_game_by_name(&mut self, game_name: String) {
        let _ = self.vec.pop_if(|x| x.name == game_name);
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
