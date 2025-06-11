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

fn main() -> io::Result<()> {
    let mut exit_condition = false;
    let mut confirmation = String::new();

    let file = File::open("src/GameLog.txt")?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

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

    Ok(())
}
