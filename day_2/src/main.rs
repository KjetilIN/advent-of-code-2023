use std::{path::Path, fs::File, io::{self, BufReader, Read}, process::exit};
use regex::Regex;


// Constant that describe how many of each there is a max number of
const MAX_POSSIBLE_REDS: u32 = 12;
const MAX_POSSIBLE_GREENS: u32 = 12;
const MAX_POSSIBLE_BLUES: u32 = 12;


fn get_game_id(input: &str) -> u32{
    let game_index = match input.find("Game"){
        Some(index)=>index,
        None => {
            eprint!("ERROR: Not a valid game error");
            return 0;
        }
    };

    
    let game_id: String = input.chars()
                                   .skip(game_index+4)
                                   .take_while(|current| current != &':')
                                   .collect();


    match game_id.trim().parse(){
        Ok(res) => res ,
        _ => {
            eprintln!("ERROR: during parsing of game number: {}", game_id.trim());
            exit(1);
        }
    }


}

// Checks if the color has gone over what is possible for he game
// - Returns true if the game is not possible 
// - Returns false if the amount is lower than what the max is or color not found
fn is_below_max_for_color(game: &str, color: &str, max:u32) -> bool{
    match game.find(color){
        Some(index) => {
            let chars_to_iter = &game[&index+5..index];
            Regex::new(r"[A-Za-z]").unwrap().replace_all(chars_to_iter, "_");
            
            let amount = chars_to_iter.trim().parse::<u32>().unwrap_or_else(|err| {
                eprintln!("ERROR: during parsing of int {chars_to_iter} : {err}");
                exit(1);
            });

            if amount > max{
                false
            }else{
                true
            }

            
        },
        _ => {true}
    }
}



fn check_game_possible(input: &str) -> u32{
    // Get the game ID of the strings 
    let game_id: u32 = get_game_id(input);

    // Parse the rest of the games
    let col_index =  match input.find(":"){
        Some(index) => index,
        _ => {
            eprintln!("ERROR: during parsing of line, did not find the : char");
            exit(1);
        }
    };

    let games_played: Vec<_> = (&input[col_index..input.len()]).split(",").collect();

    for game in games_played.iter(){
        if !is_below_max_for_color(game, "red", MAX_POSSIBLE_REDS){
            return 0;
        }

        if !is_below_max_for_color(game, "green", MAX_POSSIBLE_GREENS){
            return 0;
        }

        if !is_below_max_for_color(game, "blue", MAX_POSSIBLE_BLUES){
            return 0;
        }
    }

    return game_id;
}




fn main() -> io::Result<()>{
    println!("--- Day 2 ---");

    let mut content = String::new();

    let path = Path::new("games.txt");
    
    let file = File::open(&path)?;

    let mut buf_reader = BufReader::new(file);

    buf_reader.read_to_string(&mut content).unwrap_or_else(|err| {
        eprint!("ERROR: reading from string : {err}");
        exit(1);
    });

    // Count that is the sum of games that was possible
    let mut game_ids_possible_sum: u32 = 0;

    // For each line
    for line in content.lines(){
        game_ids_possible_sum += check_game_possible(line);
    }

    println!("Sum of IDs: {game_ids_possible_sum}");


    Ok(())
}
