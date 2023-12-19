use std::cmp::Ordering;
use std::process::exit;

use crate::classify_card::{classify_card_with_wildcard, classify_card};
use crate::hand::Hand;
use crate::score_card::{score_char_with_wildcard, score_char};

#[derive(Debug, Clone)]
pub struct Card {
    pub hand: Hand,
    pub cards: String,
    pub bid: u32,
}

/// Trait for the card
pub trait CardMethods {
    fn new(line: &str, has_wild_card: bool) -> Option<Self> where Self: Sized;
    fn compare_card(&self, compared_card: &Card) -> Ordering;
    fn compare_card_with_wildcard(&self, compared_card: &Card) -> Ordering;
}


/// Implementing the card methods for the struct
impl CardMethods for Card {
    fn new(line: &str, has_wild_card: bool) -> Option<Self> where Self: Sized {
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
        let hand = if has_wild_card {
                                        match classify_card_with_wildcard(cards){
                                            Ok(hand) =>  hand ,
                                            Err(err) => {
                                                eprintln!("ERROR: could not create card due to error in constructor: {}", err);
                                                return None;
                                            }
                                        }
                                        
                                    }else{
                                        match classify_card(cards){
                                            Ok(hand) => hand,
                                            Err(err) => {
                                                eprintln!("ERROR: could not create card due to error in constructor: {}", err);
                                                return None;
                                            }
                                        }
                                        
                                    };
    
        // Use Some(Self { ... }) instead of the return statement
        Some(Self {
            hand,
            cards: cards.to_string(), // assuming cards is a &str in the struct
            bid,
        })
    }

    /// Method for comparing two cards, part 1 
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

    /// Method for comparing cards part 2 
    fn compare_card_with_wildcard(&self, compared_card: &Card) -> Ordering{
        for (index, char) in self.cards.chars().enumerate(){
            let current_char_score: u8 =  match score_char_with_wildcard(char){
                Some(order) => order,
                None =>{
                    eprintln!("ERROR: retrieving the char order for char '{}' in cards: {}", char, self.cards);
                    exit(1)
                }
            };


            let compared_card_score: u8 = match score_char_with_wildcard(compared_card.cards.chars().nth(index).unwrap()){
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
    
    
}
