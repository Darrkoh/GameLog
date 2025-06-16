use std::io;
use anyhow::{Result, anyhow};

pub fn get_user_rating () -> Result<u8>{
    
    let mut rating = String::new();

    println!("\n Whats the rating from 1-5?");
    io::stdin().read_line(&mut rating)?;

    let trimmed = rating.trim();
    match trimmed.parse::<u8>()
    {
        // If there is a parsing error, return to menu{
       Ok(num) if num >= 1 && num <= 5 
       => return Ok(num),
        _ => {
            println!("\n Invalid rating. Please enter a number between 1 and 5.");
            return Err(anyhow!("Invalid Value")) // Create error object using anyhow and return it
        }
    };
}

pub fn get_game_name () -> Result<String>{
    let mut game_name: String = String::new();
    println!("\n\n What's the Game's name?");

    io::stdin().read_line(&mut game_name)?;

    game_name = game_name.trim().to_string();

    Ok(game_name)

} 