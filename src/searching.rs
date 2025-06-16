use crate::json_backend::Game;

pub fn linear_search(game_log: &[Game], target: &str, index_position: &mut usize) ->  bool
{
    for (i, num) in game_log.iter().enumerate()
    {
        if num.name == target
        {
            *index_position = i; // Modify the data in that address (Dereference) to update the index to the games index, for accessing the games details outside the method
            return true; // Game name exists in the list
        }
    }
    false
}