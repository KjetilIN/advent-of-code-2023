use std::{path::Path, fs::File, io::{self, BufReader, Read}, process::exit};

// Constant that describe how many of each there is a max number of each color 
const MAX_POSSIBLE_REDS: u32 = 12;
const MAX_POSSIBLE_GREENS: u32 = 13;
const MAX_POSSIBLE_BLUES: u32 = 14;


/**
 * Helper function that gets the amount of the given color
 *  - takes the vector of game items as a string and the index where the color was detected
 *  - returns the amount of the color or stops the program
 */
fn get_amount(vector: &Vec<&str>, index: usize) -> u32{
    match vector.get(index-1){
        Some(amount) => amount.parse().unwrap_or_else(|err|{
            eprintln!("ERROR: error during the parsing the amount {amount}: {err}");
            exit(1);
        }),
        None =>{
            eprintln!("ERROR: during formatting amount",);
            exit(1);

        }
    }
}

/**
 * Helper function that gets the game id
 * - Uses the vector of game items and the index of where "Game" was found
 * - Will return the game id if correctly formatted
 * - Otherwise stops the program
 */
fn get_game_id(vector: &Vec<&str>, index: usize) -> u32{
    match vector.get(index+1){
        Some(id) =>{

            // ID = "1:", so we remove the colum by taking away the last char
            let number = &id[0..id.len()-1];

            number.parse().unwrap_or_else(|err|{
                eprintln!("ERROR: error during the parsing game id {number}: {err}");
                exit(1);
            })
        },
        None => {
            eprintln!("ERROR: getting the Game ID");
            exit(1);
        }

    }
}


/**
 * The main function for part 1 
 * - Takes a input str, which is a game
 * - Checks if the game is possible
 * - Return the game id if it is possible 
 * - Return 0 if the game was not possible 
 * 
 * NB! Will exit the program if the game is not formatted correctly  
 */
pub fn check_game_possible(input: &str) -> u32{
    
    // Initialize variables  
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
                // Found the game string and will set the id
                game_id =  get_game_id(&word_list, index);
            }
            &";" =>{
                // A semicolon is found, so we reset or values 
                green = 0;
                red = 0;
                blue = 0;
            },
            &"green" => {
                // Get the amount for the color
                let amount: u32 =  get_amount(&word_list, index);

                // Add it to the total amount
                green += amount;

                // Check if game is possible 
                if green > MAX_POSSIBLE_GREENS{
                    return 0 
                }

            },
            &"blue" => {
                // Get the amount of the color blue
                let amount: u32 =  get_amount(&word_list, index);

                // Add it to the total
                blue += amount;

                // Check if game is possible 
                if blue > MAX_POSSIBLE_BLUES{
                    return 0 
                }

            },
            &"red" => {
                // Get the amount of reds
                let amount: u32 =  get_amount(&word_list, index);

                // Add it to the total
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


/**
 * Function that find the least amount of cubes needed to show for each color. 
 * - Finds the least amount of each color 
 * - returns each minimum of colors cubed
 * - will return 0 if a cube of the three colors was not shown in a game
 */
pub fn minimum_cubes_powered(input: &str) -> u32{
    
    // Set the number of cubes to be a really high number
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
            &"green" => {
                // Get the amount for the color
                let amount: u32 =  get_amount(&word_list, index);

                // If the amount is more
                if amount > green{
                    green = amount;
                }

            },
            &"blue" => {
                // Get the amount of the color blue
                let amount: u32 =  get_amount(&word_list, index);

                // If the amount is more
                if amount > blue{
                    blue = amount;
                }

            },
            &"red" => {
                // Get the amount of reds
                let amount: u32 =  get_amount(&word_list, index);

                // If the amount is more
                if amount > red{
                    red = amount;
                }

            }
            _ => {}
        }
    }


    // Return the power of each 
    red*green*blue
}


fn main() -> io::Result<()>{
    println!("--- Day 2: Cube Conundrum ---");

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

    // Sum of powers for part 2
    let mut minimal_cubes_power_sum: u32 = 0;

    // For each line
    for line in content.lines(){
        game_ids_possible_sum += check_game_possible(line);
        minimal_cubes_power_sum += minimum_cubes_powered(line);
    }

    // Print part 1 solution
    println!("Sum of IDs: {game_ids_possible_sum}");

    // Print part 2 solution 
    println!("Sum of minimal amount of cubes per game powered: {minimal_cubes_power_sum}");


    Ok(())
}
