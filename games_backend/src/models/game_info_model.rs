use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameInfo {
    name: String,
    completed: bool,
    game_type: GameType,
    genre: Genre,
    rating: f64,
}

impl GameInfo {
    pub fn from(
        name: String,
        completed: bool,
        game_type: GameType,
        genre: Genre,
        rating: f64,
    ) -> Self {
        Self {
            name,
            completed,
            game_type,
            genre,
            rating,
        }
    }

    pub fn new(name: String, genre: Genre, rating: f64, game_type: GameType) -> Self {
        Self {
            name,
            completed: false,
            genre,
            rating,
            game_type,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum GameType {
    Campaign,
    Multiplayer,
}

pub enum Genre {
    FirstPersonShooter,
    RPG,
    ThirdPersonAdventure,
    SciFiMilitaryFPS,
    StoryDriven,
    Puzzle,
    Unknown,
}

impl Genre {
    fn from_string(str: &str) -> Self {
        match str {
            "First Person Shooter" => Genre::FirstPersonShooter,
            "RPG" => Genre::RPG,
            "Puzzle" => Genre::Puzzle,
            "Sci-fi Military FPS" => Genre::SciFiMilitaryFPS,
            "Third Person Adventure" => Genre::ThirdPersonAdventure,
            "Story-Driven" => Genre::StoryDriven,
            _ => Genre::Unknown,
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Genre::FirstPersonShooter => "First Person Shooter",
            Genre::RPG => "RPG",
            Genre::Puzzle => "Puzzle",
            Genre::SciFiMilitaryFPS => "Sci-fi Military FPS",
            Genre::ThirdPersonAdventure => "Third Person Adventure",
            Genre::StoryDriven => "Story-Driven",
            Genre::Unknown => "Unknown",
        }
    }
}

impl<'de> Deserialize<'de> for Genre {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Genre::from_string(&s))
    }
}

impl Serialize for Genre {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(s)
    }
}
