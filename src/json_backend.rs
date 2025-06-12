// Crates //
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Result; // Json Parsing Wrapper

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub rating: u8,
    pub times_played: u8,
    pub last_playthrough: String,
    pub notes: String
}

// Read a parse JSON from text file into a vector 
pub fn reading_json() -> io::Result<Vec<Game>> // Result is wrapped around incase there is an error returned
{
    let file = File::open("src/GameLog.txt")?; // Gets file contents
    let metadata = fs::metadata("src/GameLog.txt")?; // Gets the information about the file (Like length for example)

    let reader = BufReader::new(file);
    let game: Vec<Game> = serde_json::from_reader(reader)?; // Adds every JSON entry in the text file to a vector

    Ok(game) // Either return the parsed Json List, or an error 
}