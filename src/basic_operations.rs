use anyhow::{anyhow, Result};

pub fn check_empty (input: &str) -> Result<String>
{
    if input.is_empty()
    {
        println!("\n\n No Text Entered, Exiting Process");
        return Err(anyhow!("No Text Entered"));
    }

    Ok(input.to_string())
}