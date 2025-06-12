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

// Crates //
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::fs;

// THIS IS THE INTERFACE USERS WILL BE INTERACTING WITH //
fn main() -> io::Result<()> {
    let mut exit_condition = false;

    let file = File::open("src/GameLog.txt")?; // Gets file contents
    let metadata = fs::metadata("GameLog.txt")?; // Gets the information about the file (Like length for example)

    let mut reader = BufReader::new(file); // Prepares file for reading by wrapping in 'BuffReader' to buffer the input
    let mut buffer = String::new(); // Empty Buffer to store file contents in (Buffer is like a dynamic array that only stores text)

    reader.read_to_string(&mut buffer)?; // Add File Contents to the buffer

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
- Notes"
    );

    loop {
        println!("Please enter the corresponding option to access each function:
        
        - Adding (1)
        - Removing (2)
        - Searching (3) 
        - Bring up whole list (4)
        ");

        // Read user's choice
        let mut input = String::new();
        io::stdin().read_line(&mut input)?; 

        input = input.trim().to_string(); // This just removes any empty space after the inputted text

        match input.as_str(){ // We turn input into a 'str' as 'String' and 'str' are not the same thing as 'str' is part of a string 
            "1"=> { 
                let index = metadata.len(); 
                adding();
            },
            "2"=> removing(),
            "3"=> searching(),
            "4"=> whole_list(buffer),
            _=> println!("Invalid Input")
        }

        // The break is in a for loop to check if user wishes to quit, So the loop will run atleast once
        if  exit_condition { 
            break;
        }
    }
    Ok(())
}

fn adding() -> io::Result<()>
{
    Ok(())
}

fn removing() -> io::Result<()>
{
    Ok(())
}

fn searching() -> io::Result<()>
{
    Ok(())
}

fn whole_list(buffer: String) // Literally just print the whole file and return
{
    print!(buffer);
}
