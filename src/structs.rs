use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub type_1: String,
    pub type_2: String,
    pub total: u32,
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub sp_attack: u32,
    pub sp_defense: u32,
    pub speed: u32,
    pub generation: u32,
    pub legendary: bool
}

impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {} | Gen: {} | Name: {}", self.id, self.generation, self.name)
    }
}