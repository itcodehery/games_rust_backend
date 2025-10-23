use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Display;

fn deserialize_yes_no<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match s {
        "Yes" => Ok(true),
        "No" => Ok(false),
        "-" => Ok(false),
        _ => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Str(s),
            &"Yes, No, or -",
        )),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameInfo {
    pub name: String,
    #[serde(deserialize_with = "deserialize_yes_no")]
    pub completed: bool,
    #[serde(rename = "type")]
    pub game_type: GameType,
    #[serde(rename = "Genre")]
    pub genre: Genre,
    pub rating: f64,
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

impl From<&GameInfo> for String {
    fn from(value: &GameInfo) -> Self {
        format!(
            "{{name : {}, completed: {}, genre: {}, rating: {}, game_type: {}}}",
            value.name, value.completed, value.genre, value.rating, value.game_type
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameType {
    Campaign,
    Multiplayer,
    #[serde(rename = "Campaign + Multiplayer")]
    Both,
}

// impl From<GameType> for String {
//     fn from(value: GameType) -> Self {
//         match value {
//             GameType::Campaign => "Campaign".to_string(),
//             GameType::Multiplayer => "Multiplayer".to_string(),
//         }
//     }
// }

impl Display for GameType {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            GameType::Campaign => "Campaign",
            GameType::Multiplayer => "Multiplayer",
            GameType::Both => "Campaign + Multiplayer",
        };
        Ok(write!(f, "{}", x)?)
    }
}

#[derive(Debug, Clone)]
pub enum Genre {
    FirstPersonShooter,
    RPG,
    ThirdPersonAdventure,
    SciFiMilitaryFPS,
    StoryDriven,
    Puzzle,
    Unknown,
}

impl Display for Genre {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Genre::FirstPersonShooter => "First Person Shooter",
            Genre::RPG => "RPG",
            Genre::Puzzle => "Puzzle",
            Genre::SciFiMilitaryFPS => "Sci-fi Military FPS",
            Genre::ThirdPersonAdventure => "Third Person Adventure",
            Genre::StoryDriven => "Story-Driven",
            Genre::Unknown => "Unknown",
        };
        Ok(write!(f, "{}", x)?)
    }
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
        serializer.serialize_str(s.as_str())
    }
}
