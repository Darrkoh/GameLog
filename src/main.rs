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

// Crates //
use std::io;
use std::u8;
use json_backend::Game;
use json_backend::reading_json;


// THIS IS THE INTERFACE USERS WILL BE INTERACTING WITH //
fn main() -> io::Result<()> {
    let mut exit_condition = false;

    let mut game_log = reading_json()?;

    println!(
        "Welcome To Your Game Log!

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
        println!("Please enter the corresponding option to access each function:
        
        - Adding (1)
        - Removing (2)
        - Searching (3) 
        - Bring up whole list (4)
        - Exit Program (5)
        ");

        // Read user's choice
        let mut input = String::new();
        io::stdin().read_line(&mut input)?; 

        input = input.trim().to_string(); // This just removes any empty space after the inputted text

        match input.as_str(){ // We turn input into a 'str' as 'String' and 'str' are not the same thing as 'str' is part of a string 
            "1"=> { 
                adding(&mut game_log); // All referenced so we can just edit the original in the main function
            },
            "2"=> { 
                removing(&mut game_log);
            },
            "3"=> searching(&game_log),
            "4"=> whole_list(&game_log),
            "5" => {
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

fn adding(game_log: &mut Vec<Game>)
{
    let mut game_name = String::new();
    let mut rating_string:String = String::new();
    let mut game_notes: String = String::new(); // Make the variable nullable using 'Option'
    let mut input = String::new();

    // GAME NAME
    println!("What's the Game's name?");
    let _ = io::stdin().read_line(&mut game_name); // Literally just to shut up the warning, im using let
    game_name = game_name.trim().to_string();

    let exists = linear_search(game_log, &game_name);

    // Leave Prematurely since the game already exists
    if exists
    {
        println!("Game already exists in the log. Please edit the existing entry");
        return; 
    }

    // RATING
    println!("What rating are you giving the Game?");
    let _ = io::stdin().read_line(&mut rating_string); // Literally just to shut up the warning, im using let
    
    let trimmed = rating_string.trim();
    let game_rating = trimmed.parse::<u8>();

    // If there is a parsing error, return to menu
    match game_rating {
        Ok(parsed) => Ok(parsed), // Either return the parsed Json List, or an error 
        Err(_e) => 
        {
            println!("Nice try, Please enter an valid number.");
            return;
        }
    }

    if (0 > game_rating || game_rating > 5 )
    {
        println!("You entered an invalid number, please enter a rating from 1-5, returning to menu")
    }

    // NOTES
    println!("Any Notes? (Just press enter if not");
    let _ = io::stdin().read_line(&mut game_notes); // Literally just to shut up the warning, im using let

    // Create Model and add to Text file/Vector

    let new_game = Game {
        &game_name;
        &game_rating,
        0,
         ,
        game_notes
    };
    adding_game(&mut game_log);

    // Confirmation message
    println!("Game added to the log! :D");
    println!("Please press 'Enter' to go back to the main menu...");
    let _ = io::stdin().read_line(&mut input); // Literally only putting this in a variable to silence the warning
}

fn removing(_game_log: &mut Vec<Game>)
{
    println!("In Progress");
}

fn searching(_game_log: &Vec<Game>)
{
    println!("In Progress");
}

fn whole_list(game_log: &Vec<Game>) // Literally just print the whole file and return
{
    if game_log.is_empty() {
        println!("Yeah the list is empty pal lmao") // lol
    }
    else {
        for num in game_log
            {
                println!("\nName: {}\n  Rating /10: {}\n  Times Played: {}\n  Last Played: {}\n  Notes: {}",
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

fn linear_search(games: &Vec<Game>, target: &str) ->  bool
{
    for (i, num) in games.iter().enumerate()
    {
        if num.name == target
        {
            return true; // Game name exists in the list
        }
    }
    false
}