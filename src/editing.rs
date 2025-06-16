use anyhow::{Result}; // So i can have easy error handling with anyhow
use std::io::{self};
use crate::{basic_operations::check_empty, get_details::{get_game_name, get_game_notes, get_user_rating}, json_backend::{self, save_to_file}};
use json_backend::Game;
use chrono::{NaiveDate};

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
    let new_notes: String;
    let mut confirmation: String = String::new();

    new_notes = get_game_notes()?;

    println!("Are you 100% sure you wish to change the notes from '{}' to '{}' ('Yes' to confirm)", game_log[*index].notes, new_notes);
    io::stdin().read_line(&mut confirmation)?; 
    confirmation = confirmation.trim().to_string();

    if confirmation.as_str() == "Yes"
    {
        game_log[*index].notes = new_notes;
        
        save_to_file(game_log)?; // Update the JSON file.
        println!("Okay Changed :D")
    }
    else {
        println!("Notes Change Cancelled...")
    }

    Ok(())
}

pub fn increment_times_played(game_log: &mut Vec<Game>, index: &usize) -> Result<()>
{
    let plays_number: u8;
    // Get the Number of plays we're adding and handle input errors
    loop {
        let mut plays = String::new();
        println!("How many plays do you want to add?");
        io::stdin().read_line(&mut plays)?;

        match check_empty(&plays)
        {
            Ok(plays) => 
            {
                plays_number = match plays.trim().parse::<u8>()
                {
                    Ok(num) => num,
                    _ => 
                    {
                        println!("You need to enter a valid number");
                        continue; // Invalid input detected so we need to go back to the start of the loop
                    }
                }
            },
            _ => 
            {
                println!("You need to enter a number");
                continue; // Invalid input detected so we need to go back to the start of the loop
            }
        }
        break;
    }

    // Get last played date, handle input errors and save the updated information to the JSON file.
    loop {
        let mut date_input: String = String::new();
        println!("When did you last play (YYYY-MM-DD)");
        io::stdin().read_line(&mut date_input)?;

        match NaiveDate::parse_from_str(&date_input.trim(), "%Y-%m-%d")
        {
            Ok(date) => { 
                game_log[*index].times_played += plays_number;
                game_log[*index].last_playthrough = date.to_string();
                save_to_file(game_log)?;
                println!("Successfully updated game :D");
                break;
            },
            _ => println!("You entered an invalid date. Please enter a valid date")
        }
    }
    Ok(())
}