use std::{collections::HashMap, path::Path, fs::File, io::{BufReader, Read}, process::exit, cmp::Ordering};

#[derive(Debug)]
enum Hand {
    FULL_HOUSE,
    FIVE_OF_KIND,
    FOUR_OF_KIND,
    THREE_OF_KIND,
    TWO_PAIR,
    ONE_PAIR,
    HIGH_CARD,
}

#[derive(Debug)]
struct Card {
    pub hand: Hand,
    cards: String,
    bid: u32,
}

#[derive(Debug)]
struct CardList{
    cards: Vec<Card>
}

trait CardListMethods {
    fn new(list: Vec<Card>) -> Self;
    fn sort_by_rank_and_cards(&mut self);
    fn get_bid_score(&self)->u32;
}

trait CardMethods {
    fn new(line: &str) -> Option<Self> where Self: Sized;
    fn compare_card(&self, compared_card: &Card) -> Ordering;
    fn sort_card_chars(&mut self);
}

impl CardMethods for Card {
    fn new(line: &str) -> Option<Self> where Self: Sized {
        let temp_vector: Vec<&str> = line.split_whitespace().collect();
    
        // Check if the vector has at least two elements
        if temp_vector.len() < 2 {
            eprintln!("ERROR: not enough elements in the line");
            return None;
        }
    
        let cards = temp_vector[0];

        // Use the unwrap_or_else method with a closure to handle the error case
        let bid: u32 = match temp_vector[1].parse() {
            Ok(value) => value,
            Err(err) => {
                eprintln!("ERROR: during parsing bid of card: {}", err);
                return None;
            }
        };
    
        // Use match to handle the Result returned by classify_card
        let hand = match classify_card(cards) {
            Ok(hand) => hand,
            Err(err) => {
                eprintln!("ERROR: could not create card due to error in constructor: {}", err);
                return None;
            }
        };
    
        // Use Some(Self { ... }) instead of the return statement
        Some(Self {
            hand,
            cards: cards.to_string(), // assuming cards is a &str in the struct
            bid,
        })
    }

    fn sort_card_chars(&mut self){
        let mut chars: Vec<char> = self.cards.chars().collect();
        chars.sort_by(|a, b| a.cmp(b));
        self.cards = String::from_iter(chars);
    }

    fn compare_card(&self, compared_card: &Card) -> Ordering{
        for (index, char) in self.cards.chars().enumerate(){
            let current_char_score: u8 =  match char_order(char){
                Some(order) => order,
                None =>{
                    eprintln!("ERROR: retrieving the char order for char '{}' in cards: {}", char, self.cards);
                    exit(1)
                }
            };


            let compared_card_score: u8 = match char_order(compared_card.cards.chars().nth(index).unwrap()){
                Some(order) => order,
                None =>{
                    eprintln!("ERROR: retrieving the char order for char '{}' in cards: {}", char, self.cards);
                    exit(1)
                }
            };

            if current_char_score < compared_card_score {
                return Ordering::Less;
            }
            
            if current_char_score > compared_card_score {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    }
    
}

fn char_order(c: char) -> Option<u8> {
    match c {
        'A' => Some(13),
        'K' => Some(12),
        'Q' => Some(11),
        'J' => Some(10),
        'T' => Some(9),
        '9' => Some(8),
        '8' => Some(7),
        '7' => Some(6),
        '6' => Some(5),
        '5' => Some(4),
        '4' => Some(3),
        '3' => Some(2),
        '2' => Some(1),
        _ => None,
    }
}

impl Hand {
    fn score(&self)-> u8{
        match self{
            Hand::FULL_HOUSE => 7,
            Hand::FIVE_OF_KIND => 6,
            Hand::FOUR_OF_KIND => 5,
            Hand::THREE_OF_KIND => 4,
            Hand::TWO_PAIR => 3,
            Hand::ONE_PAIR => 2,
            Hand::HIGH_CARD => 1,
        }
    } 
    
}

fn classify_card(cards: &str) -> Result<Hand, String> {
    if cards.chars().count() != 5 {
        eprintln!("ERROR: card given was invalid: {cards}");
        return Err("Card invalid".to_string());
    }

    let card_chars: Vec<char> = cards.chars().collect();
    let mut card_map: HashMap<char, u32> = HashMap::new();

    for char in card_chars {
        if let Some(item) = card_map.get_mut(&char) {
            *item += 1;
        } else {
            card_map.insert(char, 1);
        }
    }

    let mut max_matches = 0;
    let mut pair_count = 0;

    for (_, count) in card_map.iter() {
        // Check if we have a new top amount of matches
        if count > &max_matches {
            // Check for full house
            if count == &2 && max_matches == 3 {
                return Ok(Hand::FULL_HOUSE);
            }

            max_matches = *count;
        }

        if count >= &2 {
            pair_count += 1;
        }
    }

    // We have 3, 4 or 5 of a kind return
    if max_matches > 2 {
        match max_matches {
            3 => return Ok(Hand::THREE_OF_KIND),
            4 => return Ok(Hand::FOUR_OF_KIND),
            5 => return Ok(Hand::FIVE_OF_KIND),
            _ => {
                eprintln!("ERROR: was over 2 matches, but did not fit card type : {max_matches}");
                return Err("Card Type could not be set".to_string());
            }
        };
    } else {
        match pair_count {
            1 => return Ok(Hand::ONE_PAIR),
            2 => return Ok(Hand::TWO_PAIR),
            _ => return Ok(Hand::HIGH_CARD),
        };
    }
}

impl CardListMethods for CardList {

    fn new(list: Vec<Card>) -> Self{
        Self { cards: list }
    }

    fn sort_by_rank_and_cards(&mut self) {
        self.cards.sort_unstable_by(|c1, c2| {
            match c1.hand.score().cmp(&c2.hand.score()) {
                Ordering::Equal => c1.compare_card(c2),
                v => v,
            }
        });
    }

    fn get_bid_score(&self)->u32{
        let mut rank = 1; 
        let mut sum = 0; 
        for card in self.cards.iter(){
            sum += rank * card.bid;
            rank+=1;
        }

        sum
    }
    
}



fn main() -> std::io::Result<()> {
    println!("--- Day 7: Camel Cards ---");

    // Save the content of a file to a string; 
    let mut content = String::new();
    
    // Open the file of input relative to the folder
    let path = Path::new("cards.txt");
    let file = File::open(&path)?;

    // Create a buffer reader for the file. 
    // A buffer reader is a way to optimize reads, by making a single sys call 
    let mut buf_reader = BufReader::new(file);

    // Read the content to the mutable variable content
    buf_reader.read_to_string(&mut content)?;

    let mut cards: Vec<Card> = Vec::new();

    // For each line we need to fine the number
    for line in content.lines(){
        let card = match Card::new(line){
            Some(card) => card,
            None => exit(1)
        };

        // Add the card
        cards.push(card);
    }

    let mut card_list = CardList::new(cards);
    card_list.sort_by_rank_and_cards();

    println!("{:?}", card_list);

    println!("Card bid score = {}", card_list.get_bid_score());

    Ok(())

}
