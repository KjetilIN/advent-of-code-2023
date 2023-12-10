use std::{collections::HashMap, path::Path, fs::File, io::{BufReader, Read}, process::exit, cmp::Ordering};

#[derive(Debug)]
enum Hand {
    FullHouse,
    FiveOfKind,
    FourOfKind,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
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
    fn get_bid_score(&mut self)->u32;

}

trait CardMethods {
    fn new(line: &str) -> Option<Self> where Self: Sized;
    fn compare_card(&self, compared_card: &Card) -> Ordering;
    fn has_joker(&self) -> bool;
    fn replace_joker_with(&mut self, new_char:char);
    fn classify_card_with_wildcard(&mut self);
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

    fn compare_card(&self, compared_card: &Card) -> Ordering{
        for (index, char) in self.cards.chars().enumerate(){
            let current_char_score: u8 =  match score_char(char){
                Some(order) => order,
                None =>{
                    eprintln!("ERROR: retrieving the char order for char '{}' in cards: {}", char, self.cards);
                    exit(1)
                }
            };


            let compared_card_score: u8 = match score_char(compared_card.cards.chars().nth(index).unwrap()){
                Some(order) => order,
                None =>{
                    eprintln!("ERROR: retrieving the char order for char '{}' in cards: {}", compared_card.cards.chars().nth(index).unwrap(), compared_card.cards);
                    exit(1)
                }
            };

            match current_char_score.cmp(&compared_card_score) {
                Ordering::Equal => continue,
                non_equal_ordering => return non_equal_ordering,
            }
        }

        // Compare entire card strings when scores are equal
        self.cards.cmp(&compared_card.cards)
    }

    fn has_joker(&self) -> bool {
        match self.cards.find("J"){
            Some(_) => true,
            None => false,
        }
    }

    fn replace_joker_with(&mut self, new_char:char) {
        self.cards.replace("J", &new_char.to_string());
    }

    fn classify_card_with_wildcard(&mut self){
        if self.cards.chars().count() != 5 {
            eprintln!("ERROR: card given was invalid: {}", self.cards);
            exit(1);
        }
    
        let card_chars: Vec<char> = self.cards.chars().collect();
        let mut card_map: HashMap<char, u32> = HashMap::new();
    
        for char in card_chars {
            if let Some(item) = card_map.get_mut(&char) {
                *item += 1;
            } else {
                card_map.insert(char, 1);
            }
        }
    
    
        let mut pairs = 0;
        let mut max_matches = 0;
        let mut max_matches_char = ' ';
        let mut next_max_matches = 0;
        let mut best_card:char = ' ';
    
        let mut sorted_entries: Vec<_> = card_map.into_iter().collect();
        sorted_entries.sort_by(|a, b| a.1.cmp(&b.1));
        for (char, count) in sorted_entries {
            // Check if we have a new top amount of matches
            if count > max_matches {
    
                next_max_matches = max_matches.clone();
                max_matches = count;
                max_matches_char = char;
                
            }
    
            if count >= 2 {
                pairs += 1;
            }
        }

        let has_joker = Self::has_joker(&self);
    
        if has_joker{
            // In case of four of a kind
            if max_matches == 4{
                self.replace_joker_with(max_matches_char);
                self.hand = Hand::FiveOfKind;
                return;
            }

            if max_matches == 3{
                
            }



        }else{

        }
    
    }
    
    
}

fn score_char(c: char) -> Option<u8> {
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
            Hand::FiveOfKind => 7,
            Hand::FourOfKind => 6,
            Hand::FullHouse => 5,
            Hand::ThreeOfKind => 4,
            Hand::TwoPair => 3,
            Hand::OnePair => 2,
            Hand::HighCard => 1,
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
    let mut next_max_matches = 0; 
    let mut pair_count = 0;

    // TODO: Not sorting, changes the values WHY!
    let mut sorted_entries: Vec<_> = card_map.into_iter().collect();
    sorted_entries.sort_by(|a, b| a.1.cmp(&b.1));

    for (_, count) in sorted_entries {
        // Check if we have a new top amount of matches
        if count > max_matches {

            next_max_matches = max_matches.clone();
            max_matches = count;
            
        }

        if count >= 2 {
            pair_count += 1;
        }
    }

    if max_matches == 3 && next_max_matches == 2{
        return Ok(Hand::FullHouse);
    }

    // We have 3, 4 or 5 of a kind return
    if max_matches > 2 {
        match max_matches {
            3 => return Ok(Hand::ThreeOfKind),
            4 => return Ok(Hand::FourOfKind),
            5 => return Ok(Hand::FiveOfKind),
            _ => {
                eprintln!("ERROR: was over 2 matches, but did not fit card type : {max_matches}");
                return Err("Card Type could not be set".to_string());
            }
        };
    } else {
        match pair_count {
            1 => return Ok(Hand::OnePair),
            2 => return Ok(Hand::TwoPair),
            _ => return Ok(Hand::HighCard),
        };
    }
}



impl CardListMethods for CardList {

    fn new(list: Vec<Card>) -> Self{
        Self { cards: list }
    }

    fn sort_by_rank_and_cards(&mut self) {
        self.cards.sort_by(|c1, c2| {
            match c1.hand.score().cmp(&c2.hand.score()) {
                Ordering::Equal =>  return c1.compare_card(&c2),
                v => v,
            }
        });
    }

    fn get_bid_score(&mut self)->u32{
        self.sort_by_rank_and_cards();

        let mut rank = 1; 
        let mut sum = 0; 
        for card in self.cards.iter(){
            let card_sum = rank * card.bid;
            sum += card_sum;

            //println!("{} | {:?} | rank({})*bid({}) = {} | Total = {}", card.cards, card.hand, rank, card.bid, card_sum, sum);
            rank += 1;
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
    let score = card_list.get_bid_score();

    //println!("{:?}", card_list.cards);
    println!("Card bid score = {}", score);

    Ok(())

}
