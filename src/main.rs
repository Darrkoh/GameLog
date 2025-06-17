/* 
    Refined version of a program I made in 1st Year Uni.

    I am making this one in Rust since Java is awful to work with lol

    It will act as a game log for videogames, allowing users to:
    - Add 
    - Remove 
    - Search for games in the log
    
     Bringing up details on:
    - All playthroughs
    - Last Playthrough
    - Rating
    - Notes
*/

mod json_backend;
mod clock;
mod editing;
mod get_details;
mod searching;
mod basic_operations;
mod enums;

// Crates //
use std::io; 
use anyhow::{Result}; // So i can have easy error handling with anyhow
use json_backend::{Game, reading_json, save_to_file};
use clock::get_date;
use editing::{edit_game_name, edit_game_notes, edit_game_rating, increment_times_played};
use get_details::{get_user_rating, get_game_name, get_game_details, get_game_notes};
use searching::linear_search;
use basic_operations::check_empty;

use crate::enums::Rating;

// THIS IS THE INTERFACE USERS WILL BE INTERACTING WITH //
fn main() -> Result<()> {
    let mut exit_condition = false;

    let mut game_log = reading_json()?;

    println!(
        "\n Welcome To Your Game Log!

    Here You Can:
    - Add
    - Remove
    - Search For Games

    And also bring up details on:
    - All Playthroughs
    - Last Playthrough
    - Rating
    - Notes");

    // Menu
    loop {
        println!("\n\n Please enter the corresponding option to access each function:
        
        - Adding (1)
        - Removing (2)
        - Editing (3)
        - Searching (4) 
        - Bring up whole list (5)
        - Exit Program (6)
        ");

        // Read user's choice
        let mut input = String::new();
        io::stdin().read_line(&mut input)?; 

        match input.trim(){ // We turn input into a 'str' using trim (also eliminates blank spaces) as 'String' and 'str' are not the same thing as 'str' is part of a string 
            "1"=> {adding(&mut game_log)?;}, // All referenced so we can just edit the original in the main function
            "2"=> {removing(&mut game_log)?;},
            "3"=> {editing(&mut game_log)?;},
            "4"=> {searching(&mut game_log)?;},
            "5"=> {whole_list(&mut game_log)?;},
            "6" => {
                println!(" \nExiting Program, Thank you for using this app :D\n");
                exit_condition = true; // Close Game Condition
            }
            _=> println!(" Invalid Input")
        }

        // The break is in a for loop to check if user wishes to quit, So the loop will run atleast once
        if  exit_condition { 
            break;
        }
    }
    Ok(())
}

fn adding(mut game_log: &mut Vec<Game>) -> Result<()>
{
    let mut game_name;
    let rating: Rating;
    let game_notes: String; 
    let mut input = String::new();
    let mut index = 0;

    // GAME NAME
    game_name = get_game_name()?;

   // Exit if Empty Text
        match check_empty(&game_name) 
        {
            Ok(_input) => game_name = game_name.trim().to_string(),
            _ => return Ok(()) // Cancel operation if user enters an invalid input
        }

    let exists = linear_search(game_log, &game_name, &mut index);

    // Leave Prematurely since the game already exists
    if exists
    {
        println!("\n\n Game already exists in the log. Please edit the existing entry");
        return Ok(()); 
    }

    // RATING
    match  get_user_rating() { // Save the rating if a valid number is given
        Ok(user_rating) =>  rating = user_rating, // Tell user the rating wasn't saved as they entered an invalid number (ERROR HANDLING)
        _ => 
        { 
            println!(" Returning to previous menu...");
            return Ok(())
        }
    }
    
    // NOTES
    game_notes = get_game_notes()?;

    // Create Model and add to Text file/Vector

    let new_game = Game {
        name: game_name,
        rating: rating,
        times_played: 1,
        last_playthrough: get_date().to_string(),
        notes: game_notes
    };

    game_log.push(new_game); // Add new game to the Game Log list displayed to users 

    save_to_file(&mut game_log)?; // We need to use a pointer here to the last element in the vector (Most Recent), as we cant use 'new_game' anymore

    // Confirmation message
    println!(" Game added to the log! :D");
    println!(" Please press 'Enter' to go back to the main menu...");
    io::stdin().read_line(&mut input)?; // What the user enters does not matter, its just so we can confirm theyre okay with the result

    Ok(())
}

fn editing(game_log: &mut Vec<Game>) -> Result<()>
{
    loop {
        let mut index = 0;
        let mut game_name;
        let mut choice = String::new();
        
        game_name = get_game_name()?;

        // Exit if Empty Text
        match check_empty(&game_name) 
        {
            Ok(_input) => game_name = game_name.trim().to_string(),
            _ => return Ok(()) // Cancel operation if user enters an invalid input
        }

        let game_exists = linear_search(game_log, &game_name, &mut index);

        if !game_exists
        {
            println!(" \n\n The Game '{}' doesn't exist. Make sure it's spelt the exact same as it is in the Game Log List or we can't find it :/", game_name);
            return Ok(());
        }

        // IDEA: If list ever gets to big, have asynchronous print(Variable) here, where the variable changes to display ... animation
        println!("\n Game found! Displaying Current Details
            - Current Name: {} 
            - Current Rating {}/5
            - Current Times Played: {}
            - - Current Last Date Played: {}
            - Current Notes: {}", game_log[index].name, game_log[index].rating, game_log[index].times_played, game_log[index].last_playthrough, game_log[index].notes );

        println!("\nWhat part are you editing \n\n
            - [1] Game Name
            - [2] Rating
            - [3] Notes
            - [4] Increment Times Played
            - Anything else to go back");
        
        io::stdin().read_line(&mut choice)?; 

        match choice.trim() {
            "1" => edit_game_name(game_log, &index)?,
            "2" => edit_game_rating(game_log, &index)?,
            "3" => edit_game_notes(game_log, &index)?,
            "4" => increment_times_played(game_log, &index)?,
            _ => break
        }
    }
    Ok(())
}

fn removing(game_log: &mut Vec<Game>) -> Result<()>
{
    let mut game_name;
    game_name = get_game_name()?;

    // Yeah this is how you exit
    match check_empty(&game_name) 
    {
        Ok(_input) => game_name = game_name.trim().to_string(),
        _ => return Ok(()) // Cancel operation if user enters an invalid input
    }

    let mut index: usize = 0;
    let exists = linear_search(game_log, &game_name, &mut index);

    // If game isn't a thing, exit method
    if !exists {
        println!(" Game Not found");
        return Ok(());
    };

    println!(" Are you SURE you want to REMOVE '{}' [Enter 'Yes']", game_name);
    let mut confirmation_answer = String::new();
    io::stdin().read_line(&mut confirmation_answer)?;

    // If user does not confirm they want to remove the gamem, return to previous menu
    if confirmation_answer.trim().to_lowercase() != "yes"
    {
        println!("Operation Cancelled");
        return Ok(());
    }

    // Remove the game and its data, then save the new list to the JSON file
    game_log.remove(index);
    save_to_file(game_log)?;

    println!("Game Removed");
    Ok(())
}

fn searching(game_log: &[Game]) -> Result<()>
{
    loop {
        let game_name;
        let exists: bool;
        let mut index = 0;
        game_name = get_game_name()?;

        exists = linear_search(game_log, &game_name, &mut index);

        if !exists
        {
            println!("\n This game don't exist buddy :/");
            break;
        }
        
        let game: &Game = &game_log[index];

        get_game_details(game, &index);
    }
    Ok(())
}

fn whole_list(game_log: &[Game]) -> Result<()> // Literally just print the whole file and return
{
    if game_log.is_empty() {
        println!(" Yeah the list is empty pal lmao"); // lol
        return Ok(());
    }
    for (i, game) in game_log.iter().enumerate()
        {
            get_game_details(game, &i);
        }

    // Just a cleaner approach than having the program immediately take the user to the main menu
    println!("\n \nPlease press 'Enter' to go back to the main menu...");

    // Confirmation to return to menu
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}

/////// UNIT TESTS //////////

#[cfg(test)]
mod tests {
    // Importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_searching_for_game() {
        let game_log: Vec<Game> = vec![
            Game { name: "Zelda".to_string(), rating: Rating::Four, times_played: 1, last_playthrough: "2023-06-17".to_string(), notes: "".to_string() }, // to_string is used as they currently in a borrowed format, we need to own them
            Game { name: "Mario".to_string(), rating: Rating::Three, times_played: 2, last_playthrough: "2023-06-18".to_string(), notes: "MARIOOOO".to_string() }
        ];

        let mut index:usize  = 0;
        assert_eq!(linear_search(&game_log, &"zelda", &mut index), true); // Should be true

        let mut index: usize = 0;
        assert_eq!(linear_search(&game_log, &"Mario", &mut index), true); // Should be true

        let mut index: usize = 0;
        assert_eq!(linear_search(&game_log, &"zda", &mut index), false); // Should be false
    }
}
