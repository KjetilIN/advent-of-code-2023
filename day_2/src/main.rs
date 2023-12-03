use std::{path::Path, fs::File, io::{self, BufReader, Read}, process::exit};



// Constant that describe how many of each there is a max number of
const MAX_POSSIBLE_REDS: u32 = 12;
const MAX_POSSIBLE_GREENS: u32 = 13;
const MAX_POSSIBLE_BLUES: u32 = 14;


pub fn check_game_possible(input: &str) -> u32{
    
    let mut game_id: u32 = 0; 
    let mut green: u32 = 0;
    let mut red: u32 = 0;
    let mut blue: u32 = 0;

    // We format the game by 
    // - removing the commas 
    // - make the semicolons be on their own with spaces
    let formatted_game = input.trim()
                                 .replace(",", "")
                                 .replace(";", " ;");

    // Turn the line into a vector 
    let word_list: Vec<_> = formatted_game.split(" ").collect();

    for (index, item) in word_list.iter().enumerate(){
        match item{
            &"Game" => {
                game_id =  match word_list.get(index+1){
                    Some(id) =>{

                        // ID = "1:", so we remove the colum by taking away the last char
                        let number = &id[0..id.len()-1];

                        number.parse().unwrap_or_else(|err|{
                            eprintln!("ERROR: error during the parsing game id {number}: {err}");
                            exit(1);
                        })
                    },
                    None => {
                        eprintln!("ERROR: not correctly formatted game: {item}");
                        exit(1);
                    }

                };
            }
            &";" =>{
                // A semicolon is found, so we reset or values 
                green = 0;
                red = 0;
                blue = 0;
            },
            &"green" => {
                let amount: u32 =  match word_list.get(index-1){
                    Some(amount) => amount.parse().unwrap_or_else(|err|{
                        eprintln!("ERROR: error during the parsing the amount {amount}: {err}");
                        exit(1);
                    }),
                    None =>{
                        eprintln!("ERROR: not correctly formatted, the amount is not integer: {item}");
                        exit(1);

                    }
                };

                green += amount;

                // Check if game is possible 
                if green > MAX_POSSIBLE_GREENS{
                    return 0 
                }

            },
            &"blue" => {
                let amount: u32 =  match word_list.get(index-1){
                    Some(amount) => amount.parse().unwrap_or_else(|err|{
                        eprintln!("ERROR: error during the parsing the amount {amount}: {err}");
                        exit(1);
                    }),
                    None =>{
                        eprintln!("ERROR: not correctly formatted, the amount is not integer: {item}");
                        exit(1);

                    }
                };

                blue += amount;

                // Check if game is possible 
                if blue > MAX_POSSIBLE_BLUES{
                    return 0 
                }

            },
            &"red" => {
                let amount: u32 =  match word_list.get(index-1){
                    Some(amount) => amount.parse().unwrap_or_else(|err|{
                        eprintln!("ERROR: error during the parsing the amount {amount}: {err}");
                        exit(1);
                    }),
                    None =>{
                        eprintln!("ERROR: not correctly formatted, the amount is not integer: {item}");
                        exit(1);

                    }
                };

                red += amount;

                // Check if game is possible 
                if red > MAX_POSSIBLE_REDS{
                    return 0 
                }

            }
            _ => {}
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
