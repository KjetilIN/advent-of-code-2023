use std::{collections::HashMap, process::exit};

use crate::hand::Hand;

pub fn classify_card(cards: &str) -> Result<Hand, String> {
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

pub fn classify_card_with_wildcard(cards: &str) -> Result<Hand, String>{
    if cards.chars().count() != 5 {
        eprintln!("ERROR: card given was invalid: {}", cards);
        exit(1);
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


    let mut pairs = 0;
    let mut max_matches = 0;
    let mut next_max_matches = 0;

    let mut sorted_entries: Vec<_> = card_map.into_iter().collect();
    sorted_entries.sort_by(|a, b| a.1.cmp(&b.1));
    for (_, count) in sorted_entries {
        // Check if we have a new top amount of matches
        if count > max_matches {

            next_max_matches = max_matches.clone();
            max_matches = count;
            
        }

        if count >= 2 {
            pairs += 1;
        }
    }

    let has_joker = has_joker(cards);

    if has_joker{
        // Count how many joker cards we have 
        let joker_count:u32 = cards.find("J").unwrap().try_into().unwrap();

        // In case of Joker creates a five of a kind:
        // 1. 4 matches and one joker 
        // 2. All jokers 
        // 3. All but one is a joker
        if (max_matches == 4 && joker_count == 1) || joker_count == 5 || joker_count == 4{
            return Ok(Hand::FiveOfKind);  
        }

        // In case of four of a kind 
        let has_one_joker = joker_count == 1 && next_max_matches == 1;
        let has_two_jokers = joker_count == 2 && next_max_matches == 2;
        if max_matches == 3 && ( has_one_joker || has_two_jokers ) {
            return Ok(Hand::FourOfKind);
        }

        // In case of three of a kind
        if max_matches == 2 && joker_count != 2{
            return Ok(Hand::ThreeOfKind);
        }

        // In case the joker can create two pairs
        if pairs == 1 && joker_count != 2{
            return Ok(Hand::TwoPair);
        }

        // In case the joker can create a single pair 
        if pairs == 0 && joker_count >= 1{
            return Ok(Hand::OnePair);
        }

        return Ok(Hand::HighCard);

        // Should not become a high card

    }else{

        // Classify the normal way 

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
            match pairs {
                1 => return Ok(Hand::OnePair),
                2 => return Ok(Hand::TwoPair),
                _ => return Ok(Hand::HighCard),
            };
        }

    }

}

fn has_joker(cards: &str) -> bool {
    match cards.find("J"){
        Some(_) => true,
        None => false,
    }
}