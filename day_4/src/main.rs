use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::Path,
    process::exit,
};

#[derive(Clone)]
struct Card {
    card_id: u32,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
    matches: u32,
}

impl Card {
    /**
     * Constructor function for creating a new Card.
     * - Takes a card string of correct format
     */
    fn new(card_string: &String) -> Result<Self, String> {
        // Get the card id from the correct format
        let card_id: u32 = match &card_string[4..8].trim().parse() {
            Ok(id) => *id,
            Err(_) => {
                eprintln!(
                    "ERROR: creating card from id string : {:?}",
                    &card_string[5..9].to_string()
                );
                return Err("Card id not found".to_string());
            }
        };

        let mut winning_numbers: Vec<u32> = Vec::new();
        let mut card_numbers: Vec<u32> = Vec::new();

        let binding = card_string[9..].replace("  ", " ");

        // Create a vector from the input
        let numbers_vector: Vec<_> = binding.split(" ").filter(|x| !x.is_empty()).collect();

        let mut is_winning_numbers: bool = true;

        for i in numbers_vector.iter() {
            let item = i.trim();

            // If the divider has been found
            if item == "|" {
                is_winning_numbers = false;
                continue;
            }

            // Get the number for the current number
            let current_number: u32 = match item.parse::<u32>() {
                Ok(number) => number,
                Err(err) => {
                    eprintln!("ERROR during parsing of number {i}: {item}");
                    return Err(err.to_string());
                }
            };

            //Add it to the correct vector
            if is_winning_numbers {
                winning_numbers.push(current_number);
            } else {
                card_numbers.push(current_number);
            }
        }

        // Find the amount of matches and store it
        let mut matches: u32 = 0;
        for number in winning_numbers.iter() {
            if card_numbers.contains(number) {
                matches += 1;
            }
        }

        // Return a new instance of self
        Ok(Self {
            card_id,
            winning_numbers,
            card_numbers,
            matches,
        })
    }

    /**
     * Method that gets the amount of cards
     */
    fn get_points(&self) -> u32 {
        let mut points: u32 = 0;
        let mut winning_value = 1;
        let mut has_one_match = false;

        for number in self.card_numbers.iter() {
            if self.winning_numbers.contains(number) {
                // If the first match has happened
                if has_one_match == false {
                    points += 1;
                    has_one_match = true
                } else {
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

fn get_total_points(cards: &Vec<Card>) -> u32 {
    let mut total_points: u32 = 0;

    for card in cards.iter() {
        total_points += card.get_points();
    }

    // Return the total points
    total_points
}

struct CardContainer {
    cards_processed: u32,
    card_map: HashMap<u32, Card>,
    cards: Vec<Card>,
}

impl CardContainer {
    /**
     * Constructor
     */
    fn with_cards(cards: Vec<Card>) -> Self {
        let mut card_map: HashMap<u32, Card> = HashMap::new();

        for card in &cards {
            card_map.insert(card.card_id.clone(), card.clone());
        }

        Self {
            cards_processed: 0,
            card_map,
            cards,
        }
    }

    // Process all the cards and
    // sets the amount of cards processed to 0
    // Will remove all cards from the list
    fn process_cards(&mut self) {
        // Set the amount of cards processed to 0  
        self.cards_processed = 0;

        // Clone the cards that is available 
        let mut cards_to_process: Vec<Card> = self.cards.clone();


        // While we still have cards to process
        while !cards_to_process.is_empty() {
            // Remove the card from the vector 
            let current_card: Card = cards_to_process.swap_remove(0);

            // Increment the card processed count
            self.cards_processed += 1;

            // Add copies to be processed
            // The cards that now needs to be processed, given card id n:
            // - Add card n+1, n+2, ... , n+matches
            // Each card need to be processed 
            let mut new_card_index = current_card.card_id + 1;

            // While there is cards to add
            // Will jump over this section when there if there was no matches 
            while new_card_index <= (current_card.card_id + current_card.matches) {

                // Get the card from the card-map and clone it 
                let card_to_add: Card = match self.card_map.get(&new_card_index) {
                    Some(card) => card.clone(),
                    None => {
                        eprintln!("Error cloning card nr {}", new_card_index);
                        std::process::exit(1);
                    }
                };
                
                // Add the cards to be processes
                cards_to_process.push(card_to_add);

                new_card_index += 1;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
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
    for line in content.lines() {
        // Add the number from the file to the total sum
        let card = match Card::new(&line.to_string()) {
            Ok(card) => card,
            Err(err) => {
                eprintln!("ERROR: was not able to create card: {err}");
                exit(1);
            }
        };
        card_list.push(card)
    }

    let total_points = get_total_points(&card_list);

    println!("Total amount of points: {}", total_points);

    let mut card_container = CardContainer::with_cards(card_list);
    card_container.process_cards();

    println!("Cards Processed: {}", card_container.cards_processed);

    Ok(())
}
