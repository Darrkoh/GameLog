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
mod editing_operations;
mod adding_operations;

// Crates //
use std::io; 
use std::u8;
use anyhow::Result;
use json_backend::Game;
use json_backend::{reading_json, save_to_file};
use clock::get_date;
use editing_operations::{edit_game_name, edit_game_notes, edit_game_rating, increment_times_played};
use adding_operations::{get_user_rating, get_game_name};

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

        input = input.trim().to_string(); // This just removes any empty space after the inputted text

        match input.as_str(){ // We turn input into a 'str' as 'String' and 'str' are not the same thing as 'str' is part of a string 
            "1"=> { 
                adding(&mut game_log)?; // All referenced so we can just edit the original in the main function
            },
            "2"=> { 
                removing(&mut game_log);
            },
            "3"=> {editing(&mut game_log)?;},
            "4"=> {searching(&mut game_log);},
            "5"=> {whole_list(&mut game_log);},
            "6" => {
                println!("Exiting Program, Thank you for using this app :D");
                exit_condition = true; // Close Game Condition
            }
            _=> println!("Invalid Input")
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
    let game_name;
    let rating:u8;
    let mut game_notes: String = String::new(); 
    let mut input = String::new();
    let mut index = 0;

    // GAME NAME
    game_name = get_game_name()?;

    let exists = linear_search(game_log, &game_name, &mut index);

    // Leave Prematurely since the game already exists
    if exists
    {
        println!("Game already exists in the log. Please edit the existing entry");
        return Ok(()); 
    }

    // RATING
    rating = get_user_rating()?;


    // NOTES
    println!("Any Notes? (Just press enter if not)");
    io::stdin().read_line(&mut game_notes)?;

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
    println!("Game added to the log! :D");
    println!("Please press 'Enter' to go back to the main menu...");
    io::stdin().read_line(&mut input)?;

    Ok(())
}

fn editing(game_log: &mut Vec<Game>) -> Result<()>
{
    loop {
        let mut index = 0;
        let game_name;
        let mut choice = String::new();
        
        game_name = get_game_name()?;

        let game_exists = linear_search(game_log, &game_name, &mut index);

        if !game_exists
        {
            println!(" \n\nThe Game '{}' doesn't exist. Make sure it's spelt the exact same as it is in the Game Log List or we can't find it :/", game_name);
            return Ok(());
        }

        // IDEA: If list ever gets to big, have asynchronous print(Variable) here, where the variable changes to display ... animation
        println!("\nGame found! Displaying Current Details
            - Current Name: {} 
            - Current Rating {}/5
            - Current Times Played: {}
            - Current Notes: {}", game_log[index].name, game_log[index].rating, game_log[index].times_played, game_log[index].notes );

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

fn removing(_game_log: &mut Vec<Game>)
{
    println!("In Progress");
}

fn searching(_game_log: &mut Vec<Game>)
{
    println!("In Progress");
}

fn whole_list(game_log: &Vec<Game>) // Literally just print the whole file and return
{
    if game_log.is_empty() {
        println!("Yeah the list is empty pal lmao") // lol
    }
    else {
        for (i, num) in game_log.iter().enumerate()
            {
                println!("\n Index: {}\n  Name: {}\n  Rating: {}/5\n  Times Played: {}\n  Last Played: {}\n  Notes: {}",
                    i,
                    num.name,
                    num.rating,
                    num.times_played,
                    num.last_playthrough,
                    num.notes
                );
            }
    }

    // Just a cleaner approach than having the program immediately take the user to the main menu
    println!("\n \nPlease press 'Enter' to go back to the main menu...");

    // Confirmation to return to menu
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input); // Literally only putting this in a variable to silence the warning
}

fn linear_search(games: &Vec<Game>, target: &str, index_position: &mut usize) ->  bool
{
    for (i, num) in games.iter().enumerate()
    {
        if num.name == target
        {
            *index_position = i; // Modify the data in that address (Dereference) to update the index to the games index, for accessing the games details outside the method
            return true; // Game name exists in the list
        }
    }
    false
}