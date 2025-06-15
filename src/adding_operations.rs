use std::io;
use anyhow::{Result, anyhow};

pub fn get_user_rating () -> Result<u8>{
    
    let mut rating = String::new();

    println!("\n\nWhats the rating from 1-5?");
    io::stdin().read_line(&mut rating)?;

    let trimmed = rating.trim();
    match trimmed.parse::<u8>()
    {
        // If there is a parsing error, return to menu{
       Ok(num) if num >= 1 && num <= 5 
       => return Ok(num),
        _ => {
            println!("Invalid rating. Please enter a number between 1 and 5.");
            return Err(anyhow!("Invalid Value")) // Create own error object using anyhow and return it
        }
    };
}