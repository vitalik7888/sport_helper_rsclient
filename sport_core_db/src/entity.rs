use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type ID = u64;

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub id: ID,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: DateTime<Utc>, // FIXME only date
    pub gender: String,
    pub height: i16,
}

impl Person {
    pub fn new(id: ID, first_name: String, last_name: String, birth_date: DateTime<Utc>, gender: String, height: i16) -> Self { 
        Self { id, first_name, last_name, birth_date, gender, height } 
    }
}

impl Default for Person {
    fn default() -> Self {
        Self { 
            id: 0,
            first_name: String::from(""),
            last_name: String::from(""),
            birth_date: DateTime::default(),
            gender: String::from(""),
            height: 0
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Exercise {
    pub id: ID,
    pub name: String,
    pub description: String,
}

impl Exercise {
    pub fn new(id: ID, name: String, description: String) -> Self { Self { id, name, description } }
}

impl Default for Exercise {
    fn default() -> Self {
        Self { id: 0, name: String::from(""), description: String::from("") }
    }
}
