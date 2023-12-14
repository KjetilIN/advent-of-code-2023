mod hand;
mod card;
mod classify_card;
mod score_card;
mod card_list;
mod tests;

use std::{path::Path, fs::File, io::{BufReader, Read}, process::exit};

use crate::{card::*, card_list::*};

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

    let mut cards_part_one: Vec<Card> = Vec::new();
    let mut cards_part_two: Vec<Card> = Vec::new();

    // For each line we need to fine the number
    for line in content.lines(){
        let card = match Card::new(line,false){
            Some(card) => card,
            None => exit(1)
        };

        let card_wild = match Card::new(line, true){
            Some(card) => card,
            None => exit(1)
        };

        // Add the card
        cards_part_one.push(card);
        cards_part_two.push(card_wild);
    }

    let mut card_list_part_one = CardList::new(cards_part_one);
    let mut card_list_part_two = CardList::new(cards_part_two);
    let score_part_one = card_list_part_one.get_bid_score();
    let score_part_two = card_list_part_two.get_bid_score_with_wildcard();

    //println!("{:#?}", card_list_part_two);

    // Solution 253205868
    println!("Card bid score (part 1) = {}", score_part_one);

    // Solution NOT => 253731519
    println!("Card bid score (part 2) = {}", score_part_two);

    Ok(())

}
