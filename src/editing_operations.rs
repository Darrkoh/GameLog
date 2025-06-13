use std::io::Result; // So i can have easy error handling
use crate::json_backend;
use json_backend::Game;

// Obviously TODO
pub fn edit_game_name(game_log: &mut Vec<Game>, index: &usize) -> Result<()>
{
    Ok(())
}

pub fn edit_game_rating(game_log: &mut Vec<Game>, index: &usize) -> Result<()>
{
    Ok(())
}

pub fn edit_game_notes(game_log: &mut Vec<Game>, index: &usize) -> Result<()>
{
    Ok(())
}

pub fn increment_times_played(game_log: &mut Vec<Game>, index: &usize) -> Result<()>
{
    Ok(())
}