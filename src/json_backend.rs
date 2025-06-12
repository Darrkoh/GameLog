// Crates //
use std::io;
use std::io::BufReader;
use std::fs::File;
use serde::{Deserialize, Serialize};

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

    let reader = BufReader::new(file);
    let game = serde_json::from_reader(reader); // Adds every JSON entry in the text file to a vector

    // Custom try catch statement using match. If an error is found, an empty vector will be produced, if not the returned vector held in the 'game' variable will be produced.
    // This is to make sure the program will run if an empty list is found (it won't if line 22 just propagates errors with ?)
    match game {
        Ok(games) => Ok(games), // Either return the parsed Json List, or an error 
        Err(_e) => 
        {
            Ok(Vec::new())
        }
    }
}

// Create a Game and add it's Json data to the text file
pub fn adding_game(game_log: &mut Vec<Game>, new_game: &Game)
{

}
