use anyhow::{Result}; // So i can have easy error handling with anyhow
use std::io;
use crate::{get_details::get_user_rating, get_details::get_game_name,json_backend::{self, save_to_file}};
use json_backend::Game;

// Obviously TODO
pub fn edit_game_name(game_log: &mut Vec<Game>, index: &usize) -> Result<()>
{
    let new_name;
    let mut confirmation = String::new();

   new_name = get_game_name()?;

    println!("Are you 100% sure you wish to change the name '{}' to '{}' ('Yes' to confirm)", game_log[*index].name, new_name);
    io::stdin().read_line(&mut confirmation)?; 
    confirmation = confirmation.trim().to_string();

    if confirmation.as_str() == "Yes"
    {
        game_log[*index].name = new_name;
        
        save_to_file(game_log)?; // Update the JSON file.
        println!("Okay Changed :D")
    }
    else {
        println!("Name Change Cancelled...")
    }

    Ok(())
}

pub fn edit_game_rating(game_log: &mut Vec<Game>, index: &usize) -> Result<()>
{
    match  get_user_rating() { // Save the rating if a valid number is given
        Ok(rating) => {
            game_log[*index].rating = rating;
        save_to_file(game_log)?; // Update the JSON file.
        println!("Rating has been changed");
        } // Tell user the rating wasn't saved as they entered an invalid number (ERROR HANDLING)
        _ => println!("Returning to previous menu...")
    };
    Ok(())
}

pub fn edit_game_notes(game_log: &mut Vec<Game>, index: &usize) -> Result<()>
{
    println!("\n\n");
    Ok(())
}

pub fn increment_times_played(game_log: &mut Vec<Game>, index: &usize) -> Result<()>
{
    println!("\n\n");

    Ok(())
}