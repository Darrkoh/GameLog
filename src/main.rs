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

fn adding(_game_log: &mut Vec<Game>)
{
    println!("In Progress");
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
    println!("\n \nPlease press 'Enter' to go back to the main menu");

    // Read user's choice
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input); // Literally only putting this in a variable to silence the warning
}
