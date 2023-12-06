use std::{path::Path, fs::File, io::{BufReader, Read}, error::Error, process::exit};


struct Card {
    card_id: u32,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>
}


impl Card {
    
    /**
     * Constructor function for creating a new Card.
     * - Takes a card string of correct format 
     */
    fn new(card_string: &String) -> Result<Self, String>{

        // Get the card id from the correct format 
        let card_id:u32 = match &card_string[4..8].trim().parse(){
            Ok(id) => *id,
            Err(_) => {
                eprintln!("ERROR: creating card from id string : {:?}", &card_string[5..9].to_string());
                return Err("Card id not found".to_string());
            },
        };

        let mut winning_numbers: Vec<u32> = Vec::new();
        let mut card_numbers: Vec<u32> = Vec::new();

        let binding = card_string[9..].replace("  "," ");

        // Create a vector from the input
        let numbers_vector: Vec<_> = binding.split(" ")
                                            .filter(|x| !x.is_empty())
                                            .collect();
        
        
        let mut is_winning_numbers: bool = true;
        for i in numbers_vector.iter(){
            let item = i.trim();
            
            // If the divider has been found 
            if item == "|"{
                is_winning_numbers = false;
                continue;
            }

            // Get the number for the current number
            let current_number: u32 = match item.parse::<u32>(){
                Ok(number) => number,
                Err(err) => {
                    eprintln!("ERROR during parsing of number {i}: {item}");
                    return Err(err.to_string());
                },
            };

            //Add it to the correct vector
            if is_winning_numbers{
                winning_numbers.push(current_number);
            }else{
                card_numbers.push(current_number);
            }

        }

        // Return a new instance of self 
        Ok(Self{
            card_id,
            winning_numbers,
            card_numbers
        })


    }


    fn get_points(&self) ->u32{
        let mut points:u32 = 0; 
        let mut winning_value = 1;
        let mut has_one_match = false;

        for number in self.card_numbers.iter(){
            if self.winning_numbers.contains(number){
                // If the first match has happened
                if has_one_match == false{
                    points +=1;
                    has_one_match = true

                }else{
                    // Add the winning value
                    points += winning_value;
                    // Double the next winning value
                    winning_value *= 2;
                }

            }
        }

        points
    }
    
}


fn get_total_points(cards: &Vec<Card>) ->u32{
    let mut total_points:u32 = 0;

    for card in cards.iter(){
        total_points += card.get_points();
    }

    // Return the total points 
    total_points
}

fn main() ->std::io::Result<()> {
    println!("--- Day 4: Scratchcards ---");

    let mut content = String::new();
    
    // Open the file of input relative to the folder
    let path = Path::new("cardList.txt");
    let file = File::open(&path)?;

    // Create a buffer reader for the file. 
    // A buffer reader is a way to optimize reads, by making a single sys call 
    let mut buf_reader = BufReader::new(file);

    // Read the content to the mutable variable content
    buf_reader.read_to_string(&mut content)?;

    let mut card_list: Vec<Card> = Vec::new();

    // For each line we need to fine the number
    for line in content.lines(){
        // Add the number from the file to the total sum
        let card = match Card::new(&line.to_string()){
            Ok(card) => card,
            Err(err) => {
                exit(1);
            },
        };
        card_list.push(card)

    }

    let total_points = get_total_points(&card_list);

    println!("Total amount of points: {}",total_points);

    Ok(())
}
