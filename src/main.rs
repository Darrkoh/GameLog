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

// Crates //
use std::io::Result; // So i can have easy error handling
use std::io;
use std::u8;
use json_backend::Game;
use json_backend::reading_json;
use clock::get_date;
use json_backend::save_to_file;
use editing_operations::edit_game_name;
use editing_operations::edit_game_rating;
use editing_operations::edit_game_notes;
use editing_operations::increment_times_played;

// THIS IS THE INTERFACE USERS WILL BE INTERACTING WITH //
fn main() -> io::Result<()> {
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
    let mut game_name = String::new();
    let mut rating_string:String = String::new();
    let mut game_notes: String = String::new(); 
    let mut input = String::new();
    let mut index = 0;

    // GAME NAME
    println!("What's the Game's name?");
    let _ = io::stdin().read_line(&mut game_name); // Literally just to shut up the warning, im using let
    game_name = game_name.trim().to_string();

    let exists = linear_search(game_log, &game_name, &mut index);

    // Leave Prematurely since the game already exists
    if exists
    {
        println!("Game already exists in the log. Please edit the existing entry");
        return Ok(()); 
    }

    // RATING
    println!("What rating are you giving the Game?");
    io::stdin().read_line(&mut rating_string)?;
    
    let trimmed = rating_string.trim();
    let game_rating: u8 = match trimmed.parse::<u8>()
    {
        // If there is a parsing error, return to menu{
       Ok(num) if num >= 1 && num <= 5 => num,
        _ => {
            println!("Invalid rating. Please enter a number between 1 and 5.");
            return Ok(());
        }
    };

    // NOTES
    println!("Any Notes? (Just press enter if not)");
    let _ = io::stdin().read_line(&mut game_notes); // Literally just to shut up the warning, im using let

    // Create Model and add to Text file/Vector

    let new_game = Game {
        name: game_name,
        rating: game_rating,
        times_played: 1,
        last_playthrough: get_date().to_string(),
        notes: game_notes
    };

    game_log.push(new_game); // Add new game to the Game Log list displayed to users 

    save_to_file(&mut game_log)?; // We need to use a pointer here to the last element in the vector (Most Recent), as we cant use 'new_game' anymore

    // Confirmation message
    println!("Game added to the log! :D");
    println!("Please press 'Enter' to go back to the main menu...");
    let _ = io::stdin().read_line(&mut input); // Literally only putting this in a variable to silence the warning

    Ok(())
}

fn editing(game_log: &mut Vec<Game>) -> Result<()>
{
    loop {
        let mut index = 0;
        let mut game_name = String::new();
        let mut choice = String::new();
        
        println!("\nPlease Enter the EXACT game's name you want to edit :3");
        io::stdin().read_line(&mut game_name)?; 

        game_name = game_name.trim().to_string();

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