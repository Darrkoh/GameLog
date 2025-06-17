use crate::{enums, json_backend::Game};
use std::io;
use anyhow::{Result, anyhow};
use enums::Rating;

pub fn get_user_rating () -> Result<Rating>{
    
    let mut rating_string = String::new();

    println!("\n Whats the rating from 1-5?");
    io::stdin().read_line(&mut rating_string)?;

    let trimmed = rating_string.trim();
    match trimmed.parse::<u8>()
    {
        // If there is a parsing error, return to menu{
       Ok(rate) => 
        {
            match Rating::from_u8(rate)
            {
                Some(rating) => Ok(rating), // If value is in bounds
                None => Err(anyhow!("Value: {} is out of bounds", rate)) // If value is not in bounds and null is returned
            }
        },
        _ => {
            println!("\n Invalid rating. Please enter a number between 1 and 5.");
            return Err(anyhow!("Invalid Value")) // Create error object using anyhow and return it
        }
    }
}

pub fn get_game_name () -> Result<String>{
    let mut game_name: String = String::new();
    println!("\n\n What's the Game's name?");
    println!(" Entering nothing will exit the process");

    io::stdin().read_line(&mut game_name)?;

    game_name = game_name.trim().to_string();

    Ok(game_name)
} 


pub fn get_game_notes () -> Result<String>{
    let mut game_notes: String = String::new();
    println!("What Notes do you have? (Just press enter to go back)");
    io::stdin().read_line(&mut game_notes)?;
    let game_notes = game_notes.trim().to_string();

    Ok(game_notes)
} 

pub fn get_game_details (game: &Game, index: & usize)
{
    println!("\n Index: {}\n  Name: {}\n  Rating: {}/5\n  Times Played: {}\n  Last Played: {}\n  Notes: {}",
                index,
                game.name,
                game.rating,
                game.times_played,
                game.last_playthrough,
                game.notes
            );
} 